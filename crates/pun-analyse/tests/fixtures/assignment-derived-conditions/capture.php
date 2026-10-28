<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-assignment-derived-' . getmypid();
mkdir($root);
try {
    foreach (['arrays', 'guards', 'calls', 'limit', 'named', 'effects', 'retained', 'strings'] as $case) {
        $files = [__DIR__ . "/$case.php"];
        foreach ([0, 4, 5] as $level) {
            file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$root/$case-$level",
                ["phar://$phar/conf/config.level$level.neon", "$root/config.neon"],
                $files,
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles($files);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
            $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
                ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
            if ($result->getInternalErrors() !== []) {
                throw new RuntimeException('PHPStan reported an internal error.');
            }
            $errors = array_map(static fn($error) => [
                basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
                $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            sort($errors);
            $out["$case-$level"] = $errors;
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
