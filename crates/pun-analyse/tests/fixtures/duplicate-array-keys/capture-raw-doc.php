<?php

// php capture-raw-doc.php /absolute/path/to/phpstan-2.3.1.phar > expected-raw-doc.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-raw-doc-oracle-' . getmypid();
mkdir($root);
// The markers construct actual binary source bytes, not escaped PHPDoc spellings.
$source = str_replace(['<FF>', '<FE>', '<REP>'], ["\xff", "\xfe", "\xef\xbf\xbd"], <<<'SOURCE'
<?php
namespace RawDocBytes;
/**
 * @param array{
 * '<FF>': 'first', '<FE>': 'second', '<REP>': 'third'
 * } $shape
 */
function shape(array $shape): void {
    $a = [...[...$shape], "\xff" => 1];
    $b = [...[...$shape], "\xfe" => 1];
    $c = [...[...$shape], "<REP>" => 1];
    echo count($a);
}
/** @param '<FF>' $key */
function scalar(string $key): void {
    $a = [$key => 1, "\xff" => 2];
    $b = [$key => 1, "\xfe" => 2, "\xef\xbf\xbd" => 3];
}
function inline(string $key): void {
    /** @var '<FE>' $key */
    $a = [$key => 1, "\xfe" => 2];
    $b = [$key => 1, "\xff" => 2, "\xef\xbf\xbd" => 3];
}
/** @phpstan-type Keys array{'<FF>': 'first', "\xff": 'last', '<FE>': 'other'} */
class Aliases {
    /** @param Keys $shape */
    public function useAlias(array $shape): void {
        $a = [...[...$shape], "\xff" => 1];
        $b = [...[...$shape], "\xfe" => 1];
    }
}
/** @phpstan-assert '<FF>' $key */
function assertKey(string $key): void {}
function assertions(string $key): void {
    assertKey($key);
    $a = [$key => 1, "\xff" => 2];
    $b = [$key => 1, "\xfe" => 2, "\xef\xbf\xbd" => 3];
}
/** @return '<FF>' */
function returns(): string { return "\xff"; }
/** @method '<FE>' key() */
class Magic {}
function returned(Magic $magic): void {
    $a = [returns() => 1, "\xff" => 2];
    $b = [$magic->key() => 1, "\xfe" => 2];
}
/** @param '<FF>' '<FE>' $key */
function malformed(string $key): void {}
/** @template T of '<FF>' = '<FE>' */
class TemplateDefault {}
/**
 * @template T of '<FF>' = '<FE>'
 * @param T $key
 */
function signatureDefault($key): void {}
SOURCE
);
$source = str_replace("\n", "\r\n", $source);
$file = "$root/raw.php";
file_put_contents($file, $source);
try {
    $captures = [];
    foreach ([0, 2] as $level) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory($root))->create(
            "$root/cache-$level", ['phar://' . $phar . "/conf/config.level$level.neon", 'phar://' . $phar . '/conf/bleedingEdge.neon'], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
        $errors = array_map(static fn($error) => [
            'line' => $error->getLine(), 'identifier' => $error->getIdentifier(),
            'message' => $error->getMessage(), 'tip' => $error->getTip(), 'ignorable' => $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        usort($errors, static fn($a, $b) => [$a['line'], $a['message']] <=> [$b['line'], $b['message']]);
        $captures[$level] = $errors;
    }
    echo json_encode(['source_hex' => bin2hex($source), 'levels' => $captures], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_INVALID_UTF8_SUBSTITUTE | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
