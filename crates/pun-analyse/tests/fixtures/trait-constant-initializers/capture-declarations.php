<?php

// php capture-declarations.php /absolute/path/to/phpstan-2.3.1.phar declarations|original|overriding-selection > expected-NAME.json
// Capture every public Analyser diagnostic in the existing normalized PHPStan JSON format.
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$name = $argv[2];
$fixtures = dirname(__DIR__);
$file = match ($name) {
    'declarations' => __DIR__ . '/declarations.php',
    'original' => $fixtures . '/rules/data/Classes/data/bug-14250.php',
    'overriding-selection' => __DIR__ . '/overriding-selection.php',
};
$tmp = sys_get_temp_dir() . '/pun-constant-declarations-' . getmypid();
mkdir($tmp);
try {
    file_put_contents("$tmp/config.neon", "parameters:\n    phpVersion: 80400\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$tmp/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$tmp/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    if ($result->getInternalErrors() !== []) {
        throw new RuntimeException('PHPStan reported an internal error.');
    }
    $files = [];
    foreach ($result->getErrors() as $error) {
        $path = $error->getFile();
        if (!str_starts_with($path, "$fixtures/")) {
            throw new RuntimeException("Unexpected diagnostic file: $path");
        }
        $path = substr($path, strlen($fixtures) + 1);
        $files[$path]['messages'][] = [
            'message' => str_replace($fixtures, '<fixtures>', $error->getMessage()),
            'line' => $error->getLine(), 'ignorable' => $error->canBeIgnored(),
            'identifier' => $error->getIdentifier(), 'tip' => $error->getTip(),
        ];
        $files[$path]['errors'] = count($files[$path]['messages']);
    }
    echo json_encode([
        'totals' => ['errors' => 0, 'file_errors' => count($result->getErrors())],
        'files' => $files, 'errors' => [],
    ], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
