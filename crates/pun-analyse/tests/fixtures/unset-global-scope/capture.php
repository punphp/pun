<?php

// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);
use PhpParser\Node\Expr\Variable;
use PHPStan\Analyser\FileAnalyser;
use PHPStan\Analyser\NodeScopeResolver;
use PHPStan\Collectors\Registry;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Rules\DirectRegistry;
use PHPStan\Type\VerbosityLevel;

$phar = realpath($argv[1]);
require 'phar://'.$phar.'/vendor/autoload.php';
$tmp = sys_get_temp_dir().'/pun-unset-global-scope-oracle-'.getmypid();
mkdir($tmp);
try {
    $fixtures = [];
    foreach (['global.php', 'values.php', 'branches.php'] as $name) {
        $file = __DIR__.'/'.$name;
        $diagnostics = [];
        foreach ([1, 4] as $level) {
            $config = $tmp.'/level'.$level.'.neon';
            file_put_contents($config, "parameters:\n    level: $level\n    phpVersion: 80500\n    checkMaybeUndefinedVariables: true\n    tmpDir: $tmp/cache$level\n");
            $command = escapeshellarg(PHP_BINARY).' '.escapeshellarg($phar)
                .' analyse --no-progress --error-format=json -c '.escapeshellarg($config).' '.escapeshellarg($file);
            exec($command, $output, $status);
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
        $source = file_get_contents($file);
        $marked = [];
        $offset = 0;
        foreach (explode("\n", $source) as $line => $text) {
            if (preg_match('/\/\/ observe(?:-native)?$/', $text)) {
                preg_match('/(\$\w+)(?:\);|;)/', $text, $match, PREG_OFFSET_CAPTURE);
                $start = $offset + $match[1][1];
                $marked[$start] = [$start + strlen($match[1][0]), $line + 1, str_ends_with($text, '// observe-native')];
            }
            $offset += strlen($text) + 1;
        }
        $container = (new ContainerFactory(__DIR__))->create(
            $tmp.'/observations', ["phar://$phar/conf/config.level0.neon"], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $container->getByType(NodeScopeResolver::class)->setAnalysedFiles([$file]);
        $found = [];
        $container->getByType(FileAnalyser::class)->analyseFile(
            $file, [$file => true], new DirectRegistry([]), new Registry([]),
            static function ($node, $scope) use (&$found, $marked): void {
                if (! $node instanceof Variable || ! isset($marked[$node->getStartFilePos()])) {
                    return;
                }
                [$end, $line, $nativeSupported] = $marked[$node->getStartFilePos()];
                $certainty = $scope->hasVariableType($node->name);
                $found[] = ['start' => $node->getStartFilePos(), 'end' => $end, 'line' => $line,
                    'native_supported' => $nativeSupported,
                    'certainty' => $certainty->yes() ? 'defined' : ($certainty->no() ? 'absent' : 'possible'),
                    'selected' => $scope->getType($node)->describe(VerbosityLevel::precise()),
                    'native' => $scope->getNativeType($node)->describe(VerbosityLevel::precise())];
            },
        );
        usort($found, static fn ($a, $b) => $a['start'] <=> $b['start']);
        if (count($found) !== count($marked)) {
            throw new RuntimeException('missing observations');
        }
        $fixtures[$name] = ['source_sha256' => hash('sha256', $source), 'diagnostics' => $diagnostics, 'observations' => $found];
    }
    echo json_encode(['phar_sha256' => hash_file('sha256', $phar), 'fixtures' => $fixtures],
        JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
