<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar [/absolute/path/to/input.php] [level]
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-sequential-joins-oracle-' . getmypid();
mkdir($root);
$files = [realpath($argv[2] ?? __DIR__ . '/joins.php')];
$level = (int) ($argv[3] ?? 3);
try {
    foreach (['default', 'quiet', 'foreach-off', 'loop-off'] as $mode) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/$mode", ['phar://' . $phar . "/conf/config.level$level.neon", __DIR__ . "/$mode.neon"], $files,
            usedLevel: (string) $level,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, onlyFiles: false, debug: true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) {
            throw new RuntimeException('The PHPStan oracle reported an internal error.');
        }
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
