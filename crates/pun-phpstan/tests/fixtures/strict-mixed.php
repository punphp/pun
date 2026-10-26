<?php
// PHPStan 2.3.1 cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
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
        $template = new TemplateMixedType(
            TemplateTypeScope::createWithFunction($spec['scope'] ?? 'fixture'),
            ($spec['argument'] ?? false) ? new TemplateTypeArgumentStrategy() : new TemplateTypeParameterStrategy(),
            TemplateTypeVariance::createInvariant(),
            $spec['name'],
            $input($spec['template']),
            isset($spec['default']) ? $input($spec['default']) : null,
        );
        return ($spec['strict'] ?? false) ? $template->toStrictMixedType() : $template;
    }
    $parts = array_map($input, $spec['parts']);
    return match ($spec['kind']) {
        'union' => new UnionType($parts),
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
foreach (['union', 'intersection', 'combine-union', 'combine-intersection'] as $kind) {
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
echo json_encode(compact('rows', 'pairs'), JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)."\n";
