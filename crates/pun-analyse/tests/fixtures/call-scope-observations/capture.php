<?php
// php capture.php /absolute/path/to/phpstan-2.3.1.phar [fixture.php] [--dependencies] [--function-events] [--method-events] > expected.json
$phar = $argv[1];
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-call-observations-' . getmypid();
mkdir($root);
$file = __DIR__ . '/' . ($argv[2] ?? 'matrix.php');
$dependencies = in_array('--dependencies', $argv, true);
$reflections = in_array('--reflections', $argv, true);
$functionEvents = in_array('--function-events', $argv, true) ? [] : null;
$methodEvents = in_array('--method-events', $argv, true) ? [] : null;
try {
    $config = "parameters:\n    phpVersion: 80500\n    checkThisOnly: false\n    checkFunctionArgumentTypes: true\n";
    if (($argv[2] ?? '') === 'providers.php') {
        require __DIR__ . '/provider-extensions.php';
        $config .= "services:\n    -\n        class: PricingFunctionExtension\n        tags: [phpstan.broker.dynamicFunctionReturnTypeExtension]\n    -\n        class: PricingMethodExtension\n        tags: [phpstan.broker.dynamicMethodReturnTypeExtension]\n    -\n        class: PricingStaticExtension\n        tags: [phpstan.broker.dynamicStaticMethodReturnTypeExtension]\n";
    }
    file_put_contents("$root/config.neon", $config);
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$root/cache", ['phar://' . $phar . '/conf/config.level0.neon', "$root/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $text = static fn($expr) => substr($source, $expr->getStartFilePos(), $expr->getEndFilePos() - $expr->getStartFilePos() + 1);
    $out = [];
    $result = $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$out, &$functionEvents, &$methodEvents, $text, $source, $dependencies, $reflections): void {
            if ($functionEvents !== null && $node instanceof PhpParser\Node\Expr\FuncCall) {
                $position = $node->getStartFilePos();
                $functionEvents[$position] = ($functionEvents[$position] ?? 0) + 1;
            }
            if (!$node instanceof PhpParser\Node\Expr\MethodCall) return;
            if ($methodEvents !== null) {
                $position = $node->getStartFilePos();
                $methodEvents[$position] = ($methodEvents[$position] ?? 0) + 1;
            }
            $last = $node->getEndFilePos() + 1;
            $marker = substr($source, $last, 14);
            if (!str_starts_with($marker, '; // observe') && !str_starts_with($marker, '/* observe */')) return;
            $reads = [$node->var, ...array_map(static fn($arg) => $arg->value, $node->args)];
            if ($dependencies) {
                $reads = (new PhpParser\NodeFinder())->findInstanceOf($reads, PhpParser\Node\Expr::class);
            }
            $observe = static function ($s) use ($reads, $text): array {
                return array_map(static fn($expr) => [
                    'start' => $expr->getStartFilePos(), 'end' => $expr->getEndFilePos() + 1,
                    'expression' => $text($expr),
                    'type' => $s->getType($expr)->describe(PHPStan\Type\VerbosityLevel::precise()),
                ], $reads);
            };
            $names = [];
            if ($node->name instanceof PhpParser\Node\Identifier) {
                $names[] = ['name' => $node->name->name, 'filtered' => false, 'types' => $observe($scope)];
            } else {
                foreach ($scope->getType($node->name)->getConstantStrings() as $name) {
                    $filtered = $scope->filterByTruthyValue(new PhpParser\Node\Expr\BinaryOp\Identical($node->name, new PhpParser\Node\Scalar\String_($name->getValue())));
                    $names[] = ['name' => $name->getValue(), 'filtered' => true, 'types' => $observe($filtered)];
                }
            }
            if ($reflections) {
                foreach ($names as &$name) {
                    $method = $scope->getMethodReflection($scope->getType($node->var), $name['name']);
                    if ($method === null) throw new RuntimeException('Missing method reflection');
                    $effects = $method->hasSideEffects();
                    $name['reflection'] = [
                        'name' => $method->getName(),
                        'class' => $method->getDeclaringClass()->getName(),
                        'sideEffects' => $effects->yes() ? 'yes' : ($effects->no() ? 'no' : 'maybe'),
                    ];
                }
                unset($name);
            }
            usort($names, static fn($a, $b) => $a['name'] <=> $b['name']);
            $out[$node->getStartFilePos()] = ['line' => $node->getStartLine(), 'expression' => $text($node), 'names' => $names];
        },
    );
    if ($result->getErrors() !== [] && !in_array('--allow-parse-errors', $argv, true)) throw new RuntimeException('Observation capture failed');
    ksort($out, SORT_NUMERIC);
    $result = $container->getByType(PHPStan\Analyser\Analyser::class)->analyse([$file], debug: true);
    if ($result->getInternalErrors() !== []) throw new RuntimeException('PHPStan reported an internal error');
    $errors = array_map(static fn($error) => [
        $error->getLine(), $error->getIdentifier(), $error->getMessage(), $error->getTip(), $error->canBeIgnored(),
    ], $result->getUnorderedErrors());
    sort($errors);
    $capture = ['calls' => $out, 'errors' => $errors];
    if ($functionEvents !== null) {
        ksort($functionEvents, SORT_NUMERIC);
        $capture['function_events'] = $functionEvents;
    }
    if ($methodEvents !== null) {
        ksort($methodEvents, SORT_NUMERIC);
        $capture['method_events'] = $methodEvents;
    }
    echo json_encode($capture, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
