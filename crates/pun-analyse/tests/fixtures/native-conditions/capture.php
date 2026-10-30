<?php
// PHPStan 2.3.1; php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-native-conditions-' . getmypid();
mkdir($temporary);
try {
    $file = __DIR__ . '/conditions.php';
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $temporary, ['phar://' . $phar . '/conf/config.level9.neon'], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    if ($result->getInternalErrors() !== []) throw new RuntimeException('Oracle internal error');
    $records = array_map(static fn ($error) => [
        $error->getLine(), $error->getIdentifier(), $error->getMessage(),
        $error->getTip(), $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    sort($records);
    echo json_encode($records, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
