<?php

// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);

use PhpParser\Node\Expr;
use PhpParser\Node\ArrayItem;
use PHPStan\Analyser\FileAnalyser;
use PHPStan\Analyser\NodeScopeResolver;
use PHPStan\Collectors\Registry;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Rules\DirectRegistry;
use PHPStan\Type\VerbosityLevel;

$phar = realpath($argv[1]);
require 'phar://'.$phar.'/vendor/autoload.php';
$tmp = sys_get_temp_dir().'/pun-reference-array-read-values-'.getmypid();
mkdir($tmp);
try {
    $file = $argv[2] ?? __DIR__.'/values.php';
    $source = file_get_contents($file);
    $diagnostics = [];
    foreach ([1, 5] as $level) {
        $config = $tmp.'/level'.$level.'.neon';
        file_put_contents($config, "parameters:\n    level: $level\n    phpVersion: 80500\n    checkMaybeUndefinedVariables: true\n    tmpDir: $tmp/cache$level\n");
        exec(escapeshellarg(PHP_BINARY).' '.escapeshellarg($phar)
            .' analyse --no-progress --error-format=json -c '.escapeshellarg($config).' '.escapeshellarg($file), $output, $status);
        if ($status !== 1) {
            throw new RuntimeException('oracle exit '.$status);
        }
        $result = json_decode(implode("\n", $output), true, flags: JSON_THROW_ON_ERROR);
        $output = [];
        if ($result['errors'] !== []) {
            throw new RuntimeException('oracle run errors');
        }
        $diagnostics[$level] = array_map(static fn ($e) => [basename($file), $e['line'],
            $e['identifier'], $e['message'], $e['tip'] ?? null, $e['ignorable']], $result['files'][$file]['messages']);
    }
    $marks = [];
    $offset = 0;
    foreach (explode("\n", $source) as $line => $text) {
        if (str_ends_with($text, '// observe-array')) {
            preg_match('/= (\[|array\()/', $text, $match, PREG_OFFSET_CAPTURE);
            $marks[$offset + $match[1][1]] = false;
        } elseif (preg_match('/\/\/ observe(?:-native)?$/', $text)) {
            preg_match('/(\$\w+)(?:\);|;)/', $text, $match, PREG_OFFSET_CAPTURE);
            $marks[$offset + $match[1][1]] = str_ends_with($text, '// observe-native');
        }
        $offset += strlen($text) + 1;
    }
    $container = (new ContainerFactory(__DIR__))->create(
        $tmp.'/observations', ["phar://$phar/conf/config.level0.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $observations = [];
    $references = [];
    $container->getByType(FileAnalyser::class)->analyseFile(
        $file, [$file => true], new DirectRegistry([]), new Registry([]),
        static function ($node, $scope) use (&$observations, &$references, $marks): void {
            if ($node instanceof ArrayItem && $node->byRef) {
                $references[$node->value->getStartFilePos()] = $node->value->getEndFilePos() + 1;
                $node = $node->value;
                $nativeRequired = false;
            } elseif ($node instanceof Expr && array_key_exists($node->getStartFilePos(), $marks)) {
                $nativeRequired = $marks[$node->getStartFilePos()];
            } else {
                return;
            }
            $start = $node->getStartFilePos();
            if ($node instanceof Expr\Variable && is_string($node->name)) {
                $certainty = $scope->hasVariableType($node->name);
                $certainty = $certainty->yes() ? 'defined' : ($certainty->no() ? 'absent' : 'possible');
            } else {
                $certainty = null;
            }
            $observations[$start] = ['start' => $start, 'end' => $node->getEndFilePos() + 1,
                'line' => $node->getStartLine(), 'certainty' => $certainty, 'native_required' => $nativeRequired,
                'selected' => $scope->getType($node)->describe(VerbosityLevel::precise()),
                'native' => $scope->getNativeType($node)->describe(VerbosityLevel::precise())];
        },
    );
    ksort($observations);
    ksort($references);
    if (count($marks) + count($references) !== count($observations)) {
        throw new RuntimeException('missing observations');
    }
    echo json_encode(['phar_sha256' => hash_file('sha256', $phar), 'source_sha256' => hash('sha256', $source),
        'diagnostics' => $diagnostics, 'observations' => array_values($observations), 'reference_operands' => $references],
        JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
