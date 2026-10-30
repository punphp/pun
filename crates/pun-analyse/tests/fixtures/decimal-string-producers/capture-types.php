<?php

// Public PHPStan type API controls for the source string representation.
$phar = realpath($argv[1]);
require 'phar://' . $phar . '/vendor/autoload.php';
$root = sys_get_temp_dir() . '/pun-decimal-types-' . getmypid();
mkdir($root);
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create(
        $root, ["phar://$phar/conf/config.level0.neon"], [],
    );
    $resolver = $container->getByType(PHPStan\PhpDoc\TypeStringResolver::class);
    $inputs = [
        'string', 'decimal-int-string', 'non-decimal-int-string', 'numeric-string',
        'decimal-int-string&non-empty-string', 'decimal-int-string&non-falsy-string',
        'non-decimal-int-string&numeric-string', 'non-decimal-int-string&lowercase-string',
        'non-decimal-int-string&uppercase-string', 'non-decimal-int-string&lowercase-string&uppercase-string',
        'decimal-int-string&literal-string', 'non-decimal-int-string&non-empty-string',
        "'0'", "'-12'", "'01'", "'+1'", "'-0'", "'1.0'", "'1e2'", "'9223372036854775808'", "''",
    ];
    $types = array_map($resolver->resolve(...), $inputs);
    $describe = static fn($type) => $type->describe(PHPStan\Type\VerbosityLevel::precise());
    $out = ['types' => [], 'intersections' => [], 'contains' => []];
    foreach ($types as $i => $type) {
        $out['types'][] = [
            'input' => $inputs[$i], 'describe' => $describe($type),
            'decimal' => $type->isDecimalIntegerString()->describe(),
            'numeric' => $type->isNumericString()->yes(), 'nonempty' => $type->isNonEmptyString()->yes(),
            'lower' => $type->isLowercaseString()->yes(), 'upper' => $type->isUppercaseString()->yes(),
        ];
        foreach ($types as $j => $right) {
            $out['contains'][] = [$i, $j, $type->isSuperTypeOf($right)->yes()];
        }
    }
    foreach ([[1, 2], [2, 1], [1, 4], [4, 1], [1, 12], [12, 1], [1, 14], [14, 1], [2, 12], [12, 2], [2, 14], [14, 2], [0, 1], [1, 0], [2, 3]] as [$i, $j]) {
        $out['intersections'][] = [$i, $j, $describe(PHPStan\Type\TypeCombinator::intersect($types[$i], $types[$j]))];
    }
    echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
