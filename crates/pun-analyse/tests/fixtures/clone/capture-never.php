<?php
// php capture-never.php /absolute/path/to/phpstan/bootstrap.php > expected-never.json
require $argv[1];

final class CloneNeverReturnExtension implements PHPStan\Type\DynamicFunctionReturnTypeExtension
{
    public function isFunctionSupported(PHPStan\Reflection\FunctionReflection $reflection): bool
    {
        return in_array(strtolower($reflection->getName()), [
            'clonenevercontrols\\implicitbottom',
            'clonenevercontrols\\explicitbottom',
            'clonenevercontrols\\nonacceptingbottom',
        ], true);
    }

    public function getTypeFromFunctionCall(
        PHPStan\Reflection\FunctionReflection $reflection,
        PhpParser\Node\Expr\FuncCall $call,
        PHPStan\Analyser\Scope $scope,
    ): ?PHPStan\Type\Type {
        return match (strtolower($reflection->getName())) {
            'clonenevercontrols\\implicitbottom' => new PHPStan\Type\NeverType(),
            'clonenevercontrols\\explicitbottom' => new PHPStan\Type\NeverType(true),
            'clonenevercontrols\\nonacceptingbottom' => new PHPStan\Type\NonAcceptingNeverType(),
        };
    }
}

function describeCloneNever(PHPStan\Type\Type $type): array
{
    return [
        'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
        'kind' => $type instanceof PHPStan\Type\NonAcceptingNeverType ? 'nonaccepting'
            : ($type instanceof PHPStan\Type\NeverType ? ($type->isExplicit() ? 'explicit' : 'implicit') : null),
        'explicit' => $type instanceof PHPStan\Type\NeverType && $type->isExplicit(),
    ];
}

$file = __DIR__ . '/never.php';
$tmp = sys_get_temp_dir() . '/pun-clone-never-' . getmypid();
mkdir($tmp);
try {
    file_put_contents("$tmp/config.neon", "parameters:\n    level: 7\n    phpVersion: 80500\nservices:\n    -\n        class: CloneNeverReturnExtension\n        tags:\n            - phpstan.broker.dynamicFunctionReturnTypeExtension\n");
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
            if (!$node instanceof PhpParser\Node\Expr\Clone_) {
                return;
            }
            $operand = $scope->getType($node->expr);
            $out[(string)$node->getStartFilePos()] = [
                'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                ...describeCloneNever($scope->getType($node)),
                'operand' => [
                    'offset' => $node->expr->getStartFilePos(),
                    'length' => $node->expr->getEndFilePos() - $node->expr->getStartFilePos() + 1,
                    ...describeCloneNever($operand),
                    'terminates' => $scope->getCurrentExpressionResultStorage()->findExpressionResult($node->expr)?->isAlwaysTerminating(),
                ],
            ];
        },
    );
    ksort($out, SORT_NUMERIC);
    ksort($continued);
    $bottoms = [];
    foreach ([
        'implicit' => new PHPStan\Type\NeverType(),
        'explicit' => new PHPStan\Type\NeverType(true),
        'nonaccepting' => new PHPStan\Type\NonAcceptingNeverType(),
    ] as $name => $type) {
        $bottoms[$name] = [
            'intersection' => describeCloneNever(PHPStan\Type\TypeCombinator::intersect($type, new PHPStan\Type\ObjectWithoutClassType())),
            'clone' => describeCloneNever(PHPStan\Analyser\ExprHandler\CloneHandler::resolveCloneType($type)),
        ];
    }
    echo json_encode(['bottoms' => $bottoms, 'expressions' => $out, 'continued' => $continued], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($tmp);
}
