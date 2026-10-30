<?php
// php capture-producers.php /absolute/path/to/phpstan/bootstrap.php > expected-producers.json
// Extends the independently captured PHPStan 2.3.1 arrow-never probe with die,
// forwarding, intrinsic call termination, and statement-continuation controls.
require $argv[1];

final class ArrowNeverProbeExtension implements PHPStan\Type\DynamicFunctionReturnTypeExtension
{
    public function isFunctionSupported(PHPStan\Reflection\FunctionReflection $reflection): bool
    {
        return in_array(strtolower($reflection->getName()), [
            'arrowneverprobe\\implicitbottom',
            'arrowneverprobe\\acceptingbottom',
            'arrowneverprobe\\nonacceptingbottom',
        ], true);
    }

    public function getTypeFromFunctionCall(
        PHPStan\Reflection\FunctionReflection $reflection,
        PhpParser\Node\Expr\FuncCall $call,
        PHPStan\Analyser\Scope $scope,
    ): ?PHPStan\Type\Type {
        return match (strtolower($reflection->getName())) {
            'arrowneverprobe\\implicitbottom' => new PHPStan\Type\NeverType(),
            'arrowneverprobe\\acceptingbottom' => new PHPStan\Type\NeverType(true),
            'arrowneverprobe\\nonacceptingbottom' => new PHPStan\Type\NonAcceptingNeverType(),
        };
    }
}

function describeType(PHPStan\Type\Type $type): array
{
    return [
        'class' => get_class($type),
        'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
        'kind' => $type instanceof PHPStan\Type\NonAcceptingNeverType ? 'nonaccepting'
            : ($type instanceof PHPStan\Type\NeverType ? ($type->isExplicit() ? 'explicit-accepting' : 'implicit') : null),
        'explicit' => $type instanceof PHPStan\Type\NeverType ? $type->isExplicit() : null,
        'accepts_int' => $type->accepts(new PHPStan\Type\IntegerType(), true)->yes(),
    ];
}

$file = __DIR__ . '/producers.php';
$tmp = sys_get_temp_dir() . '/pun-never-producers-' . getmypid();
mkdir($tmp);
try {
    file_put_contents("$tmp/config.neon", "parameters:\n    level: 7\n    phpVersion: 80500\nservices:\n    -\n        class: ArrowNeverProbeExtension\n        tags:\n            - phpstan.broker.dynamicFunctionReturnTypeExtension\n");
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$tmp/cache", ['phar://' . dirname($argv[1]) . '/phpstan.phar/conf/config.level7.neon', "$tmp/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $out = [];
    $continued = [];
    $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function (PhpParser\Node $node, PHPStan\Analyser\Scope $scope) use (&$out, &$continued, $source): void {
            if ($node instanceof PhpParser\Node\Scalar\String_ && str_starts_with($node->value, 'after-')) {
                $continued[$node->value] = $node->getStartFilePos();
            }
            if (!$node instanceof PhpParser\Node\Expr\Exit_ && !$node instanceof PhpParser\Node\Expr\Throw_
                && !$node instanceof PhpParser\Node\Expr\ArrowFunction && !$node instanceof PhpParser\Node\Expr\Closure
                && !$node instanceof PhpParser\Node\Expr\FuncCall && !$node instanceof PhpParser\Node\Expr\StaticCall) {
                return;
            }
            $type = $scope->getType($node);
            $native = $scope->getNativeType($node);
            $record = [
                'line' => $node->getStartLine(),
                'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                'getType' => describeType($type),
                'getNativeType' => describeType($native),
            ];
            if ($type instanceof PHPStan\Type\ClosureType) {
                $record['callable_return'] = describeType($type->getReturnType());
                $record['native_callable_return'] = describeType($native->getReturnType());
            }
            if ($node instanceof PhpParser\Node\Expr\FuncCall || $node instanceof PhpParser\Node\Expr\StaticCall) {
                $record['terminates'] = $scope->getCurrentExpressionResultStorage()->findExpressionResult($node)?->isAlwaysTerminating();
            }
            $out[(string)$node->getStartFilePos()] = $record;
        },
    );
    ksort($out, SORT_NUMERIC);
    ksort($continued);
    $controls = [];
    foreach ([
        'implicit' => new PHPStan\Type\NeverType(),
        'explicit-accepting' => new PHPStan\Type\NeverType(true),
        'nonaccepting' => new PHPStan\Type\NonAcceptingNeverType(),
    ] as $name => $type) {
        $controls[$name] = describeType($type);
    }
    echo json_encode([
        'phar_sha256' => hash_file('sha256', dirname($argv[1]) . '/phpstan.phar'),
        'controls' => $controls,
        'expressions' => $out,
        'continued' => $continued,
    ], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
