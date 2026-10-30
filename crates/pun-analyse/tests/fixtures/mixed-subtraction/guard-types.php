<?php
// php guard-types.php /absolute/path/to/phpstan/vendor/autoload.php
declare(strict_types=1);
require $argv[1];

use PHPStan\Type\Accessory\AccessoryArrayListType;
use PHPStan\Type\ArrayType;
use PHPStan\Type\IntegerRangeType;
use PHPStan\Type\IntegerType;
use PHPStan\Type\MixedType;
use PHPStan\Type\TypeCombinator as C;

$implicit = new MixedType();
$explicit = new MixedType(true);
$domains = [
    'array' => new ArrayType($explicit, $explicit),
    'list' => C::intersect(new ArrayType(IntegerRangeType::fromInterval(0, null), $explicit), new AccessoryArrayListType()),
];
$out = [];
foreach ($domains as $name => $domain) {
    $excluded = $implicit->subtract($domain)->unsetOffset(new IntegerType())->getSubtractedType();
    $out["unset:$name"] = [
        'nonempty' => $excluded->isIterableAtLeastOnce()->yes(),
        'item-explicit' => $excluded->getIterableValueType()->isExplicitMixed(),
    ];
}
$types = ['implicit' => $implicit, 'explicit' => $explicit, 'int' => new IntegerType()];
$types['excluded'] = $explicit->subtract(new IntegerType());
foreach (['implicit', 'explicit', 'excluded'] as $a) {
    foreach ($types as $b => $type) {
        foreach (['union', 'intersect'] as $operation) {
            $result = C::$operation($types[$a], $type);
            $out["$operation:$a:$b"] = $result instanceof MixedType ? $result->isExplicitMixed() : null;
        }
    }
}
$out['subtract-explicit'] = $types['excluded']->isExplicitMixed();
$out['unset-explicit'] = $types['excluded']->unsetOffset(new IntegerType())->isExplicitMixed();
echo json_encode($out, JSON_PRETTY_PRINT | JSON_THROW_ON_ERROR), "\n";
