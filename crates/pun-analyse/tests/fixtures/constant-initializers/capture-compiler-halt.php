<?php

// php capture-compiler-halt.php /absolute/path/to/phpstan-2.3.1.phar > expected-compiler-halt.json
// Each complete source has its own symbol table. Use the analysed-file rich parser, as MutatingScope does.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-compiler-halt-oracle-' . getmypid();
mkdir($root);
try {
    $out = [];
    foreach (json_decode(file_get_contents(__DIR__ . '/compiler-halt-inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
        mkdir($dir = "$root/$name");
        file_put_contents($file = "$dir/input.php", $source);
        file_put_contents("$dir/config.neon", "parameters:\n    phpVersion: 80500\n");
        $container = (new PHPStan\DependencyInjection\ContainerFactory($dir))->create(
            "$dir/cache", ['phar://' . $phar . '/conf/config.level1.neon', "$dir/config.neon"], [$file],
        );
        $parser = $container->getService('pathRoutingParser');
        $parser->setAnalysedFiles([$file]);
        $nodes = $parser->parseFile($file);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
        $errors = [];
        foreach ($result->getUnorderedErrors() as $error) {
            $errors[] = [
                'line' => $error->getLine(), 'identifier' => $error->getIdentifier(),
                'message' => $error->getMessage(), 'tip' => $error->getTip(),
                'ignorable' => $error->canBeIgnored(),
            ];
        }
        usort($errors, static fn($a, $b) => [$a['line'], $a['message']] <=> [$b['line'], $b['message']]);
        $out[$name] = [
            'rootNodes' => array_map(static fn($node) => $node->getType(), $nodes),
            'errors' => $errors,
        ];
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
