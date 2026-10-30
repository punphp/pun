<?php

// PHPStan 2.3.1 cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$file = __DIR__ . '/calls.php';
$source = file_get_contents($file);
$ranges = [];
$offset = 0;
foreach (explode("\n", $source) as $line => $text) {
    if (preg_match('/^(\s*)(.+); \/\/ (observe|unavailable)$/', $text, $match)) {
        $start = $offset + strlen($match[1]);
        $ranges[$start] = [$start + strlen($match[2]), $line + 1, $match[2], $match[3] === 'observe'];
    }
    $offset += strlen($text) + 1;
}
$tmp = sys_get_temp_dir() . '/pun-native-call-continuation-' . getmypid();
mkdir($tmp);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $tmp, ["phar://$phar/conf/config.level0.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $found = [];
    $parenthesized = [];
    $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$found, &$parenthesized, $ranges): void {
            if ($node instanceof PhpParser\Node\Expr && isset($ranges[$node->getStartFilePos() - 1])) {
                $parenthesized[$node->getStartFilePos() - 1] = $scope->getNativeType($node)
                    ->describe(PHPStan\Type\VerbosityLevel::precise());
            }
            if (!$node instanceof PhpParser\Node\Expr || !isset($ranges[$node->getStartFilePos()])) { return; }
            [$end, $line, $expression, $supported] = $ranges[$node->getStartFilePos()];
            if ($node->getEndFilePos() + 1 !== $end) { return; }
            $found[] = ['file' => 'calls.php', 'start' => $node->getStartFilePos(), 'end' => $end,
                'line' => $line, 'expression' => $expression, 'supported' => $supported,
                'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise())];
        },
    );
    // PhpParser discards parentheses; retain that observable wrapper's original range.
    foreach ($ranges as $start => [$end, $line, $expression, $supported]) {
        if ($expression[0] !== '(') { continue; }
        $found[] = ['file' => 'calls.php', 'start' => $start, 'end' => $end, 'line' => $line,
            'expression' => $expression, 'supported' => $supported,
            'native' => $parenthesized[$start]];
    }
    usort($found, static fn ($a, $b) => $a['start'] <=> $b['start']);
    if (count($found) !== count($ranges)) { throw new RuntimeException('missing positions'); }
    echo json_encode(['phar_sha256' => hash_file('sha256', $phar),
        'source_sha256' => hash('sha256', $source), 'observations' => $found],
        JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
