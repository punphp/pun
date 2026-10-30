<?php
// php capture.php /path/to/pinned/phpstan.phar /path/to/larastan-checkout /output.json [Calls.php]
declare(strict_types=1);

$phar = realpath($argv[1]);
$larastan = realpath($argv[2]);
require "phar://$phar/vendor/autoload.php";
require "$larastan/vendor/autoload.php";
$tmp = sys_get_temp_dir().'/pun-config-shape-oracle-'.getmypid();
mkdir($tmp);
try {
    $app = new Illuminate\Foundation\Application($tmp);
    $app->instance('config', new Illuminate\Config\Repository());
    $app->instance('path.config', __DIR__.'/config');
    Illuminate\Support\Facades\Facade::setFacadeApplication($app);
    Illuminate\Container\Container::setInstance($app);
    if (!defined('LARAVEL_VERSION')) define('LARAVEL_VERSION', $app->version());
    $config = "$tmp/phpstan.neon";
    file_put_contents($config, "parameters:\n    phpVersion: 80500\n    checkConfigTypes: true\n    configDirectories:\n        - '".__DIR__."/config'\n");
    $file = __DIR__.'/'.($argv[4] ?? 'Calls.php');
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$tmp/cache", ["phar://$phar/conf/config.level0.neon", "$larastan/extension.neon", $config], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
        ->finalize($result, true, true)->getAnalyserResult();
    if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error');
    $rows = array_map(static fn ($e) => [basename($e->getFile()), $e->getLine(),
        $e->getIdentifier(), $e->getMessage(), $e->getTip(), $e->canBeIgnored()], $result->getUnorderedErrors());
    usort($rows, static fn ($a, $b) => json_encode($a) <=> json_encode($b));
    file_put_contents($argv[3], json_encode(['phar_sha256' => hash_file('sha256', $phar),
        'type_resolver' => str_replace($phar, 'phpstan.phar',
            (new ReflectionClass(PHPStan\PhpDoc\TypeNodeResolver::class))->getFileName()),
        'errors' => $rows], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
