<?php

// php capture.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
declare(strict_types=1);
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';

// Raw Mago arrays have canonical key maps. Consumer diagnostics retain observable source order.
function value(PHPStan\Type\Type $type): mixed
{
    if ($type instanceof PHPStan\Type\Constant\ConstantArrayType) {
        $items = [];
        foreach ($type->getKeyTypes() as $index => $key) {
            $items[] = [$key->getValue(), value($type->getValueTypes()[$index])];
        }
        usort($items, static fn ($a, $b) => json_encode($a[0]) <=> json_encode($b[0]));
        return ['shape' => $items];
    }
    if ($type instanceof PHPStan\Type\UnionType) {
        $items = array_map(value(...), $type->getTypes());
        usort($items, static fn ($a, $b) => json_encode($a) <=> json_encode($b));
        return ['union' => $items];
    }
    if ($type instanceof PHPStan\Type\Constant\ConstantIntegerType
        || $type instanceof PHPStan\Type\Constant\ConstantStringType
        || $type instanceof PHPStan\Type\Constant\ConstantBooleanType) {
        return $type->getValue();
    }
    return match (true) {
        $type instanceof PHPStan\Type\NullType => null,
        $type instanceof PHPStan\Type\ErrorType => '*ERROR*',
        $type instanceof PHPStan\Type\MixedType => 'mixed',
        $type instanceof PHPStan\Type\IntegerType => 'int',
        $type instanceof PHPStan\Type\StringType => 'string',
        $type instanceof PHPStan\Type\ArrayType => 'array',
        $type->isArray()->yes() => 'array',
        $type instanceof PHPStan\Type\ObjectType => ['object' => $type->getClassName()],
        default => throw new RuntimeException('Unsupported capture type ' . get_class($type)),
    };
}

function pair($scope, $expression, string $source): array
{
    $start = $expression->getStartFilePos();
    $end = $expression->getEndFilePos() + 1;
    $full = $scope->getType($expression);
    $native = $scope->getNativeType($expression);
    return ['start' => $start, 'end' => $end, 'expression' => substr($source, $start, $end - $start),
        'full' => $full->describe(PHPStan\Type\VerbosityLevel::precise()),
        'native' => $native->describe(PHPStan\Type\VerbosityLevel::precise()),
        'full_value' => value($full), 'native_value' => value($native)];
}

$tmp = sys_get_temp_dir() . '/pun-conversion-dependencies-' . getmypid();
mkdir($tmp);
$out = ['version' => '2.3.1', 'phar_sha256' => hash_file('sha256', $phar), 'sources' => [],
    'observations' => [], 'calls' => [], 'diagnostics' => []];
try {
    foreach (['native', 'selected'] as $name) {
        $file = __DIR__ . "/$name-controls.php";
        $source = file_get_contents($file);
        $out['sources'][$name] = hash('sha256', $source);
        $ranges = [];
        $offset = 0;
        foreach (explode("\n", $source) as $line => $text) {
            if (preg_match('/^(\s*)(.+); \/\/ (observe|unavailable):([\w-]+)$/', $text, $match)) {
                $start = $offset + strlen($match[1]);
                $ranges[$start] = ['end' => $start + strlen($match[2]), 'line' => $line + 1,
                    'available' => $match[3] === 'observe', 'label' => $match[4]];
            }
            $offset += strlen($text) + 1;
        }
        $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
            "$tmp/observations-$name", ["phar://$phar/conf/config.level0.neon"], [$file],
        );
        $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
        $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
        $observations = [];
        $calls = [];
        $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
            $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
            static function ($node, $scope) use (&$observations, &$calls, $source, $ranges): void {
                $start = $node->getStartFilePos();
                if ($node instanceof PhpParser\Node\Expr && isset($ranges[$start])
                    && $node->getEndFilePos() + 1 === $ranges[$start]['end']) {
                    $observations[] = $ranges[$start] + pair($scope, $node, $source);
                }
                if (!$node instanceof PhpParser\Node\Expr\MethodCall
                    || substr($source, $node->getEndFilePos() + 1, 13) !== '; // selected') {
                    return;
                }
                $reads = (new PhpParser\NodeFinder())->findInstanceOf(
                    [$node->var, ...array_map(static fn ($arg) => $arg->value, $node->args)], PhpParser\Node\Expr::class,
                );
                $names = [];
                $observe = static fn ($selected) => array_map(static fn ($expression) => pair($selected, $expression, $source), $reads);
                if ($node->name instanceof PhpParser\Node\Identifier) {
                    $names[] = ['name' => $node->name->name, 'filtered' => false, 'types' => $observe($scope)];
                } else {
                    foreach ($scope->getType($node->name)->getConstantStrings() as $method) {
                        $selected = $scope->filterByTruthyValue(new PhpParser\Node\Expr\BinaryOp\Identical(
                            $node->name, new PhpParser\Node\Scalar\String_($method->getValue()),
                        ));
                        $names[] = ['name' => $method->getValue(), 'filtered' => true, 'types' => $observe($selected)];
                    }
                }
                usort($names, static fn ($a, $b) => $a['name'] <=> $b['name']);
                $calls[] = ['start' => $start, 'end' => $node->getEndFilePos() + 1, 'names' => $names];
            },
        );
        if ($result->getErrors() !== [] || count($observations) !== count($ranges)) {
            throw new RuntimeException('Incomplete public observations');
        }
        $out['observations'][$name] = $observations;
        $out['calls'][$name] = $calls;
        foreach ($name === 'native' ? [0, 4, 5] : [4, 5] as $level) {
            foreach ($name === 'native' ? [true, false] : [true] as $certain) {
                $case = "$name/$level/" . (int) $certain;
                $config = "$tmp/" . str_replace('/', '-', $case) . '.neon';
                file_put_contents($config, "parameters:\n    phpVersion: 80500\n    treatPhpDocTypesAsCertain: "
                    . ($certain ? 'true' : 'false') . "\n");
                $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
                    "$config-cache", ["phar://$phar/conf/config.level$level.neon", $config], [$file],
                );
                $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
                $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
                $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
                $result = $container->getByType(PHPStan\Analyser\AnalyserResultFinalizer::class)
                    ->finalize($result, true, true)->getAnalyserResult();
                if ($result->getInternalErrors() !== []) {
                    throw new RuntimeException('PHPStan internal error');
                }
                $errors = array_map(static fn ($error) => [basename($error->getFile()), $error->getLine(),
                    $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored()],
                    $result->getUnorderedErrors());
                usort($errors, static fn ($a, $b) => json_encode($a) <=> json_encode($b));
                $out['diagnostics'][$case] = $errors;
            }
        }
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp,
        FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) { $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname()); }
    rmdir($tmp);
}
