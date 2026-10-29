<?php
// php capture-providers.php /absolute/path/to/phpstan-2.3.1.phar > expected-providers.json
declare(strict_types=1);

require 'phar://' . realpath($argv[1]) . '/vendor/autoload.php';

final class FailedInstantiationReturn implements PHPStan\Type\DynamicStaticMethodReturnTypeExtension {
    public function __construct(private string $class, private string $mode) {}
    public function getClass(): string { return 'FailedInstantiationProviders\\' . $this->class; }
    public function isStaticMethodSupported(PHPStan\Reflection\MethodReflection $method): bool {
        return $method->getName() === '__construct';
    }
    public function getTypeFromStaticMethodCall(PHPStan\Reflection\MethodReflection $method, PhpParser\Node\Expr\StaticCall $call, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        return match ($this->mode) {
            'argument' => $scope->getType($call->getArgs()[0]->value),
            'ordinary' => new PHPStan\Type\Constant\ConstantIntegerType(17),
            'implicit' => new PHPStan\Type\NeverType(),
            'explicit' => new PHPStan\Type\NeverType(true),
            'nonaccepting' => new PHPStan\Type\NonAcceptingNeverType(),
        };
    }
}

$file = __DIR__ . '/providers.php';
$tmp = sys_get_temp_dir() . '/pun-failed-instantiation-provider-' . getmypid();
mkdir($tmp);
try {
    $config = "parameters:\n    phpVersion: 80500\nservices:\n";
    foreach (['Provided' => 'argument', 'IllegalProvided' => 'argument', 'EmptyProvided' => 'argument', 'OverriddenNever' => 'ordinary',
        'ImplicitBottom' => 'implicit', 'ExplicitBottom' => 'explicit', 'NonacceptingBottom' => 'nonaccepting',
        'NoArgumentProvided' => 'ordinary', 'InheritedProvided' => 'argument'] as $class => $mode) {
        $config .= "    -\n        class: FailedInstantiationReturn\n        arguments: [$class, $mode]\n        tags: [phpstan.broker.dynamicStaticMethodReturnTypeExtension]\n";
    }
    file_put_contents("$tmp/config.neon", $config);
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        "$tmp/cache", ['phar://' . realpath($argv[1]) . '/conf/config.level0.neon', "$tmp/config.neon"], [$file],
    );
    $container->getService('pathRoutingParser')->setAnalysedFiles([$file]);
    $container->getByType(PHPStan\Analyser\NodeScopeResolver::class)->setAnalysedFiles([$file]);
    $source = file_get_contents($file);
    $types = [];
    $continued = [];
    $container->getByType(PHPStan\Analyser\FileAnalyser::class)->analyseFile(
        $file, [$file => true], new PHPStan\Rules\DirectRegistry([]), new PHPStan\Collectors\Registry([]),
        static function ($node, $scope) use (&$types, &$continued, $source): void {
            if ($node instanceof PhpParser\Node\Scalar\String_ && str_starts_with($node->value, 'after-')) {
                $continued[$node->value] = $node->getStartFilePos();
            }
            if (!$node instanceof PhpParser\Node\Expr\New_ && !$node instanceof PhpParser\Node\Expr\Assign
                && !($node instanceof PhpParser\Node\Expr\Variable && str_ends_with(substr($source, 0, $node->getStartFilePos()), '/*probe*/ '))) return;
            $type = $scope->getType($node);
            $types[] = [
                'start' => $node->getStartFilePos(), 'end' => $node->getEndFilePos() + 1,
                'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
                'type' => $type->describe(PHPStan\Type\VerbosityLevel::precise()),
                'error' => $type instanceof PHPStan\Type\ErrorType,
                'kind' => $type instanceof PHPStan\Type\NonAcceptingNeverType ? 'nonaccepting'
                    : ($type instanceof PHPStan\Type\NeverType ? ($type->isExplicit() ? 'explicit' : 'implicit') : null),
            ];
        },
    );
    usort($types, static fn($a, $b) => [$a['start'], $a['end']] <=> [$b['start'], $b['end']]);
    ksort($continued);
    echo json_encode(['phar_sha256' => hash_file('sha256', realpath($argv[1])), 'types' => ['providers.php' => $types], 'continued' => ['providers.php' => $continued]], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($tmp, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    rmdir($tmp);
}
