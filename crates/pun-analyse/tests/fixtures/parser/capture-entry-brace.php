<?php

// php capture-entry-brace.php /absolute/path/to/phpstan.phar > entry-brace-expected.json
// PHPStan 2.3.1's public Analyser, without diagnostic filters. Complete sources preserve the token prefix.
// Unsupported captures record upstream diagnostics separately; Pun's grammar boundary deliberately leaves
// those cases unchanged. The configured-ignore result exposes collection before public error sorting.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-entry-brace-oracle-' . getmypid();
mkdir($root);
function analyse(string $root, string $phar, array $files, array $extra = []): array {
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', ...$extra], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $processed = $container->getByType(PHPStan\Analyser\Ignore\IgnoredErrorHelper::class)->initialize()->process(
        $result->getUnorderedErrors(), false, $files, false,
    );
    $errors = [];
    foreach ($processed->getNotIgnoredErrors() as $error) {
        $errors[] = [
            'file' => basename($error->getFile()), 'line' => $error->getLine(),
            'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
            'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
        ];
    }
    usort($errors, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
    return ['errors' => $errors, 'general' => $processed->getOtherIgnoreMessages()];
}
try {
    $out = [];
    foreach (['analysed' => 'entry-brace-inputs.json', 'unsupported' => 'entry-brace-unsupported.json'] as $mode => $input) {
        foreach (json_decode(file_get_contents(__DIR__ . '/' . $input), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
            mkdir($dir = "$root/$mode-$name");
            file_put_contents($file = "$dir/input.php", $source);
            $out[$mode][$name] = analyse($dir, $phar, [$file]);
        }
    }
    foreach (json_decode(file_get_contents(__DIR__ . '/entry-brace-scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        mkdir($dir = "$root/scan-$name");
        file_put_contents("$dir/symbols.php", $source);
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        file_put_contents($config = "$dir/config.neon", "parameters:\n    scanFiles: [symbols.php]\n");
        $out['scanned'][$name] = analyse($dir, $phar, ["$dir/main.php"], [$config]);
    }
    mkdir($dir = "$root/order");
    copy(__DIR__ . '/entry-brace-order.php', $file = "$dir/order.php");
    file_put_contents($config = "$dir/config.neon", "parameters:\n    reportIgnoresWithoutComments: true\n    ignoreErrors:\n        - identifier: phpstan.parse\n        - '#^NoSuchError$#'\n");
    $out['order'] = analyse($dir, $phar, [$file], [$config]);
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
