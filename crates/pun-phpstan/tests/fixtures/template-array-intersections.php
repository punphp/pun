<?php
// PHPStan 2.3.1 (cb3b7e99489b2e3789ee2eec10501cb7a72580ea) oracle exporter.
// php template-array-intersections.php /path/to/phpstan.phar > template-array-intersections.json
Phar::loadPhar($argv[1], 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\Analyser\NameScope;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type\Generic\TemplateType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir().'/pun-template-array-oracle';
@mkdir($tmp);
$container = (new ContainerFactory($tmp))->create($tmp, [], []);
$resolver = $container->getByType(TypeStringResolver::class);
$scope = new NameScope(null, []);
$rows = [];
foreach ([
    ['array<string, int>', 'array{a: int}'],
    ['array<string, int>', 'array{a: 1, b?: 2}'],
    ['array<string, int>', 'array<int, int>'],
    ['array<int, int>', 'array<string, int>'],
    ['array<int, int|string>', 'array<int|string, int>'],
    ['array<int|string, int>', 'array<int, int|string>'],
    ['array<string, int>', 'array<string, int|string>'],
    ['array<string, int>', 'array<string, int>'],
    ['array<string, int>', 'mixed'],
    ['array<string, int>', 'int'],
    ['array<string, int>', 'non-empty-array<string, int>'],
    ['array<int, int>', 'list<int>'],
    ['array{a: int}', 'array<string, int>'],
    ['array{a: int|string, b?: int}', 'array<string, int>'],
    ['array{a?: int}', 'non-empty-array<string, int>'],
    ['array{0?: int, 1?: int}', 'list<int>'],
    ['array{a: int}', 'list<int>'],
    ['array{a: int}', 'array{a: int}'],
    ['array{a: int}', 'array'],
    ['iterable<int, int|string>', 'array<int|string, int>'],
    ['array<int, int|string>', 'iterable<int|string, int>'],
    ['iterable<int, int|string>', 'iterable<int|string, int>'],
    ['array<string, int>', 'array<string, int>', 'T', 'fixture'],
    ['array<string, int>', 'array<string, int>', 'U', 'fixture'],
    ['array<string, int>', 'array<string, int>', 'T', 'other'],
    ['array<string, int>', 'array{a: int}', 'U', 'other'],
] as $case) {
    [$bound, $other] = $case;
    foreach ([false, true] as $argument) {
        $template = TemplateTypeFactory::create(
            TemplateTypeScope::createWithFunction('fixture'),
            'T',
            $resolver->resolve($bound, $scope),
            TemplateTypeVariance::createInvariant(),
            default: $resolver->resolve('array{}', $scope),
        );
        if ($argument) {
            $template = $template->toArgument();
        }
        $otherType = $resolver->resolve($other, $scope);
        $otherTemplate = isset($case[2]) ? ['name' => $case[2], 'scope' => 'function '.$case[3].'()'] : null;
        if ($otherTemplate !== null) {
            $otherType = TemplateTypeFactory::create(
                TemplateTypeScope::createWithFunction($case[3]),
                $case[2],
                $otherType,
                TemplateTypeVariance::createInvariant(),
                default: $resolver->resolve('array{}', $scope),
            );
        }
        foreach ([false, true] as $reverse) {
            $result = $reverse
                ? TypeCombinator::intersect($otherType, $template)
                : TypeCombinator::intersect($template, $otherType);
            $rows[] = [
                'bound' => $bound,
                'other' => $other,
                'argument' => $argument,
                'reverse' => $reverse,
                'otherTemplate' => $otherTemplate,
                'descriptions' => array_map(fn ($level) => $result->describe($level), [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]),
                'template' => $result instanceof TemplateType,
            ];
        }
    }
}
foreach ([
    ['list<int>', "array{'final'}"],
    ['array<int, int>', "array{'final'}"],
    ['array{}', "array{'final'}"],
    ['non-empty-list<int>', "array{'final'}"],
    ['list<int>', 'array{0?: int}'],
    ['list<int>', 'array{}'],
] as [$bound, $with]) {
    foreach ([false, true] as $argument) {
        $template = TemplateTypeFactory::create(
            TemplateTypeScope::createWithFunction('fixture'),
            'T',
            $resolver->resolve($bound, $scope),
            TemplateTypeVariance::createInvariant(),
            default: $resolver->resolve('array{}', $scope),
        );
        if ($argument) {
            $template = $template->toArgument();
        }
        $empty = TypeCombinator::intersect($template, $resolver->resolve('array{}', $scope));
        foreach ([false, true] as $reverse) {
            $other = $resolver->resolve($with, $scope);
            $union = $reverse ? TypeCombinator::union($other, $empty) : TypeCombinator::union($empty, $other);
            $describe = fn ($type) => array_map(fn ($level) => $type->describe($level), [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]);
            $rows[] = [
                'bound' => $bound,
                'other' => 'array{}',
                'argument' => $argument,
                'reverse' => false,
                'otherTemplate' => null,
                'descriptions' => $describe($empty),
                'template' => $empty instanceof TemplateType,
                'union' => ['with' => $with, 'reverse' => $reverse, 'descriptions' => $describe($union)],
            ];
        }
    }
}
echo json_encode($rows, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
