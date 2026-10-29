<?php
// Observe the pinned producer, independently of the rule diagnostics and Rust implementation.
// php capture-producer.php /absolute/path/to/phpstan.phar [output.json] [case,...]
declare(strict_types=1);

$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-closure-write-producer-' . getmypid();
mkdir($temporary);
$out = [];
try {
    foreach (isset($argv[3]) ? explode(',', $argv[3]) : ['captures', 'overwrites', 'reads', 'boundaries'] as $case) {
        $file = __DIR__ . "/$case.php";
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$temporary/$case", ['phar://' . $phar . '/conf/config.level9.neon'], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
        $records = [];
        $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
            $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
            static function ($node) use (&$records): void {
                if (!$node instanceof PHPStan\Node\VariableWritesNode) return;
                $closure = $node->getFunctionLike();
                if (!$closure instanceof PhpParser\Node\Expr\Closure) return;
                foreach ($closure->uses as $capture) {
                    $write = $node->getWriteForNode($capture->var);
                    $records[] = [
                        'closureLine' => $closure->getStartLine(), 'captureLine' => $capture->var->getStartLine(),
                        'name' => $capture->var->name, 'byRef' => $capture->byRef, 'tracked' => $write !== null,
                        'read' => $write !== null ? $node->isRead($write) : null,
                        'used' => $write !== null ? $node->isUsed($write) : null,
                        'referenced' => $node->isVariableReferenced($capture->var->name),
                        'allNames' => $node->areAllVariableNamesReferenced(), 'opaque' => $node->isOpaque(),
                    ];
                }
            },
        );
        usort($records, static fn($a, $b) => [$a['closureLine'], $a['captureLine'], $a['name']] <=> [$b['closureLine'], $b['captureLine'], $b['name']]);
        $out[$case] = $records;
        fwrite(STDERR, "$case: " . count($records) . " complete producer records\n");
    }
    file_put_contents($argv[2] ?? __DIR__ . '/producer.json', json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR) . "\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
