<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
// Public Analyser captures, with no diagnostic projection or suppression.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-assignment-targets-oracle-' . getmypid();
mkdir($root);
function analyse(string $root, string $phar, array $files, int $level): array {
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . "/conf/config.level$level.neon", "$root/config.neon"], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $out = [];
    foreach ($result->getUnorderedErrors() as $error) {
        $out[] = [
            'file' => basename($error->getFile()), 'line' => $error->getLine(),
            'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
            'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
        ];
    }
    usort($out, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
    return $out;
}
try {
    $out = [];
    foreach (['inputs', 'nullsafe'] as $group) {
        $dir = "$root/$group";
        mkdir($dir);
        $files = [];
        foreach (json_decode(file_get_contents(__DIR__ . "/$group.json"), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
            $files[] = $file = "$dir/$name.php";
            file_put_contents($file, $source);
        }
        $out[$group] = analyse($dir, $phar, $files, 0);
    }
    $dir = "$root/levels";
    mkdir($dir);
    $files = [];
    $sources = json_decode(file_get_contents(__DIR__ . '/levels.json'), true, flags: JSON_THROW_ON_ERROR);
    foreach ($sources as $name => $source) {
        $files[] = $file = "$dir/$name.php";
        file_put_contents($file, $source);
    }
    foreach ([0, 1, 2, 7, 8, 9, 10] as $level) {
        $out['levels'][$level] = analyse($dir, $phar, $files, $level);
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
