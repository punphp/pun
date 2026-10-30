<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
// php capture.php /absolute/path/to/phpstan-2.3.1.phar selectors.php > expected-selectors.json
// php capture.php /absolute/path/to/phpstan-2.3.1.phar compatibility.php > expected-compatibility.json
// php capture.php /absolute/path/to/phpstan-2.3.1.phar compatibility-controls.php > expected-compatibility-controls.json
// php capture.php /absolute/path/to/phpstan-2.3.1.phar relation-controls.php > expected-relation-controls.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$tmp = sys_get_temp_dir() . '/pun-failed-method-callables-' . getmypid();
mkdir($tmp);
$file = __DIR__ . '/' . ($argv[2] ?? 'cases.php');
$source = file_get_contents($file);
$describe = static function (PHPStan\Type\Type $type): array {
    if ($type instanceof PHPStan\Type\ClosureType) {
        return ['kind' => 'signature', 'parameters' => array_map(
            static fn($parameter) => $parameter->getType()->describe(PHPStan\Type\VerbosityLevel::precise()),
            $type->getParameters(),
        ), 'return' => $type->getReturnType()->describe(PHPStan\Type\VerbosityLevel::precise())];
    }
    if ($type instanceof PHPStan\Type\ObjectType && $type->getClassName() === 'Closure') {
        return ['kind' => 'bare-closure'];
    }
    return ['kind' => 'value', 'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise())];
};
$out = ['phar_sha256' => hash_file('sha256', $phar), 'types' => [], 'errors' => []];
try {
    foreach ([false, true] as $bleeding) {
        foreach ([0, 5, 9] as $level) {
            $key = $level . ($bleeding ? '-bleeding' : '');
            $config = "$tmp/config-$key.neon";
            file_put_contents($config, "parameters:\n    phpVersion: 80500\n    checkThisOnly: false\n");
            $includes = ['phar://' . $phar . "/conf/config.level$level.neon", $config];
            if ($bleeding) $includes[] = 'phar://' . $phar . '/conf/bleedingEdge.neon';
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create("$tmp/cache-$key", $includes, [$file]);
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            if ($level === 0 && !$bleeding) {
                $records = [];
                $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                    $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
                    static function ($node, $scope) use (&$records, $source, $describe): void {
                        if (!$node instanceof PhpParser\Node\Expr\Assign) return;
                        $expression = $node->expr;
                        $start = $expression->getStartFilePos();
                        $end = $expression->getEndFilePos() + 1;
                        $records[$start] = [
                            'start' => $start, 'end' => $end, 'expression' => substr($source, $start, $end - $start),
                            'full' => $describe($scope->getType($expression)),
                            'native' => $describe($scope->getNativeType($expression)),
                        ];
                    },
                );
                ksort($records, SORT_NUMERIC);
                $out['types'] = array_values($records);
            }
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)->finalize($result, true, true)->getAnalyserResult();
            if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error');
            $records = array_map(static fn($error) => [
                basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
                $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            usort($records, static fn($a, $b) => json_encode($a) <=> json_encode($b));
            $out['errors'][$key] = $records;
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
