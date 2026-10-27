<?php

// php oracle.php /absolute/path/to/phpstan-2.3.1.phar > expected.json
require 'phar://' . $argv[1] . '/vendor/autoload.php';

use PHPStan\Type\Accessory\AccessoryLowercaseStringType;
use PHPStan\Type\Accessory\AccessoryNonEmptyStringType;
use PHPStan\Type\Accessory\AccessoryNonFalsyStringType;
use PHPStan\Type\Accessory\AccessoryNumericStringType;
use PHPStan\Type\Accessory\AccessoryUppercaseStringType;
use PHPStan\Type\Constant\ConstantStringType;
use PHPStan\Type\Generic\TemplateTypeFactory;
use PHPStan\Type\Generic\TemplateTypeHelper;
use PHPStan\Type\Generic\TemplateTypeMap;
use PHPStan\Type\Generic\TemplateTypeScope;
use PHPStan\Type\Generic\TemplateTypeVariance;
use PHPStan\Type\Generic\TemplateTypeVarianceMap;
use PHPStan\Type\TypeCombinator;
use PHPStan\Type\VerbosityLevel;

function shapeKeys(PHPStan\Type\Type $type): array
{
    $arrays = $type->getConstantArrays();
    if (count($arrays) !== 1) {
        throw new RuntimeException('Expected one constant array.');
    }
    $array = $arrays[0];
    $rows = [];
    foreach ($array->getKeyTypes() as $i => $key) {
        $value = $key->getValue();
        $rows[] = [is_int($value) ? ['int', $value] : ['string', bin2hex($value)], bin2hex($array->getValueTypes()[$i]->getValue())];
    }
    return $rows;
}

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
    $shapeInputs = <<<'SHAPES'
array{"\xff": 'ff', "\xfe": 'fe', "\u{FFFD}": 'replacement'}
array{"\xff": 'first', "\377": 'last', "\xfe": 'other'}
array{'\xff': 'literal', "\xff": 'decoded', '\xfe': 'other'}
array{"\x30\x31": 'first', '01': 'last', '+1': 'plus', 1: 'integer'}
array{bare: 'first', 'bare': 'last', "\x00\xff": 'binary'}
SHAPES;
    $shapeRows = [];
    foreach (explode("\n", $shapeInputs) as $input) {
        $shapeRows[] = ['input' => $input, 'entries' => shapeKeys($resolver->resolve($input))];
    }
    $template = TemplateTypeFactory::create(TemplateTypeScope::createWithFunction('shapeBytes'), 'K', new PHPStan\Type\StringType(), TemplateTypeVariance::createInvariant());
    $scope = new PHPStan\Analyser\NameScope(null, [], null, 'shapeBytes', new TemplateTypeMap(['K' => $template]));
    $lateInputs = <<<'SHAPES'
array{K: 'template', "\xff": 'ff', "\xfe": 'fe', "\u{FFFD}": 'replacement', bare: 'identifier'}
array{K: 'template', "\x31": 'one', '01': 'leading', '-1': 'negative', '+1': 'plus'}
SHAPES;
    $lateRows = [];
    foreach (explode("\n", $lateInputs) as $input) {
        foreach (["\xff", "\xfe", "\xef\xbf\xbd", '1'] as $replacement) {
            $type = $resolver->resolve($input, $scope);
            $resolved = TemplateTypeHelper::resolveTemplateTypes($type, new TemplateTypeMap(['K' => new ConstantStringType($replacement)]), TemplateTypeVarianceMap::createEmpty(), TemplateTypeVariance::createStatic());
            $lateRows[] = ['input' => $input, 'replacement' => bin2hex($replacement), 'entries' => shapeKeys($resolved)];
        }
    }
    echo json_encode(['values' => $rows, 'pairs' => $pairs, 'escapes' => $escapes, 'identity_neighbors' => $neighbors, 'callable_neighbors' => $callableNeighbors, 'shape_keys' => $shapeRows, 'late_shape_keys' => $lateRows], JSON_PRETTY_PRINT | JSON_INVALID_UTF8_SUBSTITUTE | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
