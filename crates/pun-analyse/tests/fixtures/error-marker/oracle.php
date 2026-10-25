<?php
// php oracle.php /absolute/path/to/phpstan/vendor/autoload.php
// Pinned PHPStan 2.3.1; these are public type operations, not reconstructed expectations.
declare(strict_types=1);

require $argv[1];

use PHPStan\Type\Constant\ConstantIntegerType;
use PHPStan\Type\ErrorType;
use PHPStan\Type\IntegerType;
use PHPStan\Type\MixedType;
use PHPStan\Type\NullType;
use PHPStan\Type\StaticTypeFactory;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel;

$error = new ErrorType();
$mixed = new MixedType();
$nonnull = new MixedType(false, new NullType());
$truthy = TypeCombinator::remove($mixed, StaticTypeFactory::falsey());
$integer = new ConstantIntegerType(2);
$types = [
    'error' => $error,
    'mixed' => $mixed,
    'error|error' => TypeCombinator::union($error, new ErrorType()),
    'error|integer' => TypeCombinator::union($error, $integer),
    'integer|error' => TypeCombinator::union($integer, $error),
    'error|13' => TypeCombinator::union($error, new ConstantIntegerType(13)),
    '13|error' => TypeCombinator::union(new ConstantIntegerType(13), $error),
    'error|mixed' => TypeCombinator::union($error, $mixed),
    'mixed|error' => TypeCombinator::union($mixed, $error),
    'integer|error|mixed' => TypeCombinator::union($integer, $error, $mixed),
    'integer|mixed|error' => TypeCombinator::union($integer, $mixed, $error),
    'error|nonnull' => TypeCombinator::union($error, $nonnull),
    'nonnull|error' => TypeCombinator::union($nonnull, $error),
    'error&error' => TypeCombinator::intersect($error, new ErrorType()),
    'error&mixed' => TypeCombinator::intersect($error, $mixed),
    'mixed&error' => TypeCombinator::intersect($mixed, $error),
    'error&integer' => TypeCombinator::intersect($error, $integer),
    'integer&error' => TypeCombinator::intersect($integer, $error),
    'integer&int' => TypeCombinator::intersect($integer, new IntegerType()),
    'int&integer' => TypeCombinator::intersect(new IntegerType(), $integer),
    'error&nonnull' => TypeCombinator::intersect($error, $nonnull),
    'nonnull&error' => TypeCombinator::intersect($nonnull, $error),
    'error&truthy' => TypeCombinator::intersect($error, $truthy),
    'truthy&error' => TypeCombinator::intersect($truthy, $error),
    'error&falsey' => TypeCombinator::intersect($error, StaticTypeFactory::falsey()),
    'falsey&error' => TypeCombinator::intersect(StaticTypeFactory::falsey(), $error),
    'error-null' => $error->subtract(new NullType()),
    'error-falsey' => $error->subtract(StaticTypeFactory::falsey()),
];
$expected = [];
foreach ($types as $label => $type) {
    $expected[$label] = [
        'error' => $type instanceof ErrorType,
        'type' => $type->describe(VerbosityLevel::precise()),
        'mixed' => $type instanceof MixedType,
        'unconstrained' => $type instanceof MixedType && $type->getSubtractedType() === null,
        'falsey' => $type->toBoolean()->isFalse()->yes(),
    ];
}
file_put_contents(__DIR__ . '/types.json', json_encode($expected, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
