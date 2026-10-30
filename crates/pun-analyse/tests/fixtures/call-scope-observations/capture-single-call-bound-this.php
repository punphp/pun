<?php
// php capture-single-call-bound-this.php /absolute/path/to/phpstan-2.3.1.phar
require 'phar://' . $argv[1] . '/vendor/autoload.php';
$file = realpath(__DIR__ . '/single-call-bound-this.php');
$root = sys_get_temp_dir() . '/pun-single-call-this-' . getmypid();
mkdir($root);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $argv[1] . '/conf/config.level0.neon'], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $events = [];
    $calls = [];
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$events, &$calls, $source): void {
            if (!($node instanceof PhpParser\Node\Expr\Variable && $node->name === 'this')
                && !$node instanceof PhpParser\Node\Expr\ClassConstFetch
                && !$node instanceof PhpParser\Node\Expr\MethodCall) return;
            $start = $node->getStartFilePos();
            $end = $node->getEndFilePos() + 1;
            $key = "$start:$end";
            $events[$key] = [
                'start' => $start, 'end' => $end,
                'expression' => substr($source, $start, $end - $start),
                'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                'visits' => ($events[$key]['visits'] ?? 0) + 1,
            ];
            if ($node instanceof PhpParser\Node\Expr\MethodCall && $node->name instanceof PhpParser\Node\Identifier) {
                $var = $node->var;
                $calls[(string) $start] = [
                    'expression' => substr($source, $start, $end - $start),
                    'names' => [[
                        'name' => $node->name->name,
                        'types' => [[
                            'start' => $var->getStartFilePos(), 'end' => $var->getEndFilePos() + 1,
                            'expression' => substr($source, $var->getStartFilePos(), $var->getEndFilePos() + 1 - $var->getStartFilePos()),
                            'type' => $scope->getType($var)->describe(PHPStan\Type\VerbosityLevel::precise()),
                        ]],
                    ]],
                ];
            }
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('Oracle analysis failed');
    ksort($events, SORT_NATURAL);
    ksort($calls, SORT_NUMERIC);
    echo json_encode(['events' => $events, 'calls' => $calls], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
