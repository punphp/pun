<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-compound-reads-' . getmypid();
mkdir($root);
try {
    // Observe the real AssignOpHandler handoff, not the raw target callback's
    // entry-scope ask or applyWrite's later replacement in expression storage.
    $handler = file_get_contents("phar://$phar/src/Analyser/ExprHandler/AssignOpHandler.php");
    $needle = '$targetReadResult = $target->getTargetReadResult();';
    $instrument = <<<'PHP'
        $GLOBALS['compoundReads'][] = [
            basename($scope->getFile()), $expr->var->getStartFilePos(), $expr->var->getEndFilePos() + 1,
            $targetReadResult->getType()->describe(\PHPStan\Type\VerbosityLevel::precise()),
            $targetReadResult->getType() instanceof \PHPStan\Type\ErrorType,
        ];
PHP;
    $handler = str_replace($needle, "$needle\n$instrument", $handler, $replaced);
    if ($replaced !== 1) throw new RuntimeException('Pinned AssignOpHandler handoff changed');
    file_put_contents("$root/AssignOpHandler.php", $handler);
    require "$root/AssignOpHandler.php";
    $files = [__DIR__ . '/inputs.php', realpath(__DIR__ . '/../rules/data/Arrays/data/offset-access-assignop-nullsafe.php')];
    $diagnostics = [];
    foreach ([3, 5, 9] as $level) {
        $GLOBALS['compoundReads'] = [];
        file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/$level", ["phar://$phar/conf/config.level$level.neon", "$root/config.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, onlyFiles: true, debug: true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error');
        $records = array_map(static fn($error) => [
            basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
            $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        sort($records);
        $diagnostics[$level] = $records;
    }
    $GLOBALS['compoundReads'] = [];
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
    foreach ($files as $file) {
        $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
            $file, array_fill_keys($files, true), new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
            null,
        );
        if ($result->getErrors() !== []) throw new RuntimeException('Observation capture failed');
    }
    $reads = $GLOBALS['compoundReads'];
    sort($reads);
    $reads = array_values(array_unique($reads, SORT_REGULAR));
    echo json_encode(['diagnostics' => $diagnostics, 'reads' => $reads], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
