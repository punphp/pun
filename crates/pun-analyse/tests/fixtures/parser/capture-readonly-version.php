<?php

// php capture-readonly-version.php /absolute/path/to/phpstan.phar > readonly-version-expected.json
// Public Analyser, level 0; PHP 8.0's reverse readonly keyword emulation, and neighboring target versions.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-readonly-version-oracle-' . getmypid();
mkdir($root);
function analyse(string $root, string $phar, array $files, int $version, array $extra = []): array {
    file_put_contents("$root/version-$version.neon", "parameters:\n    phpVersion: $version\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache-$version", ['phar://' . $phar . '/conf/config.level0.neon', "$root/version-$version.neon", ...$extra], $files,
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
    foreach (json_decode(file_get_contents(__DIR__ . '/readonly-version-inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $files[] = $file = "$root/$name.php";
        $namespace = 'ReadonlyVersion_' . str_replace('-', '_', $name);
        file_put_contents($file, "<?php\nnamespace $namespace;\n$code\n");
    }
    $out = [];
    foreach ([80000, 80100, 80200, 80400, 80500] as $version) {
        $out['analysed'][$version] = analyse($root, $phar, $files, $version);
    }
    foreach (json_decode(file_get_contents(__DIR__ . '/readonly-version-scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/scan-$name";
        mkdir($dir);
        file_put_contents("$dir/symbols.php", "<?php\n$code\n");
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        file_put_contents("$dir/config.neon", "parameters:\n    scanFiles: [symbols.php]\n");
        foreach ([80000, 80100, 80200, 80400, 80500] as $version) {
            $out['scanned'][$name][$version] = analyse($dir, $phar, ["$dir/main.php"], $version, ["$dir/config.neon"]);
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
