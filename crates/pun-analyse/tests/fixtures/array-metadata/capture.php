<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar [fixture.php] > callable-restoration.json
declare(strict_types=1);

$phar = realpath($argv[1]);
if (hash_file('sha256', $phar) !== '4b3ed7850a20b092331bd1040e4f54a1d3c0559b4cf006431073997ebcb4324b') {
    throw new RuntimeException('Use the pinned PHPStan 2.3.1 PHAR.');
}
require 'phar://' . $phar . '/vendor/autoload.php';
$file = __DIR__ . '/' . ($argv[2] ?? 'callable-restoration.php');
$tmp = sys_get_temp_dir() . '/pun-array-metadata-' . getmypid();
mkdir($tmp);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$tmp/cache", ["phar://$phar/conf/config.level0.neon", __DIR__ . '/phpstan.neon'], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
        ->finalize($result, true, true)->getAnalyserResult();
    if ($result->getInternalErrors() !== []) {
        throw new RuntimeException('PHPStan reported an internal error.');
    }
    $messages = array_map(static fn($error) => [
        'message' => $error->getMessage(),
        'line' => $error->getLine(),
        'ignorable' => $error->canBeIgnored(),
        'identifier' => $error->getIdentifier(),
        'tip' => $error->getTip(),
    ], $result->getUnorderedErrors());
    usort($messages, static fn($a, $b) => [$a['line'], $a['message']] <=> [$b['line'], $b['message']]);
    echo json_encode([
        'source' => 'PHPStan 2.3.1; phpstan.neon; level 0; ' . basename($file),
        'phar_sha256' => hash_file('sha256', $phar),
        'files' => [basename($file) => ['errors' => count($messages), 'messages' => $messages]],
    ], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
