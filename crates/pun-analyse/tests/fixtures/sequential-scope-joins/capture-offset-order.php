<?php

// php capture-offset-order.php /absolute/path/to/phpstan-2.3.1.phar
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$file = __DIR__ . '/constraints.php';
$temporary = sys_get_temp_dir() . '/pun-offset-order-' . getmypid();
mkdir($temporary);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $temporary, ['phar://' . $phar . '/conf/config.level4.neon', __DIR__ . '/default.neon'], [$file],
        usedLevel: '4',
    );
    $parser = $container->getService('pathRoutingParser');
    $parser->setAnalysedFiles([$file]);
    $resolver = $container->getByType(PHPStan\Analyser\NodeScopeResolver::class);
    $resolver->setAnalysedFiles([$file]);
    $scope = $container->getByType(PHPStan\Analyser\ScopeFactory::class)->create(PHPStan\Analyser\ScopeContext::create($file));
    $records = [];
    $resolver->processNodes($parser->parseFile($file), $scope, static function ($node, $scope) use (&$records) {
        if (!$node instanceof PhpParser\Node\Expr\ArrayDimFetch) {
            return;
        }
        $type = $scope->getType($node);
        $parts = $type instanceof PHPStan\Type\UnionType ? $type->getTypes() : [$type];
        $records[] = [$node->getStartLine(), array_map(
            static fn($part) => $part->describe(PHPStan\Type\VerbosityLevel::precise()), $parts,
        )];
    });
    echo json_encode($records, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($temporary);
}
