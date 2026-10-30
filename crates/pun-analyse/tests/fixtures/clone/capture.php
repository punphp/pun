<?php
// php capture.php /absolute/path/to/phpstan/bootstrap.php > expected.json
require $argv[1];
$file = __DIR__ . '/values.php';
$tmp = sys_get_temp_dir() . '/pun-clone-' . getmypid();
mkdir($tmp);
try {
    file_put_contents("$tmp/config.neon", "parameters:\n    level: 7\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$tmp/cache", ['phar://' . dirname($argv[1]) . '/phpstan.phar/conf/config.level7.neon', "$tmp/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $resolver = $container->getByType(PHPStan\Analyser\NodeScopeResolver::class);
    $resolver->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $out = [];
    $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function (PhpParser\Node $node, PHPStan\Analyser\Scope $scope) use (&$out, $source): void {
            if (!$node instanceof PhpParser\Node\Expr\Clone_ && !($node instanceof PhpParser\Node\Expr\Variable && str_contains(explode("\n", $source)[$node->getStartLine() - 1], '// observe'))) {
                return;
            }
            $type = $scope->getType($node);
            $out[(string)$node->getStartFilePos()] = [
                'line' => $node->getStartLine(),
                'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
                'explicit' => $type instanceof PHPStan\Type\MixedType ? $type->isExplicitMixed() : null,
            ];
            if ($node instanceof PhpParser\Node\Expr\Clone_) {
                $operand = $scope->getType($node->expr);
                $out[(string)$node->getStartFilePos()]['operand'] = [
                    'offset' => $node->expr->getStartFilePos(),
                    'length' => $node->expr->getEndFilePos() - $node->expr->getStartFilePos() + 1,
                    'type' => $operand->describe(PHPStan\Type\VerbosityLevel::precise()),
                    'explicit' => $operand instanceof PHPStan\Type\MixedType ? $operand->isExplicitMixed() : null,
                ];
            }
        },
    );
    ksort($out, SORT_NUMERIC);
    $errors = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true)->getUnorderedErrors();
    $errors = array_map(static fn($error) => [$error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored()], $errors);
    sort($errors);
    echo json_encode(['expressions' => $out, 'errors' => $errors], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
