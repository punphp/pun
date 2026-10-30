<?php

// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php capture.php /absolute/path/to/phpstan.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$temporary = sys_get_temp_dir() . '/pun-parameter-native-clamp-' . getmypid();
mkdir($temporary);
try {
    $out = ['phar_sha256' => hash_file('sha256', $phar), 'sources' => [], 'observations' => [], 'channels' => [], 'errors' => []];
    foreach (['types.php', 'channels.php'] as $name) {
        $file = __DIR__ . '/' . $name;
        $source = file_get_contents($file);
        $out['sources'][$name] = hash('sha256', $source);
        foreach ([0, 4, 9] as $level) {
            file_put_contents("$temporary/config.neon", "parameters:\n    phpVersion: 80500\n");
            $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                "$temporary/$name-$level", ["phar://$phar/conf/config.level$level.neon", "$temporary/config.neon"], [$file],
            );
            $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
            $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
            $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
            if ($result->getInternalErrors() !== []) throw new RuntimeException('Oracle internal error');
            $errors = array_map(static fn ($error) => [
                $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
            ], $result->getUnorderedErrors());
            sort($errors);
            $out['errors']["$name-$level"] = $errors;
            if ($level !== 0) continue;
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
            $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
                static function ($node, $scope) use (&$out, &$seen, $ranges, $source, $name): void {
                    if (!$node instanceof PhpParser\Node\Expr) return;
                    if ($node instanceof PhpParser\Node\Expr\PropertyFetch && $node->getAttribute('virtualNullsafePropertyFetch')) return;
                    $start = $node->getStartFilePos();
                    $end = $node->getEndFilePos() + 1;
                    if ($name === 'channels.php' && $node instanceof PhpParser\Node\Expr\Variable
                        && (($node->name === 'value' && substr($source, $end, 1) === ';')
                            || ($node->name === 'callback' && substr($source, $end, 3) === '(1)'))) {
                        $full = $scope->getType($node);
                        $channel = ['start' => $start, 'end' => $end,
                            'full' => $full->describe(PHPStan\Type\VerbosityLevel::precise()),
                            'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise())];
                        if ($full instanceof PHPStan\Type\Generic\TemplateType) {
                            $channel['template'] = ['name' => $full->getName(),
                                'bound' => $full->getBound()->describe(PHPStan\Type\VerbosityLevel::precise())];
                        } elseif ($full instanceof PHPStan\Type\ClosureType) {
                            $channel['callable'] = ['parameters' => array_map(
                                static fn ($parameter) => $parameter->getType()->describe(PHPStan\Type\VerbosityLevel::precise()),
                                $full->getParameters()),
                                'return' => $full->getReturnType()->describe(PHPStan\Type\VerbosityLevel::precise())];
                        }
                        $out['channels'][] = $channel;
                    }
                    if (!isset($ranges[$start]) || $ranges[$start][0] !== $end) return;
                    $seen[$start] = true;
                    $out['observations'][] = ['file' => $name, 'start' => $start, 'end' => $end,
                        'line' => $node->getStartLine(), 'expression' => substr($source, $start, $end - $start),
                        'supported' => $ranges[$start][1],
                        'full' => $scope->getType($node)->describe(PHPStan\Type\VerbosityLevel::precise()),
                        'native' => $scope->getNativeType($node)->describe(PHPStan\Type\VerbosityLevel::precise())];
                },
            );
            if (count($seen) !== count($ranges)) throw new RuntimeException('Missing marked observations');
        }
    }
    usort($out['observations'], static fn ($a, $b) => $a['start'] <=> $b['start']);
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($temporary, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($temporary);
}
