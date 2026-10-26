<?php

// php capture-pipe-parentheses.php /absolute/path/to/phpstan-2.3.1.phar > pipe-parentheses-expected.json
// Public Analyser, level 0. Explicit PHP 8.5 admits T_PIPE; older targets reverse-emulate it into | and >.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-pipe-parentheses-oracle-' . getmypid();
mkdir($root);
function rows(array $errors): array {
    $out = [];
    foreach ($errors as $error) {
        $out[] = [
            'file' => basename($error->getFile()), 'line' => $error->getLine(),
            'identifier' => $error->getIdentifier(), 'message' => $error->getMessage(),
            'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
        ];
    }
    usort($out, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
    return $out;
}
function analyse(string $root, string $phar, array $files, string $config): array {
    file_put_contents("$root/config.neon", "parameters:\n$config");
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$root/config.neon"], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $processed = $container->getByType(PHPStan\Analyser\Ignore\IgnoredErrorHelper::class)->initialize()->process(
        $result->getUnorderedErrors(), false, $files, false,
    );
    return ['errors' => rows($processed->getNotIgnoredErrors()), 'general' => $processed->getOtherIgnoreMessages()];
}
try {
    $files = [];
    foreach (json_decode(file_get_contents(__DIR__ . '/pipe-parentheses-inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $files[] = $file = "$root/$name.php";
        file_put_contents($file, "<?php\n$code\n");
    }
    $out = ['analysed' => analyse($root, $phar, $files, "    phpVersion: 80500\n")];
    foreach (json_decode(file_get_contents(__DIR__ . '/pipe-parentheses-scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/scan-$name";
        mkdir($dir);
        file_put_contents("$dir/symbols.php", "<?php\n$code\n");
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        $out['scanned'][$name] = analyse($dir, $phar, ["$dir/main.php"], "    phpVersion: 80500\n    scanFiles: [symbols.php]\n");
    }
    $dir = "$root/order";
    mkdir($dir);
    copy(__DIR__ . '/pipe-parentheses-order.php', "$dir/order.php");
    $out['order'] = analyse($dir, $phar, ["$dir/order.php"], "    phpVersion: 80500\n    ignoreErrors:\n        - identifier: phpstan.parse\n");
    foreach ([80000, 80100, 80200, 80400] as $version) {
        $dir = "$root/version-$version";
        mkdir($dir);
        file_put_contents("$dir/input.php", "<?php\n1 |> fn(\$x) => \$x;\n");
        $out['older'][$version] = analyse($dir, $phar, ["$dir/input.php"], "    phpVersion: $version\n");
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
