<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-assignment-target-reads-' . getmypid();
mkdir($root);
try {
    $files = [];
    foreach (json_decode(file_get_contents(__DIR__ . '/inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        $files[] = $file = "$root/$name.php";
        file_put_contents($file, $source);
    }
    foreach (json_decode(file_get_contents(__DIR__ . '/boundaries.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $case) {
        $files[] = $file = "$root/$name.php";
        file_put_contents($file, $case['source']);
    }
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
    $out = [];
    foreach ([0, 1, 2, 7, 8, 9, 10] as $level) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
            "$root/cache", ['phar://' . $phar . "/conf/config.level$level.neon", "$root/config.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $errors = [];
        foreach ($result->getUnorderedErrors() as $error) {
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
