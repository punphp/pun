<?php

// php capture-scans.php /absolute/path/to/phpstan.phar > expected-scans.json
// Real level-0 registry, one independent container per scan-only source. No diagnostics are filtered.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-scans-oracle-' . getmypid();
mkdir($root);
$out = [];
try {
    foreach (json_decode(file_get_contents(__DIR__ . '/scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/$name";
        mkdir($dir);
        file_put_contents("$dir/symbols.php", "<?php\n$code\n");
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        file_put_contents("$dir/config.neon", "parameters:\n    scanFiles: [symbols.php]\n");
        $files = ["$dir/main.php"];
        $container = (new PHPStan\DependencyInjection\ContainerFactory($dir))->create(
            "$dir/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$dir/config.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $errors = [];
        foreach ($container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true)->getErrors() as $error) {
            $errors[] = [
                'file' => basename($error->getFile()), 'line' => $error->getLine(),
                'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
                'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
            ];
        }
        usort($errors, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
        $out[$name] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
