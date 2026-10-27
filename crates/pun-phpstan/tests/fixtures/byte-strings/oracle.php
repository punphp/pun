<?php

// php oracle.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
require 'phar://' . $argv[1] . '/vendor/autoload.php';

use PHPStan\Type\Accessory\AccessoryLowercaseStringType;
use PHPStan\Type\Accessory\AccessoryNonEmptyStringType;
use PHPStan\Type\Accessory\AccessoryNonFalsyStringType;
use PHPStan\Type\Accessory\AccessoryNumericStringType;
use PHPStan\Type\Accessory\AccessoryUppercaseStringType;
use PHPStan\Type\Constant\ConstantStringType;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel;

$values = ['', '0', '12', 'e', 'E', 'é', "\xff", "\xfe", "\xc3", "\xc2", "\xffa", "\xffA", "\0\n\t\r\v\f\x07\x08\\\"'", "\x01\x1f\x7f", 'abcdefghijklmnopqrstuvwxy', str_repeat('é', 25), str_repeat("\xff", 25), str_repeat('é', 11) . "\xff", "\xed\xa0\x80", "\xe0\x80\x80", "\xf4\x90\x80\x80"];
$rows = [];
foreach ($values as $value) {
    $type = new ConstantStringType($value);
    $queries = [];
    foreach ([new AccessoryNonEmptyStringType(), new AccessoryNonFalsyStringType(), new AccessoryNumericStringType(), new AccessoryLowercaseStringType(), new AccessoryUppercaseStringType()] as $accessory) {
        $queries[] = $accessory->isSuperTypeOf($type)->yes();
    }
    $rows[] = [
        'hex' => bin2hex($value),
        'constants' => array_map(static fn($constant) => bin2hex($constant->getValue()), $type->getConstantStrings()),
        'queries' => $queries,
        // Only the final UTF-8 presentation boundary substitutes invalid diagnostic bytes.
        'description' => $type->describe(VerbosityLevel::precise()),
        'value_description' => $type->describe(VerbosityLevel::value()),
    ];
}
$pairs = [];
foreach ($values as $left) {
    foreach ($values as $right) {
        $a = new ConstantStringType($left);
        $b = new ConstantStringType($right);
        $pairs[] = [bin2hex($left), bin2hex($right), $a->equals($b), $a->isSuperTypeOf($b)->yes(), $a->accepts($b, true)->yes(), array_map(static fn($constant) => bin2hex($constant->getValue()), TypeCombinator::union($a, $b)->getConstantStrings())];
    }
}
$literals = <<<'LITERALS'
'a\\b\'c'
"\\\"\n\r\t\f\v\e\Xff\xff\377\777\$\q"
"\u{41}"
"\u{e9}"
"\u{20ac}"
"\u{d800}"
"\u{1f600}"
"\u{110000}"
"\u{200000}"
"\u{fffffffff}"
LITERALS;
$escapes = [];
foreach (explode("\n", $literals) as $literal) {
    $escapes[] = [bin2hex($literal), bin2hex(PHPStan\PhpDocParser\Parser\StringUnescaper::unescapeString($literal))];
}
$root = sys_get_temp_dir() . '/pun-byte-string-oracle-' . getmypid();
try {
    $container = (new PHPStan\DependencyInjection\ContainerFactory(__DIR__))->create($root, ['phar://' . $argv[1] . '/conf/config.level0.neon'], []);
    $provider = $container->getByType(PHPStan\Reflection\ReflectionProvider::class);
    $neighbors = [];
    foreach ([['stdClass', 'stdClass'], ['stdClass', 'DateTime']] as [$left, $right]) {
        $neighbors[] = [$left, $right, (new PHPStan\Type\ThisType($provider->getClass($left)))->equals(new PHPStan\Type\ThisType($provider->getClass($right)))];
    }
    $resolver = $container->getByType(PHPStan\PhpDoc\TypeStringResolver::class);
    $callableNeighbors = [];
    foreach (["array{'Closure', 'bind'}", "array{'Closure', 'bind'|1}"] as $input) {
        $callable = $resolver->resolve($input)->isCallable();
        $callableNeighbors[] = [$input, [$callable->yes(), $callable->maybe(), $callable->no()]];
    }
    echo json_encode(['values' => $rows, 'pairs' => $pairs, 'escapes' => $escapes, 'identity_neighbors' => $neighbors, 'callable_neighbors' => $callableNeighbors], JSON_PRETTY_PRINT | JSON_INVALID_UTF8_SUBSTITUTE | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
