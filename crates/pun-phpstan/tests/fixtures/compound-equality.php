<?php
// php compound-equality.php /path/to/phpstan-2.3.1.phar > compound-equality.json
Phar::loadPhar($argv[1], 'compound-equality.phar');
require 'phar://compound-equality.phar/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\Reflection\ReflectionProvider;
use PHPStan\Type\BenevolentUnionType;
use PHPStan\Type\Generic\GenericObjectType;
use PHPStan\Type\IntersectionType;
use PHPStan\Type\ObjectType;
use PHPStan\Type\UnionType;

$tmp = sys_get_temp_dir() . '/pun-compound-equality-' . getmypid();
(new ContainerFactory($tmp))->create($tmp, [], [])->getByType(ReflectionProvider::class);
$inputs = [
    [['A', 'B'], ['B', 'A']],
    [['A', 'B'], ['A', 'C']],
    [['A', 'A', 'B'], ['B', 'A', 'A']],
    [['A', 'A', 'B'], ['A', 'B', 'B']],
    [['A', 'B'], ['A', 'A', 'B']],
];
$rows = [];
foreach (['union' => UnionType::class, 'benevolent' => BenevolentUnionType::class, 'intersection' => IntersectionType::class] as $kind => $class) {
    foreach ($inputs as [$left, $right]) {
        $make = static fn (array $names) => new $class(array_map(static fn ($name) => new ObjectType($name), $names));
        $a = $make($left);
        $b = $make($right);
        $genericA = new GenericObjectType('ArrayIterator', [new ObjectType('Key'), $a]);
        $genericB = new GenericObjectType('ArrayIterator', [new ObjectType('Key'), $b]);
        $swapped = new GenericObjectType('ArrayIterator', [$b, new ObjectType('Key')]);
        $rows[] = [$kind, $left, $right, $a->equals($b), $genericA->equals($genericB), $genericA->equals($swapped)];
    }
}
echo "[\n" . implode(",\n", array_map(static fn ($row) => '  ' . json_encode($row, JSON_THROW_ON_ERROR), $rows)) . "\n]\n";
