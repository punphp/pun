<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar [source.php] [probes|all]
require 'phar://' . $argv[1] . '/vendor/autoload.php';
$file = realpath($argv[2] ?? __DIR__ . '/contract.php');
$probesOnly = ($argv[3] ?? 'probes') === 'probes';
$root = sys_get_temp_dir() . '/pun-nested-reference-' . getmypid();
mkdir($root);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $argv[1] . '/conf/config.level4.neon'], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $events = [];
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$events, $source, $probesOnly): void {
            if (!$node instanceof PhpParser\Node\Expr) return;
            $start = $node->getStartFilePos();
            $end = $node->getEndFilePos() + 1;
            $rest = substr($source, $end, (strpos($source, "\n", $end) ?: strlen($source)) - $end);
            if ($probesOnly) {
                if (!str_contains($rest, '// probe')) return;
                if (!$node instanceof PhpParser\Node\Expr\CallLike && !$node instanceof PhpParser\Node\Expr\Assign) return;
                $operands = [$node];
                if ($node instanceof PhpParser\Node\Expr\Assign) {
                    $operands[] = $node->expr;
                } else {
                    foreach ($node->getArgs() as $argument) $operands[] = $argument->value;
                }
                foreach ($operands as $operand) {
                    $start = $operand->getStartFilePos();
                    $end = $operand->getEndFilePos() + 1;
                    $key = "$start:$end";
                    $events[$key] = [
                        'start' => $start, 'end' => $end, 'expression' => substr($source, $start, $end - $start),
                        'full' => $scope->getType($operand)->describe(PHPStan\Type\VerbosityLevel::precise()),
                        'keep_void' => $scope->getKeepVoidType($operand)->describe(PHPStan\Type\VerbosityLevel::precise()),
                        'native' => $scope->getNativeType($operand)->describe(PHPStan\Type\VerbosityLevel::precise()),
                        'visits' => ($events[$key]['visits'] ?? 0) + 1,
                    ];
                }
                return;
            }
            if (!str_contains($rest, '// probe') && ($probesOnly || !($node instanceof PhpParser\Node\Expr\FuncCall))) return;
            $key = "$start:$end";
            $events[$key] = [
                'start' => $start, 'end' => $end, 'expression' => substr($source, $start, $end - $start),
                'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'keep_void' => $scope->getKeepVoidType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'visits' => ($events[$key]['visits'] ?? 0) + 1,
            ];
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('Oracle analysis failed');
    ksort($events, SORT_NATURAL);
    echo json_encode($events, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
