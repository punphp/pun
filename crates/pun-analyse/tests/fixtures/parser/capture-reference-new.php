<?php

// php capture-reference-new.php /absolute/path/to/phpstan-2.3.1.phar > reference-new-expected.json
// Public Analyser, level 0. Separate PHP 5.6/7.0 controls use the phar's public PhpParser factory:
// PHPStan config admits only PHP >= 7.1, while PhpVersion::allowsAssignNewByReference changes at 7.0.
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-parser-reference-new-oracle-' . getmypid();
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
    foreach (json_decode(file_get_contents(__DIR__ . '/reference-new-inputs.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $files[] = $file = "$root/$name.php";
        file_put_contents($file, "<?php\n$code\n");
    }
    $out = ['analysed' => analyse($root, $phar, $files, "    phpVersion: 80500\n")];
    foreach (json_decode(file_get_contents(__DIR__ . '/reference-new-scans.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/scan-$name";
        mkdir($dir);
        file_put_contents("$dir/symbols.php", "<?php\n$code\n");
        copy(__DIR__ . '/scan-main.php', "$dir/main.php");
        $out['scanned'][$name] = analyse($dir, $phar, ["$dir/main.php"], "    phpVersion: 80500\n    scanFiles: [symbols.php]\n");
    }
    $dir = "$root/order";
    mkdir($dir);
    copy(__DIR__ . '/reference-new-order.php', "$dir/order.php");
    $out['order'] = analyse($dir, $phar, ["$dir/order.php"], "    phpVersion: 80500\n    ignoreErrors:\n        - identifier: phpstan.parse\n");
    foreach ([70100, 70400, 80000, 80100, 80200, 80400, 80500] as $version) {
        $dir = "$root/version-$version";
        mkdir($dir);
        file_put_contents("$dir/input.php", "<?php\n\$value =& new stdClass;\n");
        $out['versions'][$version] = analyse($dir, $phar, ["$dir/input.php"], "    phpVersion: $version\n");
    }
    foreach (json_decode(file_get_contents(__DIR__ . '/reference-new-unsupported.json'), true, flags: JSON_THROW_ON_ERROR) as $name => $code) {
        $dir = "$root/unsupported-$name";
        mkdir($dir);
        file_put_contents("$dir/input.php", "<?php\n$code\n");
        $out['unsupported'][$name] = analyse($dir, $phar, ["$dir/input.php"], "    phpVersion: 80500\n");
    }
    foreach (['5.6' => 50600, '7.0' => 70000] as $version => $id) {
        $parser = (new PhpParser\ParserFactory())->createForVersion(PhpParser\PhpVersion::fromString($version));
        $handler = new PhpParser\ErrorHandler\Collecting();
        $parser->parse("<?php\n\$value =& new stdClass;\n", $handler);
        $out['parserVersions'][$id] = array_map(static fn($error) => $error->getMessage(), $handler->getErrors());
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
