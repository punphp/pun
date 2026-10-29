<?php
// php capture-parent-sentinel.php /absolute/path/to/phpstan-2.3.1.phar > expected-parent-sentinel.json
declare(strict_types=1);

$phar = isset($argv[1]) ? realpath($argv[1]) : false;
$expectedHash = '85157dba3d95d77ca603d2b7303aa78a538f9e3b56ccad07dd794c53aa96d1ae';
if ($phar === false || hash_file('sha256', $phar) !== $expectedHash) {
    throw new RuntimeException('Expected the pinned PHPStan 2.3.1 PHAR (SHA-256 ' . $expectedHash . ').');
}
require 'phar://' . $phar . '/vendor/autoload.php';

function describeParentSentinel(PHPStan\Type\Type $type): array {
    return [
        'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
        'error' => $type instanceof PHPStan\Type\ErrorType,
        'kind' => $type instanceof PHPStan\Type\NonAcceptingNeverType ? 'nonaccepting'
            : ($type instanceof PHPStan\Type\NeverType ? ($type->isExplicit() ? 'explicit' : 'implicit') : null),
        'class' => get_class($type),
    ];
}

$file = __DIR__ . '/parent-sentinel.php';
$files = [$file];
$tmp = sys_get_temp_dir() . '/pun-parent-sentinel-' . getmypid();
mkdir($tmp);
$out = ['phar_sha256' => $expectedHash, 'types' => [], 'continued' => [], 'errors' => []];
try {
    file_put_contents("$tmp/config.neon", "parameters:\n    phpVersion: 80500\n    checkThisOnly: false\n");
    foreach ([0, 5, 9] as $level) {
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$tmp/cache-$level", ['phar://' . $phar . "/conf/config.level$level.neon", "$tmp/config.neon"], $files,
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles($files);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles($files);
        if ($level === 0) {
            $source = file_get_contents($file);
            $types = [];
            $continued = [];
            $observed = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
                $file, [$file => true], new PHPStan\Rules\DirectRegistry([]),
                new PHPStan\Collectors\Registry([]),
                static function ($node, $scope) use (&$types, &$continued, $source): void {
                    if ($node instanceof PhpParser\Node\Scalar\String_ && str_starts_with($node->value, 'after-')) {
                        $continued[$node->value] = $node->getStartFilePos();
                    }
                    if (!$node instanceof PhpParser\Node\Expr
                        || !str_ends_with(substr($source, 0, $node->getStartFilePos()), '/*probe*/ ')) return;
                    $start = $node->getStartFilePos();
                    $end = $node->getEndFilePos() + 1;
                    // A fetch/call and its receiver can share a start offset;
                    // retain the complete marked expression, not the receiver.
                    if (isset($types[$start]) && $types[$start]['end'] > $end) return;
                    $type = $scope->getType($node);
                    $types[$start] = [
                        'start' => $start, 'end' => $end,
                        'expression' => substr($source, $start, $end - $start),
                        ...describeParentSentinel($type),
                        'object_class_names' => $type->getObjectClassNames(),
                        'is_object' => $type->isObject()->describe(),
                        'is_callable' => $type->isCallable()->describe(),
                        'is_iterable' => $type->isIterable()->describe(),
                        'is_cloneable' => $type->isCloneable()->describe(),
                        'can_access_properties' => $type->canAccessProperties()->describe(),
                        'can_call_methods' => $type->canCallMethods()->describe(),
                        'is_offset_accessible' => $type->isOffsetAccessible()->describe(),
                        'is_class_string' => $type->isClassString()->describe(),
                        'is_literal_string' => $type->isLiteralString()->describe(),
                        'native' => describeParentSentinel($scope->getNativeType($node)),
                    ];
                },
            );
            if ($observed->getErrors() !== []) throw new RuntimeException('Type observation failed.');
            ksort($types);
            ksort($continued);
            if (count($types) !== substr_count($source, '/*probe*/')) {
                throw new RuntimeException('Not every marked expression was observed.');
            }
            $out['types'][basename($file)] = array_values($types);
            $out['continued'][basename($file)] = $continued;
        }
        // Diagnostics use the complete level registry, not the empty observer registry.
        $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse($files, debug: true);
        $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
            ->finalize($result, true, true)->getAnalyserResult();
        if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan internal error.');
        $errors = array_map(static fn($error) => [
            basename($error->getFile()), $error->getLine(), $error->getIdentifier(),
            $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
        ], $result->getUnorderedErrors());
        usort($errors, static fn($a, $b) => json_encode($a) <=> json_encode($b));
        $out['errors'][$level] = $errors;
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
