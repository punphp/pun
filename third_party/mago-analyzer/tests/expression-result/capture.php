<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar [fixture.php] > expected.tsv
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
require __DIR__ . '/extensions.php';
require __DIR__ . '/controls.php';
$root = sys_get_temp_dir() . '/pun-expression-results-' . getmypid();
mkdir($root);
try {
    $captureControls = !isset($argv[2]);
    $file = $captureControls ? __DIR__ . '/matrix.php' : realpath($argv[2]);
    if ($file === false) throw new RuntimeException('Fixture does not exist');
    $config = "parameters:\n    phpVersion: 80500\nservices:\n";
    foreach (['ResultFunctionProvider' => 'phpstan.broker.dynamicFunctionReturnTypeExtension', 'ResultMethodProvider' => 'phpstan.broker.dynamicMethodReturnTypeExtension', 'ResultExpressionOverride' => 'phpstan.broker.expressionTypeResolverExtension'] as $class => $tag) {
        $config .= "    -\n        class: $class\n        tags: [$tag]\n";
    }
    file_put_contents("$root/config.neon", $config);
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create("$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$root/config.neon"], [$file]);
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $resolver = $container->getByType(PHPStan\Analyser\NodeScopeResolver::class);
    $resolver->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $rows = [];
    $controls = null;
    $factory = $container->getByType(PHPStan\Analyser\ExpressionResultFactory::class);
    $describe = static fn($type) => $type->describe(PHPStan\Type\VerbosityLevel::precise());
    $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile($file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$rows, &$controls, $source, $resolver, $describe, $factory, $captureControls): void {
            if (!$node instanceof PhpParser\Node\Expr || substr($source, $node->getEndFilePos() + 1, 12) !== '/* result */') return;
            $scope = $scope->toWalkScope();
            $result = $resolver->processExprOnDemand($node, $scope, new PHPStan\Analyser\ExpressionResultStorage());
            $raw = (new ReflectionMethod($result, 'resolveOwnRawType'))->invoke($result, false);
            $text = substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1);
            $rawNative = (new ReflectionMethod($result, 'resolveOwnRawType'))->invoke($result, true);
            $rows[$node->getStartFilePos()] = implode("\t", [$node->getStartLine(), $text, $describe($result->getType()), $describe($raw), $describe($result->getKeepVoidType(false)), $describe($result->getNativeType()), $describe($rawNative), $describe($result->getKeepVoidType(true)), $node->getStartFilePos(), $node->getEndFilePos() + 1]);
            if ($captureControls && $controls === null) $controls = resultControls($factory, $scope, $node, $describe);
        });
    ksort($rows);
    echo implode("\n", $rows), "\n";
    if ($controls !== null) file_put_contents(__DIR__ . '/controls.tsv', $controls);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    if ($result->getInternalErrors() !== []) throw new RuntimeException('Internal PHPStan error');
    $errors = array_map(static fn($error) => [$error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored()], $result->getUnorderedErrors());
    sort($errors);
    fwrite(STDERR, json_encode($errors, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR) . "\n");
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
