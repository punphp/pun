<?php
// PHPStan 2.3.1. php capture.php /absolute/path/to/phpstan.phar > expected.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-native-calls-' . getmypid();
mkdir($root);
$out = ['phar_sha256' => hash_file('sha256', $phar), 'sources' => [], 'types' => []];
try {
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
    foreach (['certainty', 'boundaries'] as $case) {
        $file = __DIR__ . "/$case.php";
        $source = file_get_contents($file);
        $out['sources'][$case] = hash('sha256', $source);
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/$case", ["phar://$phar/conf/config.level0.neon", "$root/config.neon"], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
        // A direct assignment target is not a value evaluation/argument observation.
        $targets = [];
        foreach ((new PhpParser\NodeFinder())->findInstanceOf(
            $container->getService('pathRoutingParser')->parseFile($file), PhpParser\Node\Expr\Assign::class,
        ) as $assignment) {
            $targets[$assignment->var->getStartFilePos() . ':' . ($assignment->var->getEndFilePos() + 1)] = true;
        }
        $types = [];
        $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
            $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
            static function ($node, $scope) use (&$types, $source, $case, $targets): void {
                if (!$node instanceof PhpParser\Node\Expr) return;
                $start = $node->getStartFilePos();
                $end = $node->getEndFilePos() + 1;
                if (isset($targets[$start . ':' . $end])) return;
                $tail = substr($source, $end, 24);
                $available = !str_starts_with($tail, ' /* unavailable */');
                if ($case === 'certainty') {
                    if (!$node instanceof PhpParser\Node\Expr\Variable && !$node instanceof PhpParser\Node\Expr\FuncCall) return;
                    if ($scope->getType($node) instanceof PHPStan\Type\ErrorType) return;
                } elseif (!str_starts_with($tail, ' /* native */') && $available) {
                    return;
                }
                $types[$start . ':' . $end] = [
                    'line' => $node->getStartLine(),
                    'source' => substr($source, $start, $end - $start),
                    'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                    'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                    // Unavailable points document the intentionally partial provider, not an oracle type.
                    'available' => $available,
                ];
            },
        );
        ksort($types, SORT_NATURAL);
        $out['types'][$case] = $types;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
