<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-declaration-purity-' . getmypid();
mkdir($root);
try {
    $files = [];
    foreach (['matrix', 'supported', 'boundaries'] as $name) {
        $files[] = $file = "$root/$name.php";
        copy(__DIR__ . "/$name.php", $file);
    }
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
    $out = [];
    foreach ([1, 2, 4, 10] as $level) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
            "$root/cache", ['phar://' . $phar . "/conf/config.level$level.neon", "$root/config.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)->finalize($result, onlyFiles: true, debug: true);
        $errors = [];
        foreach ($result->getErrors() as $error) {
            $errors[] = [
                'file' => basename($error->getFile()), 'line' => $error->getLine(),
                'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
                'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
            ];
        }
        usort($errors, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
        $out[$level] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
