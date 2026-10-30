<?php

// Independently recapture the existing native-lane boundary records after the
// array extension. The temporary source retains the original line numbers.
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$source = file_get_contents(__DIR__ . '/../native-lane/condition-boundaries.php');
$source = preg_replace('/^(\s*)(.+); \/\/ native$/m', '$1\\PHPStan\\dumpNativeType($2);', $source);
$tmp = sys_get_temp_dir() . '/pun-native-isset-lane-' . getmypid();
mkdir($tmp);
try {
    $file = "$tmp/boundaries.php";
    file_put_contents($file, $source);
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$tmp/cache", ["phar://$phar/conf/config.level0.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
        ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
    if ($result->getInternalErrors() !== []) { throw new RuntimeException('Oracle internal error'); }
    $errors = [];
    foreach ($result->getUnorderedErrors() as $error) {
        if ($error->getIdentifier() !== 'phpstan.dumpNativeType') {
            throw new RuntimeException($error->getMessage());
        }
        $errors[] = ['message' => $error->getMessage(), 'line' => $error->getLine(),
            'ignorable' => $error->canBeIgnored(), 'identifier' => $error->getIdentifier()];
    }
    usort($errors, static fn($a, $b) => $a['line'] <=> $b['line']);
    echo json_encode($errors, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
