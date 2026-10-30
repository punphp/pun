<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://'.$phar.'/vendor/autoload.php';
$temporary = sys_get_temp_dir().'/pun-builtin-method-effect-conditions-'.getmypid();
mkdir($temporary);
try {
    $file = __DIR__.'/input.php';
    file_put_contents($temporary.'/config.neon', "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $temporary.'/cache', ['phar://'.$phar.'/conf/config.level4.neon', $temporary.'/config.neon'], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    if ($result->getInternalErrors() !== []) {
        throw new RuntimeException('PHPStan internal error');
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
