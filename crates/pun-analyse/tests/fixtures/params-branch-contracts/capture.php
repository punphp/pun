<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar [output.json]
declare(strict_types=1);

$phar = realpath($argv[1]);
require "phar://$phar/vendor/autoload.php";
$tmp = sys_get_temp_dir() . '/pun-params-branch-contracts-' . getmypid();
mkdir($tmp);
$out = ['version' => '2.3.1', 'phar_sha256' => hash_file('sha256', $phar), 'sources' => [], 'errors' => []];
try {
    foreach (['named', 'constants', 'variants'] as $name) {
        $file = __DIR__ . "/$name.php";
        $out['sources']["$name.php"] = hash_file('sha256', $file);
        foreach ([0, 4, 5] as $level) {
            $mode = "$name/$level";
            $config = "$tmp/config.neon";
            file_put_contents($config, "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$tmp/$name-$level", ["phar://$phar/conf/config.level$level.neon", $config], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
                ->finalize($result, true, true)->getAnalyserResult();
            if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error');
            $rows = array_map(static fn ($error) => [basename($error->getFile()), $error->getLine(),
                $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored()],
                $result->getUnorderedErrors());
            usort($rows, static fn ($a, $b) => json_encode($a) <=> json_encode($b));
            $out['errors'][$mode] = $rows;
        }
    }
    file_put_contents($argv[2] ?? __DIR__ . '/expected.json',
        json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR) . "\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
