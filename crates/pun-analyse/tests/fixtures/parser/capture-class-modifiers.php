<?php

// php capture-class-modifiers.php /absolute/path/to/phpstan.phar > class-modifier-expected.json
// Public Analyser, level 0, without diagnostic filters. A unique namespace isolates each analysed input.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-class-modifiers-oracle-' . getmypid();
mkdir($root);
function analyse(string $root, string $phar, array $files, array $extra = []): array {
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', ...$extra], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $errors = [];
    foreach ($container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true)->getErrors() as $error) {
        $errors[] = [
            'file' => basename($error->getFile()), 'line' => $error->getLine(),
            'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
            'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
        ];
    }
    usort($errors, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
    return $errors;
}
try {
    $files = [];
    foreach (json_decode(file_get_contents(__DIR__ . '/class-modifier-inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $files[] = $file = "$root/$name.php";
        $namespace = 'ClassModifier_' . str_replace('-', '_', $name);
        file_put_contents($file, "<?php\nnamespace $namespace;\n$code\n");
    }
    $out = ['analysed' => analyse($root, $phar, $files)];
    foreach (json_decode(file_get_contents(__DIR__ . '/class-modifier-scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/scan-$name";
        mkdir($dir);
        file_put_contents("$dir/symbols.php", "<?php\n$code\n");
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        file_put_contents("$dir/config.neon", "parameters:\n    scanFiles: [symbols.php]\n");
        $out['scanned'][$name] = analyse($dir, $phar, ["$dir/main.php"], ["$dir/config.neon"]);
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
