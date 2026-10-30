<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
if (hash_file('sha256', $phar) !== '85157dba3d95d77ca603d2b7303aa78a538f9e3b56ccad07dd794c53aa96d1ae') {
    throw new RuntimeException('Use the pinned PHPStan 2.3.1 PHAR.');
}
require 'phar://' . $phar . '/vendor/autoload.php';
$tmp = sys_get_temp_dir() . '/pun-sentinel-edges-' . getmypid();
mkdir($tmp);
$file = isset($argv[2]) ? realpath($argv[2]) : __DIR__ . '/values.php';
$out = ['phar_sha256' => hash_file('sha256', $phar), 'source_sha256' => hash_file('sha256', $file), 'errors' => []];
try {
    file_put_contents("$tmp/config.neon", "parameters:\n    phpVersion: 80500\n");
    foreach ([0, 4, 5] as $level) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$tmp/cache-$level", ["phar://$phar/conf/config.level$level.neon", "$tmp/config.neon"], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, true, true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error');
        $errors = array_map(static fn($error) => [
            $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        usort($errors, static fn($a, $b) => json_encode($a) <=> json_encode($b));
        $out['errors'][$level] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
