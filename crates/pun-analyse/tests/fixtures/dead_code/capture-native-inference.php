<?php
// PHPStan 2.3.1: php capture-native-inference.php /absolute/path/to/phpstan.phar
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-native-inference-' . getmypid();
mkdir($temporary);
$file = __DIR__ . '/native-inference.php';
try {
    $config = "$temporary/phpstan.neon";
    file_put_contents($config, "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$temporary/cache", ["phar://$phar/conf/config.level4.neon", $config], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
        ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
    if ($result->getInternalErrors() !== []) throw new RuntimeException('Oracle internal error');
    $errors = array_map(static fn($error) => [
        $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    sort($errors);
    echo json_encode([
        'phar_sha256' => hash_file('sha256', $phar),
        'source_sha256' => hash_file('sha256', $file),
        'errors' => $errors,
    ], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
