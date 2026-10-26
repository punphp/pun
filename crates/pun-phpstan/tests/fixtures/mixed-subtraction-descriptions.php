<?php
// PHPStan 2.3.1 cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php mixed-subtraction-descriptions.php /path/to/phpstan.phar > mixed-subtraction-descriptions.json
Phar::loadPhar($argv[1], 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\Analyser\NameScope;
use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type\Accessory\NonEmptyArrayType;
use PHPStan\Type\ArrayType;
use PHPStan\Type\BenevolentUnionType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\IntersectionType;
use PHPStan\Type\MixedType;
use PHPStan\Type\Type;
use PHPStan\Type\UnionType;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir().'/pun-subtraction-description-oracle';
@mkdir($tmp);
$resolver = (new ContainerFactory($tmp))->create($tmp, [], [])->getByType(TypeStringResolver::class);
$scope = new NameScope(null, []);
$input = function (string|array $spec, bool $argument = false) use (&$input, $resolver, $scope): Type {
    if (is_string($spec)) {
        return $resolver->resolve($spec, $scope);
    }
    if (isset($spec['template'])) {
        $argument = $argument || $spec['argument'];
        $type = TemplateTypeFactory::create(
            TemplateTypeScope::createWithFunction('subtractionDescriptions'),
            $spec['name'],
            $input($spec['template'], $argument),
            TemplateTypeVariance::createInvariant(),
        );
        return $argument ? $type->toArgument() : $type;
    }
    if (isset($spec['mixed'])) {
        return new MixedType($spec['explicit'], $input($spec['mixed'], $argument));
    }
    if (isset($spec['array'])) {
        return new ArrayType($input($spec['array'][0], $argument), $input($spec['array'][1], $argument));
    }
    if (($spec['accessory'] ?? null) === 'non-empty-array') {
        return new NonEmptyArrayType();
    }
    foreach (['union' => UnionType::class, 'benevolent' => BenevolentUnionType::class, 'intersection' => IntersectionType::class] as $kind => $class) {
        if (isset($spec[$kind])) {
            return new $class(array_map(fn ($part) => $input($part, $argument), $spec[$kind]));
        }
    }
    throw new LogicException('Unknown type input');
};
$domains = [
    'literal' => '0',
    'literal-pipe' => "'a|b'",
    'plain-mixed' => 'mixed',
    'never' => 'never',
    'int' => 'int',
    'array' => 'array<mixed, mixed>',
    'array-union-value' => 'array<int, int|string>',
    'nonempty-array' => 'non-empty-array<mixed, mixed>',
    'explicit-nonempty-array' => ['intersection' => [['array' => ['mixed', 'mixed']], ['accessory' => 'non-empty-array']]],
    'nonempty-list' => 'non-empty-list<string>',
    'nonempty-string' => 'non-empty-string',
    'object-intersection' => ['intersection' => ['Countable', 'Iterator']],
    'union' => ['union' => ['int', 'string']],
    'benevolent' => ['benevolent' => ['int', 'string']],
    'union-intersection' => ['union' => ['non-empty-array<mixed, mixed>', 'null']],
    'excluded-int' => ['mixed' => 'int', 'explicit' => false],
    'excluded-array' => ['mixed' => 'non-empty-array<mixed, mixed>', 'explicit' => true],
    'excluded-union' => ['mixed' => ['union' => ['int', 'string']], 'explicit' => false],
    'intersection-excluded' => ['intersection' => [['mixed' => 'int', 'explicit' => false], 'Stringable']],
];
$rows = [];
$record = function (string $label, array $spec) use (&$rows, $input): void {
    $type = $input($spec);
    $rows[] = [
        'label' => $label,
        'input' => $spec,
        'descriptions' => array_map(fn ($level) => $type->describe($level), [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]),
    ];
};
foreach ([false, true] as $explicit) {
    foreach ($domains as $label => $domain) {
        $record("exclude:$label:$explicit", ['mixed' => $domain, 'explicit' => $explicit]);
    }
    foreach (['int', 'plain-mixed', 'nonempty-array', 'object-intersection', 'union', 'benevolent', 'excluded-int', 'excluded-array'] as $label) {
        foreach ([false, true] as $argument) {
            $template = ['template' => $domains[$label], 'name' => 'T', 'argument' => $argument];
            $record("exclude-template:$label:$explicit:$argument", ['mixed' => $template, 'explicit' => $explicit]);
            $record("template-exclusion:$label:$explicit:$argument", ['template' => ['mixed' => $domains[$label], 'explicit' => $explicit], 'name' => 'T', 'argument' => $argument]);
            $record("exclude-nested-template:$label:$explicit:$argument", ['mixed' => ['template' => $template, 'name' => 'U', 'argument' => $argument], 'explicit' => $explicit]);
        }
    }
}
echo json_encode($rows, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
