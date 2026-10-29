<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
declare(strict_types=1);

$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$tmp = sys_get_temp_dir() . '/pun-failed-instantiation-' . getmypid();
mkdir($tmp);
$files = [__DIR__ . '/illegal.php', __DIR__ . '/controls.php'];
$out = ['phar_sha256' => hash_file('sha256', $phar), 'types' => [], 'continued' => [], 'errors' => []];
function describeInstantiation(PHPStan\Type\Type $type): array {
    return [
        'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
        'error' => $type instanceof PHPStan\Type\ErrorType,
        'kind' => $type instanceof PHPStan\Type\NonAcceptingNeverType ? 'nonaccepting'
            : ($type instanceof PHPStan\Type\NeverType ? ($type->isExplicit() ? 'explicit' : 'implicit') : null),
    ];
}
try {
    foreach ([0, 5, 9] as $level) {
        file_put_contents("$tmp/config.neon", "parameters:\n    phpVersion: 80500\n    checkThisOnly: false\n");
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$tmp/cache-$level", ['phar://' . $phar . "/conf/config.level$level.neon", "$tmp/config.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
        if ($level === 0) foreach ($files as $file) {
            $source = file_get_contents($file);
            $types = [];
            $continued = [];
            $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                $file, array_fill_keys($files, true), new PHPStan\Rules\DirectRegistry([]),
                new PHPStan\Collectors\Registry([]),
                static function ($node, $scope) use (&$types, &$continued, $source): void {
                    if ($node instanceof PhpParser\Node\Scalar\String_ && str_starts_with($node->value, 'after-')) {
                        $continued[$node->value] = $node->getStartFilePos();
                    }
                    if (!$node instanceof PhpParser\Node\Expr\New_
                        && !$node instanceof PhpParser\Node\Expr\Assign
                        && !($node instanceof PhpParser\Node\Expr\Variable
                            && str_ends_with(substr($source, 0, $node->getStartFilePos()), '/*probe*/ '))) return;
                    $type = $scope->getType($node);
                    $types[] = [
                        'start' => $node->getStartFilePos(), 'end' => $node->getEndFilePos() + 1,
                        'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                        ...describeInstantiation($type),
                        'class' => get_class($type),
                        'final' => $type instanceof PHPStan\Type\ObjectType ? $type->getClassReflection()?->isFinalByKeyword() : null,
                        'native' => describeInstantiation($scope->getNativeType($node)),
                    ];
                },
            );
            usort($types, static fn($a, $b) => [$a['start'], $a['end']] <=> [$b['start'], $b['end']]);
            ksort($continued);
            $out['types'][basename($file)] = $types;
            $out['continued'][basename($file)] = $continued;
        }
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, true, true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error');
        $errors = array_map(static fn($error) => [
            basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
            $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        usort($errors, static fn($a, $b) => json_encode($a) <=> json_encode($b));
        $out['errors'][$level] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
