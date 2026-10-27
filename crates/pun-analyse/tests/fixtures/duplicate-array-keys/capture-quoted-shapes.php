<?php

// php capture-quoted-shapes.php /absolute/path/to/phpstan-2.3.1.phar > expected-quoted-shapes.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-quoted-shapes-' . getmypid();
mkdir($root);
$file = __DIR__ . '/quoted-shapes.php';
try {
    $config = "$root/config.neon";
    file_put_contents($config, "parameters:\n    phpVersion: 80400\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', 'phar://' . $phar . '/conf/bleedingEdge.neon', $config], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $provider = $container->getByType(PHPStan\Reflection\ReflectionProvider::class);
    $shapes = [];
    foreach (['binary', 'quotes', 'equal', 'numeric', 'ascii'] as $name) {
        $type = $provider->getFunction(new PhpParser\Node\Name('QuotedShapeKeys\\' . $name), null)->getVariants()[0]->getParameters()[0]->getType();
        $arrays = $type->getConstantArrays();
        if (count($arrays) !== 1) {
            throw new LogicException("Expected one constant shape for $name");
        }
        $array = $arrays[0];
        $entries = [];
        foreach ($array->getKeyTypes() as $i => $key) {
            $value = $key->getValue();
            $entries[] = [is_int($value) ? ['int', $value] : ['string', bin2hex($value)], bin2hex($array->getValueTypes()[$i]->getValue())];
        }
        sort($entries);
        $shapes[$name] = $entries;
    }
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    $errors = array_map(static fn($error) => [
        'line' => $error->getLine(), 'identifier' => $error->getIdentifier(),
        'message' => $error->getMessage(), 'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    usort($errors, static fn($a, $b) => [$a['line'], $a['message']] <=> [$b['line'], $b['message']]);
    // Only diagnostic presentation substitutes invalid UTF-8. Shape identities above remain hexadecimal.
    echo json_encode(['shapes' => $shapes, 'errors' => $errors], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_INVALID_UTF8_SUBSTITUTE | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
