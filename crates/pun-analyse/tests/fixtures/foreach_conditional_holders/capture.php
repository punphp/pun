<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-foreach-holders-' . getmypid();
mkdir($root);
$files = [
    'bug13054' => realpath(__DIR__ . '/../rules/data/Methods/data/bug-13054.php'),
    'calls' => __DIR__ . '/calls.php',
    'scopes' => __DIR__ . '/scopes.php',
];
$out = ['phar_sha256' => hash_file('sha256', $phar)];
try {
    foreach ($files as $case => $file) {
        foreach ($case === 'scopes' ? [0] : [0, 5] as $level) {
            file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$root/$case-$level", ["phar://$phar/conf/config.level$level.neon", "$root/config.neon"], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            $source = file_get_contents($file);
            $observations = [];
            if ($case === 'scopes') {
                $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                    $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
                    static function ($node, $scope) use (&$observations, $source): void {
                        if (!$node instanceof PhpParser\Node\Expr || !str_starts_with(substr($source, $node->getEndFilePos() + 1), '; // observe')) return;
                        $observations[$node->getStartFilePos()] = [
                            'start' => $node->getStartFilePos(), 'end' => $node->getEndFilePos() + 1,
                            'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                            'type' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                            'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                        ];
                    },
                );
            }
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error');
            $errors = array_map(static fn($error) => [
                basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
                $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            sort($errors);
            ksort($observations, SORT_NUMERIC);
            $out["$case-$level"] = ['errors' => $errors, 'observations' => array_values($observations)];
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
