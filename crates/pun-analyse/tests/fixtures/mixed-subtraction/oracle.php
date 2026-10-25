<?php
// php oracle.php /absolute/path/to/phpstan/vendor/autoload.php
declare(strict_types=1);
require $argv[1];

use PHPStan\Type\Constant\ConstantArrayType;
use PHPStan\Type\Constant\ConstantIntegerType;
use PHPStan\Type\Constant\ConstantStringType;
use PHPStan\Type\ArrayType;
use PHPStan\Type\ErrorType;
use PHPStan\Type\IntegerRangeType;
use PHPStan\Type\IntegerType;
use PHPStan\Type\MixedType;
use PHPStan\Type\NeverType;
use PHPStan\Type\NullType;
use PHPStan\Type\StringType;
use PHPStan\Type\TypeCombinator as C;
use PHPStan\Type\VerbosityLevel as V;

$m = new MixedType();
$z = new ConstantIntegerType(0);
$n = new NullType();
$empty = new ConstantArrayType([], []);
$types = [
    'mixed' => $m, 'error' => new ErrorType(), 'zero' => $z,
    'one' => new ConstantIntegerType(1), 'thirteen' => new ConstantIntegerType(13),
    'null' => $n, 'int' => new IntegerType(), 'string' => new StringType(),
    'literal' => new ConstantStringType('x'), 'empty' => $empty, 'never' => new NeverType(),
    'explicit-never' => new NeverType(true),
    'range' => IntegerRangeType::fromInterval(-1, 1),
    'not-zero' => $m->subtract($z), 'not-null' => $m->subtract($n),
    'not-one' => $m->subtract(new ConstantIntegerType(1)),
    'not-zero-null' => $m->subtract($z)->subtract($n),
    'not-int' => $m->subtract(new IntegerType()),
    'not-string' => $m->subtract(new StringType()),
    'not-range' => $m->subtract(IntegerRangeType::fromInterval(-1, 1)),
    'not-empty' => $m->subtract($empty),
    'not-zero-empty' => $m->subtract($z)->subtract($empty),
];
$out = ['types' => [], 'relations' => []];
$record = static function ($type) {
    return ['precise' => $type->describe(V::precise()), 'value' => $type->describe(V::value()),
        'typeOnly' => $type->describe(V::typeOnly()), 'error' => $type instanceof ErrorType,
        'excluded' => $type instanceof MixedType ? $type->getSubtractedType()?->describe(V::precise()) : null];
};
foreach ($types as $name => $type) {
    $out['types'][$name] = $record($type);
}
foreach (['mixed', 'error', 'not-zero', 'not-null', 'not-zero-empty', 'not-empty'] as $name) {
    $out['types']["unset:$name"] = $record($types[$name]->unsetOffset($z));
}
foreach (['mixed', 'error', 'not-zero'] as $name) {
    foreach (['zero', 'null', 'mixed', 'error', 'not-one', 'never'] as $remove) {
        $out['types']["subtract:$name:$remove"] = $record($types[$name]->subtract($types[$remove]));
    }
}
foreach (['not-zero', 'not-null', 'not-zero-null', 'not-int', 'not-string', 'not-range'] as $name) {
    foreach (['zero', 'one', 'thirteen', 'null', 'int', 'string', 'literal', 'mixed', 'error', 'not-zero', 'not-one', 'not-null', 'not-zero-null', 'never', 'explicit-never', 'range'] as $other) {
        foreach ([[$name, $other], [$other, $name]] as [$a, $b]) {
            $out['types']["union:$a:$b"] = $record(C::union($types[$a], $types[$b]));
            $out['types']["intersect:$a:$b"] = $record(C::intersect($types[$a], $types[$b]));
            $out['relations']["$a:$b"] = [
                'super' => strtolower($types[$a]->isSuperTypeOf($types[$b])->result->describe()),
                'accepts' => strtolower($types[$a]->accepts($types[$b], true)->result->describe()),
            ];
        }
    }
}
$out['utilities'] = [
    'toString' => $types['not-zero']->toString() instanceof ErrorType,
    'toInteger' => $types['not-zero']->toInteger() instanceof ErrorType,
    'toBitwiseNotType' => $types['not-zero']->toBitwiseNotType() instanceof ErrorType,
    'reasons' => $z->accepts($types['not-zero'], true)->reasons,
    'accepts-reasons' => $types['not-zero']->accepts($z, true)->reasons,
    'strict-toInteger' => (new \PHPStan\Type\StrictMixedType())->toInteger() instanceof ErrorType,
    'phpdoc' => (string) $types['not-zero']->toPhpDocNode(),
    'cloneable' => $types['not-zero']->isCloneable()->yes(),
    'traverse' => $types['not-zero']->traverse(static fn ($t) => $t instanceof ConstantIntegerType ? new ConstantIntegerType(1) : $t)->describe(V::precise()),
    'implicit' => !$types['not-zero']->isExplicitMixed(),
    'explicit' => !(new MixedType(true, $z))->isExplicitMixed(),
    'array-int' => (new ArrayType($types['not-zero'], new IntegerType()))->describe(V::precise()),
    'array-mixed' => (new ArrayType($types['not-zero'], $m))->describe(V::precise()),
    'legacy-nonnull-union' => C::union($types['not-null'], $m->subtract($n))->describe(V::precise()),
    'legacy-nonnull-intersect' => C::intersect($types['not-zero'], $m->subtract($n))->describe(V::precise()),
    'legacy-nonnull-subtract' => $m->subtract($n)->subtract($z)->describe(V::precise()),
    'legacy-truthy-union' => C::union($types['not-zero'], C::remove($m, \PHPStan\Type\StaticTypeFactory::falsey()))->describe(V::precise()),
    'legacy-nonnull-contained' => $types['not-null']->isSuperTypeOf($m->subtract($n))->yes(),
];
$sections = [];
foreach ($out as $name => $rows) {
    $lines = [];
    foreach ($rows as $label => $result) {
        $lines[] = '    ' . json_encode($label) . ': ' . json_encode($result, JSON_UNESCAPED_SLASHES);
    }
    $sections[] = '  ' . json_encode($name) . ": {\n" . implode(",\n", $lines) . "\n  }";
}
file_put_contents(__DIR__ . '/types.json', "{\n" . implode(",\n", $sections) . "\n}\n");
