<?php

// php capture-wide-integers.php /absolute/path/to/phpstan-2.3.1.phar > expected-wide-integers.json
// Capture the complete public level-0 analyser output, plus PHPParser's literal kinds, bytes and values.
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-wide-integers-oracle-' . getmypid();
mkdir($root);
try {
    $file = __DIR__ . '/wide-integers.php';
    file_put_contents("$root/config.neon", "parameters:\n    phpVersion: 80400\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$root/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    if ($result->getInternalErrors() !== []) {
        throw new RuntimeException('PHPStan reported an internal error.');
    }
    $errors = array_map(static fn($e) => [
        'line' => $e->getLine(), 'identifier' => $e->getIdentifier(), 'message' => $e->getMessage(),
        'tip' => $e->getTip(), 'ignorable' => $e->canBeIgnored(),
    ], $result->getErrors());
    usort($errors, static fn($a, $b) => [$a['line'], $a['message']] <=> [$b['line'], $b['message']]);
    $parser = (new PhpParser\ParserFactory())->createForVersion(PhpParser\PhpVersion::fromString('8.4'));
    $nodes = $parser->parse(file_get_contents($file));
    $literals = [];
    $visit = function ($node) use (&$visit, &$literals): void {
        if (is_array($node)) { foreach ($node as $child) $visit($child); return; }
        if (!$node instanceof PhpParser\Node) return;
        if ($node instanceof PhpParser\Node\Scalar\Int_ || $node instanceof PhpParser\Node\Scalar\Float_) {
            $literals[] = [
                'kind' => $node->getType(), 'raw' => $node->getAttribute('rawValue'),
                'value' => is_float($node->value) ? bin2hex(pack('E', $node->value)) : (string) $node->value,
                'start' => $node->getStartFilePos(), 'end' => $node->getEndFilePos() + 1,
            ];
        }
        foreach ($node->getSubNodeNames() as $key) $visit($node->$key);
    };
    $visit($nodes);
    echo json_encode(['errors' => $errors, 'literals' => $literals], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($root);
}
