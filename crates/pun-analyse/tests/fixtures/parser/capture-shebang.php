<?php

// PHPStan 2.3.1 public Analyser, complete unmodified sources. Each source gets its own symbol table.
// php capture-shebang.php /absolute/path/to/phpstan.phar > shebang-expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-shebang-oracle-' . getmypid();
mkdir($root);
function analyse(string $root, string $phar, array $files, int $level, array $extra = []): array {
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . "/conf/config.level$level.neon", ...$extra], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $errors = [];
    foreach ($result->getErrors() as $error) {
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
    $out = [];
    foreach (['analysed' => 'shebang-inputs.json', 'scanned' => 'shebang-scans.json'] as $mode => $input) {
        foreach (json_decode(file_get_contents(__DIR__ . '/' . $input), true, flags: JSON_THROW_ON_ERROR) as $name => $source) {
            mkdir($dir = "$root/$mode-$name");
            if ($mode === 'analysed') {
                file_put_contents($file = "$dir/input.php", $source);
                $out[$mode][$name] = analyse($dir, $phar, [$file], 0);
            } else {
                file_put_contents("$dir/symbols.php", $source);
                copy(__DIR__ . '/scan-main.php', "$dir/main.php");
                file_put_contents($config = "$dir/config.neon", "parameters:\n    scanFiles: [symbols.php]\n");
                $out[$mode][$name] = analyse($dir, $phar, ["$dir/main.php"], 2, [$config]);
            }
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
