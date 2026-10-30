<?php

// php capture-namespace-post.php /absolute/path/to/phpstan.phar > namespace-post-expected.json
// PHPStan 2.3.1's public Analyser; complete sources retain initial HTML and shebangs. Configured ignores expose
// root-reduction ordering after body semantic checks, and mixed styles suppress the outside-code pass.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-namespace-post-oracle-' . getmypid();
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
function analyse(string $root, string $phar, array $files, array $extra = []): array {
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', ...$extra], $files,
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
    foreach (json_decode(file_get_contents(__DIR__ . '/namespace-post-inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $files[] = $file = "$root/$name.php";
        file_put_contents($file, $code);
    }
    $out = ['analysed' => analyse($root, $phar, $files)];
    foreach (json_decode(file_get_contents(__DIR__ . '/namespace-post-scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/scan-$name";
        mkdir($dir);
        file_put_contents("$dir/symbols.php", $code);
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        file_put_contents("$dir/config.neon", "parameters:\n    scanFiles: [symbols.php]\n");
        $out['scanned'][$name] = analyse($dir, $phar, ["$dir/main.php"], ["$dir/config.neon"]);
    }
    foreach (json_decode(file_get_contents(__DIR__ . '/namespace-post-order.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/order-$name";
        mkdir($dir);
        file_put_contents("$dir/order.php", $code);
        file_put_contents("$dir/config.neon", "parameters:\n    ignoreErrors:\n        - identifier: phpstan.parse\n");
        $out['order'][$name] = analyse($dir, $phar, ["$dir/order.php"], ["$dir/config.neon"]);
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
