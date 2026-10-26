<?php

// php capture-mixed-document-closing.php /absolute/path/to/phpstan-2.3.1.phar > mixed-document-closing-expected.json
// Public level-0 Analyser. parseDocString checks the closing marker after its body expressions reduce.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-mixed-document-closing-oracle-' . getmypid();
mkdir($root);
function rows(array $errors): array {
    $out = [];
    foreach ($errors as $error) {
        $out[] = [
            'file' => basename($error->getFile()), 'line' => $error->getLine(),
            'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
            'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
        ];
    }
    usort($out, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
    return $out;
}
function analyse(string $root, string $phar, array $files, string $config): array {
    file_put_contents("$root/config.neon", "parameters:\n$config");
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$root/config.neon"], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $processed = $container->getByType(PHPStan\Analyser\Ignore\IgnoredErrorHelper::class)->initialize()->process(
        $result->getUnorderedErrors(), false, $files, false,
    );
    return ['errors' => rows($processed->getNotIgnoredErrors()), 'general' => $processed->getOtherIgnoreMessages()];
}
try {
    $files = [];
    foreach (json_decode(file_get_contents(__DIR__ . '/mixed-document-closing-inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        $files[] = $file = "$root/$name.php";
        file_put_contents($file, $source);
    }
    $out = ['analysed' => analyse($root, $phar, $files, "    phpVersion: 80500\n")];
    foreach (json_decode(file_get_contents(__DIR__ . '/mixed-document-closing-scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        $dir = "$root/scan-$name";
        mkdir($dir);
        file_put_contents("$dir/symbols.php", $source);
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        $out['scanned'][$name] = analyse($dir, $phar, ["$dir/main.php"], "    phpVersion: 80500\n    scanFiles: [symbols.php]\n");
    }
    foreach (json_decode(file_get_contents(__DIR__ . '/mixed-document-closing-order.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        $dir = "$root/order-$name";
        mkdir($dir);
        file_put_contents("$dir/order.php", $source);
        $out['order'][$name] = analyse($dir, $phar, ["$dir/order.php"], "    phpVersion: 80500\n    ignoreErrors:\n        - identifier: phpstan.parse\n");
    }
    foreach ([70100, 70300, 70400, 80000, 80100, 80200, 80400, 80500] as $version) {
        $dir = "$root/version-$version";
        mkdir($dir);
        copy(__DIR__ . '/mixed-document-closing-version.php', "$dir/input.php");
        $out['versions'][$version] = analyse($dir, $phar, ["$dir/input.php"], "    phpVersion: $version\n");
    }
    foreach (json_decode(file_get_contents(__DIR__ . '/mixed-document-closing-boundaries.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        $dir = "$root/boundary-$name";
        mkdir($dir);
        file_put_contents("$dir/input.php", $source);
        $out['boundaries'][$name] = analyse($dir, $phar, ["$dir/input.php"], "    phpVersion: 80500\n");
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
