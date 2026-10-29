<?php

// php capture-prepared.php /absolute/path/to/phpstan-2.3.1.phar [written.php] > records.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-prepared-coalesce-' . getmypid();
mkdir($root);
try {
    $out = [];
    $files = [__DIR__ . '/' . ($argv[2] ?? 'prepared.php')];
    foreach ([0, 1, 4, 5, 9] as $level) {
        file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/$level", ["phar://$phar/conf/config.level$level.neon", "$root/config.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
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
        $out[$level] = $records;
    }
    $file = $files[0];
    $source = file_get_contents($file);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
    $parser = (new PhpParser\ParserFactory())->createForNewestSupportedVersion();
    $producer = [];
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$producer, $source, $parser): void {
            if ($node instanceof PhpParser\Node\Expr\ArrayDimFetch
                && str_contains(strtok(substr($source, $node->getEndFilePos() + 1), "\n"), '// input:')) {
                $producer[] = ['line' => $node->getStartLine(), 'error' => $scope->getType($node) instanceof PHPStan\Type\ErrorType];
            }
            if (!$node instanceof PhpParser\Node\Expr\FuncCall || $node->name->toString() !== 'observe') return;
            $tail = substr($source, $node->getEndFilePos() + 1);
            if (!preg_match('/^; \/\/ holders: ([^\n]+)/', $tail, $match)) return;
            $reads = [];
            foreach (explode(' | ', $match[1]) as $text) {
                $expr = $parser->parse("<?php $text;")[0]->expr;
                $record = ['expression' => $text, 'certainty' => $scope->hasExpressionType($expr)->describe()];
                if ($expr instanceof PhpParser\Node\Expr\Variable) {
                    $record['value'] = $scope->getType($expr)->describe(PHPStan\Type\VerbosityLevel::precise());
                }
                if ($scope->getType($expr) instanceof PHPStan\Type\ErrorType) $record['error'] = true;
                $reads[] = $record;
            }
            $producer[] = ['line' => $node->getStartLine(), 'reads' => $reads];
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('Observation capture failed');
    echo json_encode(['diagnostics' => $out, 'producer' => $producer], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
