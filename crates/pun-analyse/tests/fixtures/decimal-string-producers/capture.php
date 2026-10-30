<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-decimal-producers-' . getmypid();
mkdir($root);
try {
    $out = [];
    $native = file_get_contents(__DIR__ . '/native.php');
    $native = preg_replace('/^(    )(.+); \/\/ native$/m', '$1\\PHPStan\\dumpNativeType($2);', $native);
    file_put_contents("$root/native.php", $native);
    foreach (['producers' => __DIR__ . '/producers.php', 'native' => "$root/native.php"] as $case => $file) {
        foreach ([0, 4, 9] as $level) {
            file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$root/$case-$level", ["phar://$phar/conf/config.level$level.neon", "$root/config.neon"], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
                ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
            if ($result->getInternalErrors() !== []) {
                throw new RuntimeException('PHPStan reported an internal error.');
            }
            $records = array_map(static fn($error) => [
                basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
                $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            sort($records);
            $out["$case-$level"] = $records;
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
