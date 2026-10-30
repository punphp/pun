<?php
// PHPStan 2.3.1: php capture.php /absolute/path/to/phpstan.phar > expected.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-certainty-tips-' . getmypid();
mkdir($temporary);
$out = ['phar_sha256' => hash_file('sha256', $phar), 'sources' => [], 'cases' => []];
try {
    $files = [
        'shapes' => __DIR__ . '/shapes.php',
        'certainty' => __DIR__ . '/../../native-calls/certainty.php',
        'controls' => __DIR__ . '/controls.php',
    ];
    foreach ($files as $case => $file) {
        $file = realpath($file);
        $out['sources'][$case] = hash_file('sha256', $file);
        foreach ([0, 4, 5] as $level) {
            $config = "$temporary/$case-$level.neon";
            file_put_contents($config, "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$temporary/$case-$level", ["phar://$phar/conf/config.level$level.neon", $config], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
                ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
            if ($result->getInternalErrors() !== []) throw new RuntimeException('Oracle internal error');
            $errors = array_map(static fn($error) => [
                basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
                $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            sort($errors);
            $out['cases']["$case-$level"] = $errors;
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
