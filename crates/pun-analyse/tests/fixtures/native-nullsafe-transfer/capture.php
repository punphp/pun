<?php

// PHPStan 2.3.1 cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$tmp = sys_get_temp_dir() . '/pun-native-nullsafe-transfer-' . getmypid();
mkdir($tmp);
try {
    $files = array_map(static fn ($name) => __DIR__ . '/' . $name,
        ['consumer.php', 'properties.php', 'receiver-boundary.php']);
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $tmp, ["phar://$phar/conf/config.level1.neon"], $files,
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles($files);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
    $found = [];
    $hashes = [];
    foreach ($files as $file) {
        $source = file_get_contents($file);
        $name = basename($file);
        $hashes[$name] = hash('sha256', $source);
        $ranges = [];
        $offset = 0;
        foreach (explode("\n", $source) as $line => $text) {
            if (preg_match('/^(\s*)(.+); \/\/ (observe|unavailable)$/', $text, $match)) {
                $start = $offset + strlen($match[1]);
                $ranges[$start] = [$start + strlen($match[2]), $match[3] === 'observe'];
            }
            $offset += strlen($text) + 1;
        }
        $seen = [];
        $carried = [];
        $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
            $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
            static function ($node, $scope) use (&$found, &$seen, &$carried, $ranges, $source, $name): void {
                if ($name === 'consumer.php' && $node instanceof PHPStan\Node\NullsafePropertyFetchExpressionNode) {
                    $receiver = $node->getOriginalNode()->var;
                    $found[] = ['file' => $name, 'start' => $receiver->getStartFilePos(),
                        'end' => $receiver->getEndFilePos() + 1, 'line' => $receiver->getStartLine(),
                        'expression' => '$box', 'supported' => true, 'position' => 'incoming receiver',
                        'full' => $node->getCalledOnType()->describe(PHPStan\Type\VerbosityLevel::precise()),
                        'native' => $node->getCalledOnNativeType()->describe(PHPStan\Type\VerbosityLevel::precise())];
                }
                $results = match (true) {
                    $node instanceof PHPStan\Node\IssetExpressionNode => $node->getVarResults(),
                    $node instanceof PHPStan\Node\CoalesceExpressionNode => [$node->getSubjectResult()],
                    default => [],
                };
                foreach ($results as $result) {
                    $expression = $result->getExpr();
                    if ($expression instanceof PhpParser\Node\Expr\NullsafePropertyFetch) {
                        $carried[$expression->getStartFilePos() . ':' . ($expression->getEndFilePos() + 1)] = [
                            $result->getType()->describe(PHPStan\Type\VerbosityLevel::precise()),
                            $result->getNativeType()->describe(PHPStan\Type\VerbosityLevel::precise())];
                    }
                }
                if (!$node instanceof PhpParser\Node\Expr) { return; }
                if ($node instanceof PhpParser\Node\Expr\PropertyFetch && $node->getAttribute('virtualNullsafePropertyFetch')) { return; }
                $start = $node->getStartFilePos();
                $end = $node->getEndFilePos() + 1;
                if ($name === 'consumer.php') {
                    if (!$node instanceof PhpParser\Node\Expr\NullsafePropertyFetch
                        && !$node instanceof PhpParser\Node\Expr\Isset_
                        && !$node instanceof PhpParser\Node\Expr\BinaryOp\Coalesce) { return; }
                    $supported = true;
                } else {
                    if (!isset($ranges[$start]) || $ranges[$start][0] !== $end) { return; }
                    $supported = $ranges[$start][1];
                    $seen[$start] = true;
                }
                $found[] = ['file' => $name, 'start' => $start, 'end' => $end,
                    'line' => $node->getStartLine(), 'expression' => substr($source, $start, $end - $start),
                    'supported' => $supported, 'position' => 'expression',
                    'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                    'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise())];
            },
        );
        foreach ($found as &$record) {
            $key = $record['start'] . ':' . $record['end'];
            if ($record['file'] === $name && isset($carried[$key])) {
                [$record['full'], $record['native']] = $carried[$key];
                $record['position'] = 'carried operand';
            }
        }
        unset($record);
        if ($name !== 'consumer.php' && count($seen) !== count($ranges)) {
            throw new RuntimeException('missing marked observations');
        }
    }
    usort($found, static fn ($a, $b) => [$a['file'], $a['start'], -$a['end']] <=> [$b['file'], $b['start'], -$b['end']]);
    echo json_encode(['phar_sha256' => hash_file('sha256', $phar),
        'sources' => $hashes, 'observations' => $found],
        JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
