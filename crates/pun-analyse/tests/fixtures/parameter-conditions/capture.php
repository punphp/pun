<?php
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-parameter-conditions-' . getmypid();
mkdir($temporary);
$out = [];
try {
    foreach (['entry', 'captures', 'subtraction', 'native', 'bug-12875'] as $case) {
        $file = $case === 'bug-12875'
            ? realpath(__DIR__ . '/../rules/data/Methods/data/bug-12875.php')
            : __DIR__ . "/$case.php";
        foreach ([0, 9] as $level) {
            file_put_contents("$temporary/config.neon", "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$temporary/$case-$level", ['phar://' . $phar . "/conf/config.level$level.neon", "$temporary/config.neon"], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            if ($result->getInternalErrors() !== []) throw new RuntimeException('Oracle internal error');
            $errors = array_map(static fn($error) => [
                $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            sort($errors);
            $out["$case-$level"] = $errors;
            if ($case !== 'native' || $level !== 0) continue;
            $source = file_get_contents($file);
            $types = [];
            $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
                static function ($node, $scope) use (&$types, $source): void {
                    if (!$node instanceof PhpParser\Node\Expr\Variable) return;
                    $end = $node->getEndFilePos() + 1;
                    $tail = substr($source, $end, 16);
                    if (!str_starts_with($tail, '; // observe') && !str_starts_with($tail, ' /* observe */')) return;
                    $types[$node->getStartFilePos()] = [
                        'end' => $end,
                        'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                        'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                    ];
                },
            );
            ksort($types, SORT_NUMERIC);
            $out['native-types'] = $types;
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
