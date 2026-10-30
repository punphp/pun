<?php
declare(strict_types=1);

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = realpath($argv[1]);
require 'phar://'.$phar.'/vendor/autoload.php';
$root = sys_get_temp_dir().'/pun-destructuring-assignment-scopes-'.getmypid();
mkdir($root);
try {
    $files = [];
    foreach (json_decode(file_get_contents(__DIR__.'/inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        file_put_contents($file = "$root/$name.php", $source);
        $files[] = $file;
    }
    $out = ['phar_sha256' => hash_file('sha256', $phar), 'diagnostics' => []];
    foreach (['level0' => [0, false], 'default' => [1, true], 'quiet' => [1, false]] as $mode => [$level, $maybe]) {
        file_put_contents($config = "$root/$mode.neon", "parameters:\n    phpVersion: 80500\n    checkMaybeUndefinedVariables: ".($maybe ? 'true' : 'false')."\n");
        $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
            "$root/cache-$mode", ["phar://$phar/conf/config.level$level.neon", $config], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)->finalize($result, true, true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal errors');
        $out['diagnostics'][$mode] = array_map(static fn ($e) => [basename($e->getFile()), $e->getLine(),
            $e->getIdentifier(), $e->getMessage(), $e->getTip(), $e->canBeIgnored()], $result->getUnorderedErrors());
        usort($out['diagnostics'][$mode], static fn ($a, $b) => json_encode($a) <=> json_encode($b));
    }
    $file = "$root/observations.php";
    preg_match_all('/\/\* observe \*\/ (\$\w+)/', file_get_contents($file), $matches, PREG_OFFSET_CAPTURE);
    $marked = array_fill_keys(array_column($matches[1], 1), true);
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/observations-cache", ["phar://$phar/conf/config.level0.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $found = [];
    $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$found, $marked): void {
            if (!$node instanceof PhpParser\Node\Expr\Variable || !isset($marked[$node->getStartFilePos()])) return;
            $found[] = ['start' => $node->getStartFilePos(), 'line' => $node->getStartLine(),
                'selected' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise())];
        },
    );
    if (count($found) !== count($marked)) throw new RuntimeException('missing or repeated observations');
    $out['observations'] = $found;
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
