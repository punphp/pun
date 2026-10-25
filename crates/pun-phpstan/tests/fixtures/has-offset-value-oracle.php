<?php
// php has-offset-value-oracle.php /absolute/path/to/phpstan/vendor/autoload.php
// Public operations from PHPStan 2.3.1, cb3b7e99489b2e3789ee2eec10501cb7a72580ea.
declare(strict_types=1);
require $argv[1];

use PHPStan\Type\Accessory\HasOffsetValueType;
use PHPStan\Type\Accessory\NonEmptyArrayType;
use PHPStan\Type\ArrayType;
use PHPStan\Type\Constant\ConstantArrayType;
use PHPStan\Type\Constant\ConstantIntegerType;
use PHPStan\Type\Constant\ConstantStringType;
use PHPStan\Type\ErrorType;
use PHPStan\Type\IntegerType;
use PHPStan\Type\StringType;
use PHPStan\Type\Type;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel;

$int = new IntegerType();
$string = new StringType();
$five = new ConstantIntegerType(5);
$a = new ConstantStringType('a');
$rows = [
    [['offset' => 5, 'value' => 'string'], new HasOffsetValueType($five, $string)],
    [['offset' => 5, 'value' => 'int'], new HasOffsetValueType($five, $int)],
    [['offset' => 7, 'value' => 'string'], new HasOffsetValueType(new ConstantIntegerType(7), $string)],
    [['offset' => 'a', 'value' => 'int'], new HasOffsetValueType($a, $int)],
    [['type' => 'array<int, string>'], new ArrayType($int, $string)],
    [['type' => 'array<int, int>'], new ArrayType($int, $int)],
    [['type' => 'array{5: string}'], new ConstantArrayType([$five], [$string])],
    [['type' => 'array{5?: string}'], new ConstantArrayType([$five], [$string], [0], [0])],
    [['type' => 'array{a: int}'], new ConstantArrayType([$a], [$int])],
    [['type' => 'array{}'], new ConstantArrayType([], [])],
    [['type' => 'array{...<int, string>}'], new ConstantArrayType([], [], [0], [], null, [$int, $string])],
    [['type' => 'int'], $int],
    [['type' => 'array<int, *ERROR*>'], new ArrayType($int, new ErrorType())],
];
$out = [];
foreach ($rows as [$input, $type]) {
    $out['types'][] = [
        'input' => $input,
        'describe' => array_map(fn ($level) => $type->describe($level), [VerbosityLevel::precise(), VerbosityLevel::value(), VerbosityLevel::typeOnly()]),
        'queries' => array_map(fn ($method) => $type->$method()->describe(), ['isArray', 'isString', 'isList', 'isIterable', 'isIterableAtLeastOnce', 'isCloneable']),
    ];
}
foreach ($rows as $i => [, $left]) {
    foreach ($rows as $j => [, $right]) {
        if (!$left instanceof HasOffsetValueType && !$right instanceof HasOffsetValueType) {
            continue;
        }
        $out['comparisons'][] = [$i, $j, $left->isSuperTypeOf($right)->result->describe()];
    }
}
$base = $rows[0][1];
$out['traverse'] = $base->traverse(fn (Type $t) => $t instanceof StringType ? $int : new ConstantIntegerType(7))->describe(VerbosityLevel::precise());
$wrapped = TypeCombinator::intersect($rows[4][1], new NonEmptyArrayType(), $base);
$out['wrapped'] = $wrapped->describe(VerbosityLevel::precise());
$out['phpdoc'] = [(string) $base->toPhpDocNode(), (string) $wrapped->toPhpDocNode()];
file_put_contents(__DIR__ . '/has-offset-value.json', json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n");
