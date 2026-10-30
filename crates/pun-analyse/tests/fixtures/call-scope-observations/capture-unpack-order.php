<?php
// php capture-unpack-order.php /absolute/path/to/phpstan-2.3.1.phar unpack-order.php
require 'phar://' . $argv[1] . '/vendor/autoload.php';
$file = __DIR__ . '/' . $argv[2];
$root = sys_get_temp_dir() . '/pun-unpack-order-' . getmypid();
mkdir($root);
try {
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $argv[1] . '/conf/config.level0.neon', "$root/config.neon"], [$file],
    );
    $parser = $container->getService('pathRoutingParser');
    $parser->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $text = static fn($node) => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1);
    $finder = new PhpParser\NodeFinder();
    $roots = [];
    foreach ($finder->findInstanceOf($parser->parseFile($file), PhpParser\Node\Stmt\Function_::class) as $function) {
        if (!in_array($function->name->toString(), ['ordered', 'earlierWrite', 'laterWrite', 'earlierImpurity', 'laterImpurity'], true)) continue;
        foreach ($finder->findInstanceOf($function->stmts, PhpParser\Node\Expr\CallLike::class) as $call) {
            foreach ($call->getArgs() as $arg) {
                if (!$arg->unpack) continue;
                $roots[$arg->value->getStartFilePos()] = ['expression' => $text($arg->value), 'line' => $arg->value->getStartLine(), 'end' => $arg->value->getEndFilePos() + 1, 'unpacked' => $arg->unpack];
            }
        }
    }
    $arguments = [];
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$arguments, $roots): void {
            $start = $node->getStartFilePos();
            if (!$node instanceof PhpParser\Node\Expr || !isset($roots[$start]) || $node->getEndFilePos() + 1 !== $roots[$start]['end']) return;
            $type = $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise());
            if (isset($arguments[$start]) && $arguments[$start]['type'] !== $type) throw new RuntimeException('Inconsistent repeated argument');
            $arguments[$start] = $roots[$start] + ['type' => $type, 'visits' => ($arguments[$start]['visits'] ?? 0) + 1];
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('Oracle analysis failed');
    ksort($arguments, SORT_NUMERIC);
    if (count($arguments) !== count($roots)) throw new RuntimeException('Unevaluated argument: ' . json_encode(array_diff_key($roots, $arguments)));
    echo json_encode(['calls' => new stdClass(), 'arguments' => $arguments], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
