<?php

// php capture.php /absolute/phpstan-2.3.1.phar [file.php ...] > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
if (hash_file('sha256', $phar) !== '85157dba3d95d77ca603d2b7303aa78a538f9e3b56ccad07dd794c53aa96d1ae') {
    throw new RuntimeException('Use the pinned PHPStan 2.3.1 PHAR.');
}
require 'phar://' . $phar . '/vendor/autoload.php';
$files = array_slice($argv, 2) ?: [__DIR__ . '/callbacks.php'];
$tmp = sys_get_temp_dir() . '/pun-converter-remainder-' . getmypid();
mkdir($tmp);
$out = ['phar_sha256' => hash_file('sha256', $phar), 'sources' => [], 'records' => []];
try {
    foreach ($files as $index => $file) {
        $file = realpath($file);
        $name = basename($file);
        $out['sources'][$name] = hash_file('sha256', $file);
        foreach ([0, 5] as $level) {
            $config = "$tmp/$index-$level.neon";
            file_put_contents($config, "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$config-cache", ["phar://$phar/conf/config.level$level.neon", $config], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
                ->finalize($result, true, true)->getAnalyserResult();
            if ($result->getInternalErrors() !== []) {
                throw new RuntimeException('PHPStan internal error');
            }
            $records = array_map(static fn ($error) => [basename($error->getFile()), $error->getLine(),
                $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored()],
                $result->getUnorderedErrors());
            usort($records, static fn ($a, $b) => json_encode($a) <=> json_encode($b));
            $out['records']["$name/$level"] = $records;
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
