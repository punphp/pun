<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-block-pollution-oracle-' . getmypid();
mkdir($root);
// Keep the raw native fixture call-free for Mago's partial native API. PHPStan's dump calls
// replace only marked reads, preserving the source lines used by the public Analyzer test.
$native = preg_replace('/^(\s*)(\$\w+); \/\/ native$/m', '$1\\PHPStan\\dumpNativeType($2);', file_get_contents(__DIR__ . '/native.php'));
file_put_contents("$root/native.php", $native);
$files = [realpath(__DIR__ . '/matrix.php'), realpath("$root/native.php")];
try {
    foreach (['default', 'enabled', 'disabled', 'quiet', 'independent', 'loops-only', 'secondary', 'secondary-enabled'] as $mode) {
        $secondary = str_starts_with($mode, 'secondary');
        $level = $secondary ? 4 : 2;
        $analysed = $secondary ? [realpath(__DIR__ . '/secondary.php')] : $files;
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/$mode", ['phar://' . $phar . "/conf/config.level$level.neon", __DIR__ . "/$mode.neon"], $analysed,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($analysed);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($analysed, debug: true);
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
