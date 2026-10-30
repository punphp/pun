<?php

// php phpstan-capture.php /absolute/path/to/phpstan.phar [/absolute/path/to/input.php]
require 'phar://'.realpath($argv[1]).'/vendor/autoload.php';
require __DIR__.'/bootstrap.php';

use PhpParser\Node;
use PHPStan\Analyser\FileAnalyser;
use PHPStan\Analyser\NodeScopeResolver;
use PHPStan\Analyser\Scope;
use PHPStan\Collectors\Registry as CollectorRegistry;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Rules\DirectRegistry;
use PHPStan\Type\VerbosityLevel;

$temporary = sys_get_temp_dir().'/pun-repeated-assertions-'.getmypid();
mkdir($temporary);
try {
    $config = $temporary.'/config.neon';
    $declarations = dirname(__DIR__, 4).'/pun-lint/tests/phpunit-stubs';
    file_put_contents($config, "parameters:\n    phpVersion: 80400\n    scanDirectories:\n        - $declarations\n");
    $file = $argv[2] ?? __DIR__.'/input.php';
    $container = (new ContainerFactory(__DIR__))->create($temporary.'/cache', [$config], [$file]);
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $result = $container->getByType(FileAnalyser::class)->analyseFile(
        $file,
        [$file => true],
        new DirectRegistry([]),
        new CollectorRegistry([]),
        static function (Node $node, Scope $scope) use ($source): void {
            if (!$node instanceof Node\Stmt\Expression || !str_starts_with(substr($source, $node->getEndFilePos() + 1), ' // observe')) {
                return;
            }
            $expression = $node->expr;
            echo json_encode([
                'line' => $expression->getStartLine(),
                'start' => $expression->getStartFilePos(),
                'end' => $expression->getEndFilePos() + 1,
                'expression' => substr($source, $expression->getStartFilePos(), $expression->getEndFilePos() - $expression->getStartFilePos() + 1),
                'full' => $scope->getType($expression)->describe(VerbosityLevel::precise()),
                'native' => $scope->getNativeType($expression)->describe(VerbosityLevel::precise()),
            ], JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
        },
    );
    if ($result->getErrors() !== []) {
        throw new RuntimeException('oracle run errors');
    }
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($temporary);
}
