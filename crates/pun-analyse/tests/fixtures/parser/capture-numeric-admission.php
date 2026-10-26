<?php

// php capture-numeric-admission.php /absolute/path/to/phpstan-2.3.1.phar > numeric-admission-expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-numeric-admission-oracle-' . getmypid();
mkdir($root);
function analyse(string $root, string $phar, array $files, int $version, string $extra = '', int $level = 0): array {
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: $version\n$extra");
    $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
        "$root/cache", ['phar://' . $phar . "/conf/config.level$level.neon", "$root/config.neon"], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
    $processed = $container->getByType(PHPStan\Analyser\Ignore\IgnoredErrorHelper::class)->initialize()->process(
        $result->getUnorderedErrors(), false, $files, false,
    );
    $rows = array_map(static fn($e) => [
        'file' => basename($e->getFile()), 'line' => $e->getLine(),
        'identifier' => $e->getIdentifier(), 'message' => $e->getMessage(),
        'tip' => $e->getTip(), 'ignorable' => $e->canBeIgnored(),
    ], $processed->getNotIgnoredErrors());
    usort($rows, static fn($a, $b) => [$a['file'], $a['line'], $a['message']] <=> [$b['file'], $b['line'], $b['message']]);
    return ['errors' => $rows, 'general' => $processed->getOtherIgnoreMessages()];
}
try {
    $inputs = json_decode(file_get_contents(__DIR__ . '/numeric-admission-inputs.json'), true, flags: JSON_THROW_ON_ERROR);
    $out = [];
    foreach (['analysed' => $inputs, 'boundaries' => json_decode(file_get_contents(__DIR__ . '/numeric-admission-boundaries.json'), true, flags: JSON_THROW_ON_ERROR)] as $group => $cases) {
        foreach ($cases as $name => $source) {
            $dir = "$root/$group-$name";
            mkdir($dir);
            file_put_contents("$dir/input.php", $source);
            $out[$group][$name] = analyse($dir, $phar, ["$dir/input.php"], 80500, "    ignoreErrors:\n        - identifier: phpstan.parse\n");
        }
    }
    foreach ([70100, 70300, 70400, 80000, 80100, 80200, 80400, 80500] as $version) {
        foreach (['legacy', 'valid', 'overflow-prefix', 'offset-bare'] as $name) {
            $dir = "$root/version-$version-$name";
            mkdir($dir);
            file_put_contents("$dir/input.php", $inputs[$name]);
            $out['versions'][$version][$name] = analyse($dir, $phar, ["$dir/input.php"], $version);
        }
    }
    $declarations = 'class BeforeScan {} function before_scan(): void {} const BEFORE_SCAN=1;';
    $after = 'class AfterScan {} function after_scan(): void {} const AFTER_SCAN=1;';
    foreach (['valid' => '077', 'invalid' => '08', 'invalid-first' => '08', 'invalid-last' => '08', 'overflow-prefix' => '010000000000000000000008'] as $name => $literal) {
        $dir = "$root/scan-$name";
        mkdir($dir);
        $parts = [$declarations, "\$a=$literal;", $after];
        if ($name === 'invalid-first') $parts = ["\$a=$literal;", $declarations, $after];
        if ($name === 'invalid-last') $parts = [$declarations, $after, "\$a=$literal;"];
        file_put_contents("$dir/symbols.php", "<?php\n" . implode("\n", $parts) . "\n");
        file_put_contents("$dir/main.php", "<?php\nnew BeforeScan(); before_scan(); echo BEFORE_SCAN;\nnew AfterScan(); after_scan(); echo AFTER_SCAN;\n");
        $out['scanned'][$name] = analyse($dir, $phar, ["$dir/main.php"], 80500, "    scanFiles: [symbols.php]\n", 1);
    }
    $entry = "$root/entry";
    mkdir($entry);
    file_put_contents("$entry/input.php", "<?php\n}\n\$a=08;\n");
    $out['entry'] = analyse($entry, $phar, ["$entry/input.php"], 80500);
    // Retain native token classes and PHPParser's literal values/spans, including pre-PHP-7 permissive octals.
    foreach ([50600, 70000, 80500] as $version) {
        $parser = (new PhpParser\ParserFactory())->createForVersion(PhpParser\PhpVersion::fromComponents(intdiv($version, 10000), intdiv($version % 10000, 100)));
        foreach (['legacy', 'valid', 'overflow-prefix', 'overflow-wide', 'overflow-edge', 'separated-prefix', 'offset-bare'] as $name) {
            $handler = new PhpParser\ErrorHandler\Collecting();
            $nodes = $parser->parse($inputs[$name], $handler);
            $literals = [];
            $visitor = function ($node) use (&$visitor, &$literals): void {
                if (is_array($node)) { foreach ($node as $child) $visitor($child); return; }
                if (!$node instanceof PhpParser\Node) return;
                if ($node instanceof PhpParser\Node\Scalar\Int_ || $node instanceof PhpParser\Node\Scalar\Float_) {
                    $literals[] = ['kind' => $node->getType(), 'raw' => $node->getAttribute('rawValue'), 'value' => $node->value, 'start' => $node->getStartFilePos(), 'end' => $node->getEndFilePos() + 1];
                }
                foreach ($node->getSubNodeNames() as $key) $visitor($node->$key);
            };
            $visitor($nodes);
            $out['literals'][$version][$name] = $literals;
            if ($name === 'legacy') {
                // PHPStan's config schema starts at 7.1; earlier policies come from its pinned parser.
                $out['legacy_policy'][$version] = array_map(static fn($e) => [
                    'line' => $e->getStartLine(), 'message' => $e->getMessage(),
                ], $handler->getErrors());
            }
        }
    }
    foreach ($inputs as $name => $source) {
        $out['tokens'][$name] = array_values(array_map(static fn($t) => [token_name($t[0]), $t[1], $t[2]], array_filter(token_get_all($source), static fn($t) => is_array($t) && in_array($t[0], [T_LNUMBER, T_DNUMBER, T_NUM_STRING]))));
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
