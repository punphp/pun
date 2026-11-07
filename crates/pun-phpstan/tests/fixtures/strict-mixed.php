<?php
// PHPStan 2.3.5 95d9e8b0578ebf75439a3744a845ef25417028fa.
// php strict-mixed.php /path/to/phpstan.phar > strict-mixed.json
Phar::loadPhar($argv[1], 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\Type\Generic\TemplateMixedType;
use PHPStan\Type\Generic\TemplateTypeArgumentStrategy;
use PHPStan\Type\Generic\TemplateTypeParameterStrategy;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\IntersectionType;
use PHPStan\Type\IntegerType;
use PHPStan\Type\MixedType;
use PHPStan\Type\StrictMixedType;
use PHPStan\Type\Type;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\UnionType;
use PHPStan\Type\VerbosityLevel;
use PHPStan\Type\ErrorType;

// RuleLevelHelper reads generic classes through reflection since PHPStan 2.3.2: boot a container.
$tmp = sys_get_temp_dir() . '/pun-strict-mixed-' . getmypid();
(new PHPStan\DependencyInjection\ContainerFactory($tmp))->create($tmp, [], []);

$input = function (string|array $spec) use (&$input): Type {
    if (is_string($spec)) {
        return match ($spec) {
            'mixed' => new MixedType(),
            'explicit' => new MixedType(true),
            'strict' => new StrictMixedType(),
            'int' => new IntegerType(),
            'string' => new PHPStan\Type\StringType(),
            'null' => new PHPStan\Type\NullType(),
            'object' => new PHPStan\Type\ObjectWithoutClassType(),
            'error' => new ErrorType(),
        };
    }
    if (isset($spec['excluded'])) {
        return new MixedType($spec['explicit'] ?? false, $input($spec['excluded']));
    }
    if (isset($spec['template'])) {
        $template = PHPStan\Type\Generic\TemplateTypeFactory::create(
            TemplateTypeScope::createWithFunction($spec['scope'] ?? 'fixture'),
            $spec['name'],
            $input($spec['template']),
            TemplateTypeVariance::createInvariant(),
            ($spec['argument'] ?? false) ? new TemplateTypeArgumentStrategy() : new TemplateTypeParameterStrategy(),
            isset($spec['default']) ? $input($spec['default']) : null,
        );
        return ($spec['strict'] ?? false) ? $template->toStrictMixedType() : $template;
    }
    $parts = array_map($input, $spec['parts']);
    return match ($spec['kind']) {
        'array' => new PHPStan\Type\ArrayType(...$parts),
        'iterable' => new PHPStan\Type\IterableType(...$parts),
        'generic' => new PHPStan\Type\Generic\GenericObjectType('ArrayObject', $parts),
        'union' => new UnionType($parts),
        'benevolent' => new PHPStan\Type\BenevolentUnionType($parts),
        'intersection' => new IntersectionType($parts),
        'combine-union' => TypeCombinator::union(...$parts),
        'combine-intersection' => TypeCombinator::intersect(...$parts),
    };
};
$template = ['template' => 'mixed', 'name' => 'T'];
$strict = [...$template, 'strict' => true];
$specs = ['mixed', 'explicit', 'strict', $template, $strict];
foreach ([false, true] as $argument) {
    foreach (['mixed', 'explicit', ['excluded' => 'int'], $template] as $bound) {
        foreach ([null, 'string', ['template' => 'mixed', 'name' => 'U', 'default' => 'int']] as $default) {
            $specs[] = ['template' => $bound, 'name' => 'T', 'argument' => $argument, 'default' => $default, 'strict' => true];
        }
    }
}
foreach (['union', 'benevolent', 'intersection', 'combine-union', 'combine-intersection'] as $kind) {
    foreach ([['strict', 'object'], [$strict, 'object'], [$strict, ['template' => 'mixed', 'name' => 'U', 'strict' => true, 'default' => 'int']]] as $parts) {
        $specs[] = compact('kind', 'parts');
    }
}
$queries = ['isArray', 'isCallable', 'isCloneable', 'isIterable', 'isIterableAtLeastOnce', 'isString', 'isInteger', 'isBoolean', 'isNull'];
$casts = ['toString', 'toNumber', 'toInteger', 'toFloat', 'toBoolean', 'toBitwiseNotType'];
$rows = [];
foreach ($specs as $spec) {
    $type = $input($spec);
    $row = [
        'input' => $spec,
        'descriptions' => array_map(fn ($level) => $type->describe($level), [VerbosityLevel::typeOnly(), VerbosityLevel::value(), VerbosityLevel::precise()]),
        'phpdoc' => (new PHPStan\PhpDocParser\Printer\Printer())->print($type->toPhpDocNode()),
        'members' => $type->canAccessProperties()->describe(),
    ];
    // Compound rows exercise combinator identity and descriptions, not unrelated object reflection queries.
    if (!is_array($spec) || !isset($spec['kind'])) {
        $row['queries'] = array_map(fn ($query) => $type->$query()->describe(), $queries);
        $row['casts'] = array_map(fn ($cast) => $type->$cast() instanceof ErrorType, $casts);
        $row['iterable'] = array_map(fn ($query) => $type->$query()->describe(VerbosityLevel::precise()), ['getIterableKeyType', 'getIterableValueType']);
    }
    $rows[] = $row;
}
$pairs = [];
foreach (['strict', $strict] as $a) {
    foreach (['mixed', 'explicit', 'strict', 'int', ['excluded' => 'int'], $template, $strict, [...$strict, 'scope' => 'other'], [...$strict, 'argument' => true], [...$strict, 'default' => 'int']] as $b) {
        $left = $input($a);
        $right = $input($b);
        $pairs[] = [$a, $b, $left->equals($right), $left->isSuperTypeOf($right)->result->describe(), $right->isSuperTypeOf($left)->result->describe(), $left->accepts($right, true)->result->describe(), $right->accepts($left, true)->result->describe()];
    }
}
$tree = function (Type $type) use (&$tree): array {
    if ($type instanceof PHPStan\Type\Generic\TemplateType) {
        return ['template', $type->getName(), $type->getScope()->describe(), $type->isArgument(), $tree($type->getBound()), $type->getDefault() === null ? null : $tree($type->getDefault())];
    }
    if ($type instanceof ErrorType) return ['error'];
    if ($type instanceof StrictMixedType) return ['strict'];
    if ($type instanceof MixedType) {
        return $type->getSubtractedType() === null ? ['mixed', $type->isExplicitMixed()] : ['subtracted', $type->isExplicitMixed(), $tree($type->getSubtractedType())];
    }
    if ($type instanceof PHPStan\Type\ArrayType) return ['array', $tree($type->getKeyType()), $tree($type->getItemType())];
    if ($type instanceof PHPStan\Type\IterableType) return ['iterable', $tree($type->getIterableKeyType()), $tree($type->getIterableValueType())];
    if ($type instanceof PHPStan\Type\Generic\GenericObjectType) return ['generic', $type->getClassName(), array_map($tree, $type->getTypes())];
    if ($type instanceof UnionType) return ['union', array_map($tree, $type->getTypes())];
    if ($type instanceof IntersectionType) return ['intersection', array_map($tree, $type->getTypes())];
    return [$type->describe(VerbosityLevel::precise())];
};
$transformSpecs = ['mixed', 'explicit', 'strict', 'error', ['excluded' => 'int'], ['excluded' => 'int', 'explicit' => true],
    [...$strict, 'default' => 'explicit'],
    ['template' => $template, 'name' => 'Nested', 'default' => ['template' => 'mixed', 'name' => 'Default']],
    ['template' => 'int', 'name' => 'Concrete', 'default' => 'explicit'],
    ['template' => ['kind' => 'array', 'parts' => ['int', 'explicit']], 'name' => 'Items', 'default' => 'mixed'],
    ['kind' => 'array', 'parts' => ['mixed', ['kind' => 'iterable', 'parts' => ['explicit', $strict]]]],
    ['kind' => 'generic', 'parts' => ['mixed', ['template' => 'mixed', 'name' => 'T', 'default' => 'explicit']]],
    ['kind' => 'union', 'parts' => ['object', $strict]],
    ['kind' => 'intersection', 'parts' => [$strict, ['template' => 'mixed', 'name' => 'U']]],
];
$reflection = new ReflectionClass(PHPStan\Rules\RuleLevelHelper::class);
$helper = $reflection->newInstanceWithoutConstructor();
$copies = [];
foreach ($transformSpecs as $spec) {
    $type = $input($spec);
    $checked = $type instanceof TemplateMixedType ? $type->toStrictMixedType() : ($type instanceof MixedType ? new StrictMixedType() : $type);
    $copies[] = ['input' => $spec, 'tree' => $tree($checked)];
}
$transforms = [];
foreach ([false, true] as $explicit) {
    foreach ([false, true] as $implicit) {
        $reflection->getProperty('checkExplicitMixed')->setValue($helper, $explicit);
        $reflection->getProperty('checkImplicitMixed')->setValue($helper, $implicit);
        foreach ($transformSpecs as $spec) {
            $type = $reflection->getMethod('transformCommonType')->invoke($helper, $input($spec));
            $transforms[] = ['input' => $spec, 'explicit' => $explicit, 'implicit' => $implicit, 'tree' => $tree($type)];
        }
    }
}
echo json_encode(compact('rows', 'pairs', 'copies', 'transforms'), JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
