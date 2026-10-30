<?php
// php capture-single-call-arguments.php /absolute/path/to/phpstan-2.3.1.phar
require 'phar://' . $argv[1] . '/vendor/autoload.php';
$file = realpath(__DIR__ . '/' . ($argv[2] ?? 'single-call-arguments.php'));
$root = sys_get_temp_dir() . '/pun-single-call-arguments-' . getmypid();
mkdir($root);
try {
    $config = "parameters:\n    phpVersion: 80500\n";
    if (in_array('--providers', $argv, true)) {
        require __DIR__ . '/single-call-provider-extension.php';
        $config .= "services:\n    -\n        class: SingleCallReturnExtension\n        tags: [phpstan.broker.dynamicMethodReturnTypeExtension]\n";
    }
    file_put_contents("$root/config.neon", $config);
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $argv[1] . '/conf/config.level0.neon', "$root/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $records = [];
    $dynamicCalls = in_array('--dynamic-calls', $argv, true);
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$records, $source, $dynamicCalls): void {
            if (!$node instanceof PhpParser\Node\Expr\PreInc
                && !($node instanceof PhpParser\Node\Expr\Variable && in_array($node->name, ['i', 'value'], true))
                && !($dynamicCalls && $node instanceof PhpParser\Node\Expr\FuncCall && $node->name instanceof PhpParser\Node\Expr)
                && !$node instanceof PhpParser\Node\Expr\MethodCall) return;
            $start = $node->getStartFilePos();
            $end = $node->getEndFilePos() + 1;
            $type = $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise());
            if (isset($records[$start]) && $records[$start]['type'] !== $type) throw new RuntimeException('Repeated type changed');
            $records[$start] = [
                'end' => $end,
                'expression' => substr($source, $start, $end - $start),
                'type' => $type,
                'visits' => ($records[$start]['visits'] ?? 0) + 1,
            ];
        },
    );
    if ($result->getErrors() !== []) throw new RuntimeException('Oracle analysis failed');
    ksort($records, SORT_NUMERIC);
    echo json_encode($records, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
