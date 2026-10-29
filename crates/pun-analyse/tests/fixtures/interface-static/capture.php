<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
declare(strict_types=1);

$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-interface-static-' . getmypid();
mkdir($root);
$files = [__DIR__ . '/direct.php', __DIR__ . '/neighbors.php'];
$out = ['types' => [], 'errors' => []];
try {
    foreach ([0, 5, 9] as $level) {
        $config = "$root/level-$level.neon";
        file_put_contents($config, "parameters:\n    phpVersion: 80500\n    checkThisOnly: false\n");
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$root/cache-$level", ['phar://' . $phar . "/conf/config.level$level.neon", $config], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
        if ($level === 0) {
            foreach ($files as $file) {
                $source = file_get_contents($file);
                $types = [];
                $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                    $file, array_fill_keys($files, true), new PHPStan\Rules\DirectRegistry([]),
                    new PHPStan\Collectors\Registry([]),
                    static function ($node, $scope) use (&$types, $source): void {
                        if (!$node instanceof PhpParser\Node\Expr\StaticCall) return;
                        $type = $scope->getType($node);
                        $raw = $scope->getKeepVoidType($node);
                        $types[] = [
                            'start' => $node->getStartFilePos(), 'end' => $node->getEndFilePos() + 1,
                            'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                            'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
                            'error' => $type instanceof PHPStan\Type\ErrorType,
                            'raw_type' => $raw->describe(PHPStan\Type\VerbosityLevel::precise()),
                            'raw_error' => $raw instanceof PHPStan\Type\ErrorType,
                        ];
                    },
                );
                $out['types'][basename($file)] = $types;
            }
        }
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, true, true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan reported an internal error');
        $errors = array_map(static fn($error) => [
            basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
            $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        usort($errors, static fn($a, $b) => json_encode($a) <=> json_encode($b));
        $out['errors'][$level] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n";
} finally {
    exec('rm -rf ' . escapeshellarg($root));
}
