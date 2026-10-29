<?php declare(strict_types=1);

// PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
// php object-subtraction-api.php /absolute/path/phpstan.phar /tmp/oracle-cache > object-subtraction-api.json
require 'phar://' . realpath($argv[1]) . '/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type as T;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel as V;

$tmp = $argv[2];
@mkdir($tmp, recursive: true);
$resolver = (new ContainerFactory($tmp))->create($tmp, [], [])->getByType(TypeStringResolver::class);
$inputs = [
    'exception' => 'Exception', 'runtime' => 'RuntimeException', 'logic' => 'LogicException',
    'std' => 'stdClass', 'mixed' => 'mixed', 'never' => 'never',
    'not-runtime' => ['object' => 'Exception', 'excluded' => 'RuntimeException'],
    'not-logic' => ['object' => 'Exception', 'excluded' => 'LogicException'],
    'throwable-not-runtime' => ['object' => 'Throwable', 'excluded' => 'RuntimeException'],
    'throwable-not-logic' => ['object' => 'Throwable', 'excluded' => 'LogicException'],
    'logic-not-exception' => ['object' => 'LogicException', 'excluded' => 'Exception'],
    'iterator-not-array' => ['object' => 'Iterator<int, string>', 'excluded' => 'ArrayIterator<int, string>'],
    'union' => ['union' => ['RuntimeException', 'LogicException']],
    'mixed-not-exception' => ['mixed' => 'Exception'],
    'explicit-not-exception' => ['mixed' => 'Exception', 'explicit' => true],
    'mixed-not-subtracted' => ['mixed' => ['object' => 'Exception', 'excluded' => 'RuntimeException']],
    'mixed-not-union' => ['mixed' => ['union' => ['Exception', 'stdClass']]],
    'mixed-not-partly-known' => ['mixed' => ['union' => ['Exception', 'UnknownExcluded']]],
    'mixed-not-throwable' => ['mixed' => 'Throwable'],
    'mixed-not-datetime' => ['mixed' => 'DateTimeInterface'],
    'throwable-not-exception' => ['object' => 'Throwable', 'excluded' => 'Exception'],
    'not-runtime-logic' => ['object' => 'Exception', 'excluded' => ['union' => ['RuntimeException', 'LogicException']]],
    'not-nested' => ['object' => 'Exception', 'excluded' => ['object' => 'RuntimeException', 'excluded' => 'LogicException']],
    'error-object' => 'Error', 'datetime' => 'DateTime', 'datetime-immutable' => 'DateTimeImmutable',
    'static-runtime' => ['static' => 'RuntimeException'], 'traversable' => 'Traversable',
    'parent' => ['parent' => true], 'never-implicit' => ['never' => true],
];
$input = function ($spec) use (&$input, $resolver): T\Type {
    if (is_string($spec)) { return $resolver->resolve($spec); }
    if (isset($spec['object'])) {
        return $input($spec['object'])->changeSubtractedType($input($spec['excluded']));
    }
    if (isset($spec['mixed'])) { return new T\MixedType($spec['explicit'] ?? false, $input($spec['mixed'])); }
    if (isset($spec['static'])) { return new T\StaticType($input($spec['static'])->getClassReflection()); }
    if (isset($spec['parent'])) { return new T\NonexistentParentClassType(); }
    if (isset($spec['never'])) { return new T\NeverType(); }
    return new T\UnionType(array_map($input, $spec['union']));
};
$types = array_map($input, $inputs);
$describe = fn ($type) => array_map(fn ($v) => $type->describe($v), [V::precise(), V::value(), V::typeOnly()]);
$out = ['inputs' => $inputs];
foreach (['not-runtime', 'iterator-not-array'] as $name) {
    $type = $types[$name];
    $out['queries'][$name] = [
        'descriptions' => $describe($type), 'names' => $type->getObjectClassNames(),
        'key' => $describe($type->getIterableKeyType()), 'value' => $describe($type->getIterableValueType()),
    ];
}
foreach (['exception', 'runtime', 'logic', 'std', 'not-logic', 'throwable-not-runtime',
    'logic-not-exception', 'union', 'mixed', 'never'] as $other) {
    foreach ([['not-runtime', $other], [$other, 'not-runtime']] as [$a, $b]) {
        $out['super']["$a:$b"] = $types[$a]->isSuperTypeOf($types[$b])->result->describe();
    }
}
foreach (['logic', 'runtime', 'std', 'not-logic'] as $other) {
    foreach ([['not-runtime', $other], [$other, 'not-runtime']] as [$a, $b]) {
        $out['unions']["$a:$b"] = $describe(TypeCombinator::union($types[$a], $types[$b]));
    }
}
$observe = function (T\Type $type, T\Type $a, T\Type $b) use ($describe): array {
    $out = [
        'descriptions' => $describe($type), 'names' => $type->getObjectClassNames(),
        'key' => $describe($type->getIterableKeyType()), 'value' => $describe($type->getIterableValueType()),
        'implicitMixed' => $type instanceof T\MixedType && !$type->isExplicitMixed(),
    ];
    foreach (['toString', 'toNumber', 'toInteger', 'toFloat', 'toBoolean', 'toBitwiseNotType'] as $method) {
        $out['casts'][] = $type->$method() instanceof T\ErrorType;
    }
    foreach ([$a, $b] as $other) {
        $out['relations'][] = [
            'equals' => [$type->equals($other), $other->equals($type)],
            'super' => [$type->isSuperTypeOf($other)->result->describe(), $other->isSuperTypeOf($type)->result->describe()],
            'accepts' => array_map(fn ($strict) => [
                $type->accepts($other, $strict)->result->describe(), $other->accepts($type, $strict)->result->describe(),
            ], [false, true]),
        ];
    }
    return $out;
};
foreach ([
    ['not-runtime', 'not-logic'], ['not-runtime', 'throwable-not-runtime'],
    ['not-runtime', 'throwable-not-logic'], ['not-runtime', 'runtime'], ['not-runtime', 'logic'],
    ['not-runtime', 'std'], ['not-runtime', 'mixed'], ['not-runtime', 'never-implicit'], ['not-runtime', 'parent'],
    ['mixed-not-exception', 'runtime'], ['mixed-not-exception', 'exception'], ['mixed-not-exception', 'std'],
    ['explicit-not-exception', 'runtime'], ['mixed-not-subtracted', 'logic'], ['mixed-not-subtracted', 'runtime'],
    ['mixed-not-union', 'runtime'], ['mixed-not-throwable', 'exception'], ['mixed-not-throwable', 'error-object'],
    ['mixed-not-datetime', 'datetime'], ['mixed-not-datetime', 'datetime-immutable'],
    ['throwable-not-exception', 'runtime'], ['not-runtime-logic', 'runtime'], ['not-nested', 'runtime'],
    ['mixed-not-exception', 'static-runtime'], ['mixed-not-throwable', 'not-runtime'],
    ['mixed-not-exception', 'traversable'],
    ['mixed-not-partly-known', 'runtime'],
] as [$a, $b]) {
    foreach ([[$a, $b], [$b, $a]] as [$left, $right]) {
        $out['combinators']["$left:$right"] = [
            'union' => $observe(TypeCombinator::union($types[$left], $types[$right]), $types[$left], $types[$right]),
            'intersection' => $observe(TypeCombinator::intersect($types[$left], $types[$right]), $types[$left], $types[$right]),
        ];
    }
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR) . "\n";
