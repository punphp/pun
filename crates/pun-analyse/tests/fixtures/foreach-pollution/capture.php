<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-foreach-pollution-oracle-' . getmypid();
mkdir($root);
$files = array_map('realpath', [
    __DIR__ . '/matrix.php',
    __DIR__ . '/../rules/data/Variables/data/foreach-always-iterable.php',
]);
try {
    foreach (['default', 'enabled', 'disabled', 'quiet', 'independent'] as $mode) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/$mode", ['phar://' . $phar . '/conf/config.level0.neon', __DIR__ . "/$mode.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $errors = array_map(static fn($error) => [
            basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
            $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        sort($errors);
        $out[$mode] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
