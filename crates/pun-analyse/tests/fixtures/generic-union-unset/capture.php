<?php

// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://'.$phar.'/vendor/autoload.php';
$temporary = sys_get_temp_dir().'/pun-generic-union-unset-'.getmypid();
mkdir($temporary);
try {
    $out = [];
    file_put_contents($temporary.'/config.neon', "parameters:\n    phpVersion: 80500\n");
    foreach (['virtual' => [0, 4], 'values' => [0], 'scopes' => [0], 'objects' => [0], 'members' => [0]] as $name => $levels) {
        $file = __DIR__.'/'.$name.'.php';
        foreach ($levels as $level) {
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                $temporary.'/'.$name.'-'.$level,
                ['phar://'.$phar.'/conf/config.level'.$level.'.neon', $temporary.'/config.neon'], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
                ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
            if ($result->getInternalErrors() !== []) {
                throw new RuntimeException('PHPStan internal error');
            }
            $records = array_map(static fn($error) => [
                basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
                $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            sort($records);
            $out[$name][$level] = $records;
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($temporary);
}
