<?php

// php capture-producer.php /absolute/path/to/phpstan-2.3.1.phar > producer.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-offset-producer-' . getmypid();
mkdir($root);
try {
    $file = __DIR__ . '/producer.php';
    $source = file_get_contents($file);
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ["phar://$phar/conf/config.level0.neon", "$root/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $parser = (new PhpParser\ParserFactory())->createForNewestSupportedVersion();
    $out = [];
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$out, $source, $parser): void {
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
                $reads[] = $record;
            }
            $out[] = ['line' => $node->getStartLine(), 'reads' => $reads];
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('Observation capture failed');
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
