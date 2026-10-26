<?php
// PHPStan 2.3.1 public ObjectType/TypeCombinator contracts, not Analyser snapshots.
// php type-api.php /path/to/phpstan.phar > type-api.json
Phar::loadPhar($argv[1], 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\PhpDoc\TypeStringResolver;
use PHPStan\Type\IntersectionType;
use PHPStan\Type\MixedType;
use PHPStan\Type\ObjectType;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\TypeTraverser;
use PHPStan\Type\VerbosityLevel;

$tmp = sys_get_temp_dir().'/pun-object-subtraction-type-api';
@mkdir($tmp);
(new ContainerFactory($tmp))->create($tmp, [], [])->getByType(TypeStringResolver::class);
$a = new ObjectType('A');
$b = new ObjectType('B');
$c = new ObjectType('C');
$ac = new IntersectionType([$a, $c]);
$bc = new IntersectionType([$b, $c]);
$types = [
    'A' => $a,
    'B' => $b,
    'C' => $c,
    'C-not-A' => $c->changeSubtractedType($ac),
    'C-not-B' => $c->changeSubtractedType($bc),
    'A-not-A' => $a->changeSubtractedType($a),
    'C-not-A-or-B' => $c->changeSubtractedType(TypeCombinator::union($ac, $bc)),
    'C-not-never' => $c->changeSubtractedType(new \PHPStan\Type\NeverType()),
    'B-and-C' => $bc,
    'B-and-C-not-A' => new IntersectionType([$b, $c->changeSubtractedType($ac)]),
    'not-A' => new MixedType(subtractedType: $a),
    'not-zero' => new MixedType(subtractedType: new \PHPStan\Type\Constant\ConstantIntegerType(0)),
    'not-C-not-A' => new MixedType(subtractedType: $c->changeSubtractedType($ac)),
];
$describe = static fn ($type) => $type->describe(VerbosityLevel::precise());
$out = [];
foreach ($types as $name => $type) {
    $out['types'][$name] = [
        'description' => $describe($type),
        'object' => $type->isObject()->describe(),
        'string' => $type->isString()->describe(),
        'iterable' => $type->isIterable()->describe(),
        'names' => $type->getObjectClassNames(),
        'key' => $describe($type->getIterableKeyType()),
        'value' => $describe($type->getIterableValueType()),
        'bool' => $describe($type->toBoolean()),
        'stringCastError' => $type->toString() instanceof \PHPStan\Type\ErrorType,
        'traverse' => $describe(TypeTraverser::map($type, static fn ($type, $traverse) => $traverse($type))),
    ];
    foreach ($types as $otherName => $other) {
        $out['relations'][$name][$otherName] = [
            'super' => $type->isSuperTypeOf($other)->result->describe(),
            'accepts' => $type->accepts($other, true)->result->describe(),
        ];
    }
}
foreach ([['C-not-A', 'C-not-B'], ['C', 'C-not-A'], ['A', 'C-not-B'], ['not-A', 'C-not-A'], ['not-A', 'A'], ['not-A', 'B-and-C'], ['not-A', 'B-and-C-not-A']] as [$left, $right]) {
    foreach ([[$left, $right], [$right, $left]] as [$left, $right]) {
        $out['unions']["$left|$right"] = $describe(TypeCombinator::union($types[$left], $types[$right]));
    }
}
foreach ([['not-A', 'C'], ['not-A', 'C-not-B'], ['not-A', 'A'], ['not-C-not-A', 'C'], ['not-zero', 'C']] as [$left, $right]) {
    $out['intersections']["$left&$right"] = $describe(TypeCombinator::intersect($types[$left], $types[$right]));
}
$ba = $b->changeSubtractedType(new IntersectionType([$a, $b]));
$ca = $c->changeSubtractedType($ac);
$compound = [
    'two-exclusions' => new IntersectionType([$ba, $ca]),
    'iterable-conjunct' => new IntersectionType([$b, $ca, new \PHPStan\Type\IterableType(new MixedType(), new MixedType())]),
];
foreach ($compound as $name => $type) {
    $out['compoundUnions'][$name] = $describe(TypeCombinator::union($types['not-A'], $type));
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES), "\n";
