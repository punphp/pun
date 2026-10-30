<?php

// php capture.php /absolute/path/to/phpstan.phar [output.json]
// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);

$phar = realpath($argv[1]);
$root = __DIR__;
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-in-array-narrowing-' . getmypid();
mkdir($temporary);
$expected = [];
try {
    foreach ([0, 4] as $level) {
        $file = "$root/decisions.php";
        $config = "$temporary/level$level.neon";
        file_put_contents($config, "parameters:\n    phpVersion: 80500\n");
        $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
            "$temporary/level$level", ["phar://$phar/conf/config.level$level.neon", $config], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) {
            throw new RuntimeException('Oracle internal error');
        }
        $expected[$level] = [];
        foreach ($result->getUnorderedErrors() as $error) {
            $expected[$level][] = [
                'file' => str_replace($root, '$ROOT', $error->getFile()),
                'line' => $error->getLine(),
                'identifier' => $error->getIdentifier(),
                'message' => $error->getMessage(),
                'tip' => $error->getTip(),
                'ignorable' => $error->canBeIgnored(),
            ];
        }
        echo "level $level: ", count($expected[$level]), " complete records\n";
    }
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($temporary);
}
file_put_contents($argv[2] ?? "$root/expected-phpstan.json",
    json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
