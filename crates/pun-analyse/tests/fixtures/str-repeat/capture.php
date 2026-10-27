<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-str-repeat-oracle-' . getmypid();
mkdir($root);
$files = glob(__DIR__ . '/*.php');
$files = array_values(array_filter($files, static fn($file) => basename($file) !== 'capture.php'));
try {
    $config = "$root/config.neon";
    file_put_contents($config, "parameters:\n    phpVersion: 80500\n    featureToggles:\n        switchConditionAlwaysFalse: true\n");
    $errors = [];
    $groups = [
        0 => array_values(array_filter($files, static fn($file) => basename($file) !== 'byte-comparisons.php')),
        4 => [__DIR__ . '/byte-comparisons.php'],
    ];
    foreach ($groups as $level => $paths) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/cache-$level", ['phar://' . $phar . "/conf/config.level$level.neon", $config], $paths,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($paths);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($paths, debug: true);
        $errors = array_merge($errors, $result->getUnorderedErrors());
    }
    $errors = array_map(static fn($error) => [
        basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
        $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
    ], $errors);
    sort($errors);
    // The public Rust diagnostic API is UTF-8. Substitute only at this final presentation boundary;
    // the separate byte-string oracle records constant values losslessly as hexadecimal bytes.
    echo json_encode($errors, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_INVALID_UTF8_SUBSTITUTE | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
