<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-native-intersections-' . getmypid();
mkdir($root);
$file = __DIR__ . '/types.php';
try {
    $config = "$root/config.neon";
    file_put_contents($config, "parameters:\n    phpVersion: 80500\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', $config], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $nodes = $container->getService('defaultAnalysisParser')->parseFile($file);
    $provider = $container->getByType(PHPStan\Reflection\ReflectionProvider::class);
    $types = [];
    foreach ($nodes as $node) {
        if (!$node instanceof PhpParser\Node\Stmt\Function_) {
            continue;
        }
        $name = $node->name->toString();
        $variant = $provider->getFunction(new PhpParser\Node\Name($name), null)->getVariants()[0];
        foreach (['parameter' => $node->params[0]->type, 'return' => $node->returnType] as $slot => $hint) {
            $parsed = PHPStan\Type\ParserNodeTypeToPHPStanType::resolve($hint, null);
            $reflected = $slot === 'parameter' ? $variant->getParameters()[0]->getNativeType() : $variant->getNativeReturnType();
            $describe = static fn($type) => $type->describe(PHPStan\Type\VerbosityLevel::precise());
            if ($describe($parsed) !== $describe($reflected)) {
                throw new LogicException("Native reflection and parser differ for $name $slot");
            }
            $types[$name][$slot] = [
                'type' => $describe($reflected),
                'hint' => substr($source, $hint->getStartFilePos(), $hint->getEndFilePos() - $hint->getStartFilePos() + 1),
            ];
        }
    }
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    $errors = array_map(static fn($error) => [
        $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    sort($errors);
    echo json_encode(['types' => $types, 'errors' => $errors], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
