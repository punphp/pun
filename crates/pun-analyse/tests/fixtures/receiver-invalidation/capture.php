<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar [patched] > expected.json
// phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-receiver-invalidation-' . getmypid();
mkdir($temporary);
try {
    $patched = ($argv[2] ?? '') === 'patched';
    $files = [__DIR__ . ($patched ? '/patched.php' : '/receivers.php')];
    $configs = ['phar://' . $phar . '/conf/config.level0.neon'];
    if ($patched) {
        $configs[] = __DIR__ . '/patched.neon';
    }
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $temporary, $configs, $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    if ($result->getInternalErrors() !== []) {
        throw new RuntimeException('The PHPStan oracle reported an internal error.');
    }
    $errors = array_map(static fn($error) => [
        basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
        $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    sort($errors);
    echo json_encode($errors, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($temporary);
}
