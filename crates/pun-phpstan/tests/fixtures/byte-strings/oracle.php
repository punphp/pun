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

function shapeKeyMetadata(PHPStan\Type\Type $type): array
{
    $arrays = $type->getConstantArrays();
    if (count($arrays) !== 1) {
        throw new RuntimeException('Expected one constant array.');
    }
    $array = $arrays[0];
    $entries = [];
    foreach ($array->getKeyTypes() as $i => $key) {
        $value = $key->getValue();
        $values = array_map(static fn($constant) => bin2hex($constant->getValue()), $array->getValueTypes()[$i]->getConstantStrings());
        sort($values);
        $entries[] = [is_int($value) ? ['int', $value] : ['string', bin2hex($value)], $values, $array->isOptionalKey($i)];
    }
    $list = $array->isList();
    return ['entries' => $entries, 'list' => [$list->yes(), $list->maybe(), $list->no()]];
}

// The parser boundary retains quoted numeric strings; numeric key coercion belongs to type resolution.
function parsedBytes(PHPStan\PhpDocParser\Ast\Type\TypeNode $node): array|string
{
    if ($node instanceof PHPStan\PhpDocParser\Ast\Type\ConstTypeNode) {
        return bin2hex($node->constExpr->value);
    }
    return array_map(static function ($item) {
        $key = $item->keyName;
        if ($key instanceof PHPStan\PhpDocParser\Ast\ConstExpr\ConstExprStringNode) {
            $key = ['string', bin2hex($key->value)];
        } elseif ($key instanceof PHPStan\PhpDocParser\Ast\ConstExpr\ConstExprIntegerNode) {
            $key = ['int', $key->value];
        } else {
            $key = ['ident', $key->name];
        }
        return [$key, parsedBytes($item->valueType)];
    }, $node->items);
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
    $parserConfig = new PHPStan\PhpDocParser\ParserConfig([]);
    $lexer = new PHPStan\PhpDocParser\Lexer\Lexer($parserConfig);
    $constParser = new PHPStan\PhpDocParser\Parser\ConstExprParser($parserConfig);
    $typeParser = new PHPStan\PhpDocParser\Parser\TypeParser($parserConfig, $constParser);
    $docParser = new PHPStan\PhpDocParser\Parser\PhpDocParser($parserConfig, $typeParser, $constParser);
    $nodeResolver = $container->getByType(PHPStan\PhpDoc\TypeNodeResolver::class);
    $numericInputs = <<<'SHAPES'
array{"\x31": 'first', 1: 'last'}
array{1: 'first', "\x31": 'last'}
array{"\x31"?: 'first', 1: 'last'}
array{1: 'first', "\x31"?: 'last'}
array{"\x31"?: 'first', 1?: 'last'}
array{0: 'first', "\x30"?: 'last'}
array{0?: 'first', "\x30": 'last'}
array{'0': 'zero', '00': 'double', '-0': 'negative-zero', '+1': 'plus', '01': 'leading', ' 1': 'space', '1.0': 'decimal', bare: 'bare'}
array{'9223372036854775807': 'max', '9223372036854775808': 'overflow', '-9223372036854775808': 'min', '-9223372036854775809': 'underflow'}
array{"\x2d1": 'first', -1: 'last', "1\x00": 'nul', "\xff1": 'ff', "\xfe1": 'fe', "\u{FFFD}1": 'replacement'}
array{'\x31': 'single', "\x31": 'double', 1: 'last'}
array{"\x31": 'explicit', ...<1, 'extra'>}
array{1: 'explicit', ...<"\x31", 'extra'>}
array{"\x31"?: 'explicit', ...<1|2, 'extra'>}
array{'01': 'leading', ...<1|'01', 'extra'>}
array{"\x30": 'zero', ...<0|1, 'extra'>}
array{"\x2d1": 'negative', ...<-1|0, 'extra'>}
SHAPES;
    $numericRows = [];
    foreach (explode("\n", $numericInputs) as $input) {
        $node = $typeParser->parse(new PHPStan\PhpDocParser\Parser\TokenIterator($lexer->tokenize($input)));
        $row = ['source_hex' => bin2hex($input), 'resolved' => shapeKeyMetadata($nodeResolver->resolve($node, new PHPStan\Analyser\NameScope(null, [])))];
        if ($node->sealed) {
            $builder = PHPStan\Type\Constant\ConstantArrayTypeBuilder::createEmpty();
            $builder->disableArrayDegradation();
            $items = [];
            foreach ($node->items as $item) {
                $key = $item->keyName;
                $keyType = $key instanceof PHPStan\PhpDocParser\Ast\ConstExpr\ConstExprIntegerNode
                    ? new PHPStan\Type\Constant\ConstantIntegerType((int) $key->value)
                    : new ConstantStringType($key instanceof PHPStan\PhpDocParser\Ast\Type\IdentifierTypeNode ? $key->name : $key->value);
                $valueType = $nodeResolver->resolve($item->valueType, new PHPStan\Analyser\NameScope(null, []));
                $keyValue = $keyType->getValue();
                $items[] = [is_int($keyValue) ? ['int', $keyValue] : ['string', bin2hex($keyValue)], bin2hex($valueType->getValue()), $item->optional];
                $builder->setOffsetValueType($keyType, $valueType, $item->optional);
            }
            $row['items'] = $items;
            $row['builder'] = shapeKeyMetadata($builder->getArray());
        }
        $numericRows[] = $row;
    }
    $numericLateRows = [];
    foreach ([
        'array{K: "template", "\x31": "one", "01": "leading"}',
        'array{"\x31": "one", K?: "template", "01": "leading"}',
        'array{K?: "template", 1: "one", ...<K|2, "extra">}',
    ] as $input) {
        foreach (['1', '01', '-1', "\xff", "\xfe", "\xef\xbf\xbd"] as $replacement) {
            $type = $resolver->resolve($input, $scope);
            $resolved = TemplateTypeHelper::resolveTemplateTypes($type, new TemplateTypeMap(['K' => new ConstantStringType($replacement)]), TemplateTypeVarianceMap::createEmpty(), TemplateTypeVariance::createStatic());
            $numericLateRows[] = ['source_hex' => bin2hex($input), 'replacement' => bin2hex($replacement), 'resolved' => shapeKeyMetadata($resolved)];
        }
    }
    $rawInputs = ["'\xff'", "\"\xfe\"", "'\xef\xbf\xbd'", "'\xff\\xff'", "\"\xff\\xff\"",
        "array{'\xff': 'first', '\xfe': 'second', '\xef\xbf\xbd': 'third'}",
        "array{'\xff': 'first', \"\\xff\": 'last', '\xfe': 'other'}",
        'array{"\x31": "first", "\x30\x31": "zero", bare: "bare", 1: "last"}',
    ];
    $rawRows = [];
    foreach ($rawInputs as $input) {
        $type = $resolver->resolve($input);
        $parsed = $typeParser->parse(new PHPStan\PhpDocParser\Parser\TokenIterator($lexer->tokenize($input)));
        $rawRows[] = ['source_hex' => bin2hex($input), 'parsed' => parsedBytes($parsed), 'value' => $type instanceof ConstantStringType ? bin2hex($type->getValue()) : shapeKeys($type)];
    }
    $prefixRows = [];
    foreach (["array{'\xff': int} \$x", "'\xfe' trailing", "array{'\xff': int}\r\nrest"] as $input) {
        $tokens = new PHPStan\PhpDocParser\Parser\TokenIterator($lexer->tokenize($input));
        $typeParser->parse($tokens);
        $prefixRows[] = ['source_hex' => bin2hex($input), 'offset' => $tokens->currentTokenOffset()];
    }
    $errorRows = [];
    foreach (["'\xff' '\xfe'", "array{'\xff': int, '\xfe'} trailing"] as $input) {
        try {
            $resolver->resolve($input);
            throw new LogicException('Expected a parser error');
        } catch (PHPStan\PhpDocParser\Parser\ParserException $e) {
            $errorRows[] = ['source_hex' => bin2hex($input), 'message' => $e->getMessage()];
        }
    }
    $docRows = [];
    foreach (["/**\r\n * @param array{\r\n * '\xff': 'first',\r\n * '\xfe': 'second',\r\n * '\xef\xbf\xbd': 'third'\r\n * } \$x\r\n * @return '\xff'\r\n */", "/** @var '\xfe' \$x */"] as $input) {
        $doc = $docParser->parse(new PHPStan\PhpDocParser\Parser\TokenIterator($lexer->tokenize($input)));
        $tags = [];
        foreach ($doc->children as $tag) {
            $type = $nodeResolver->resolve($tag->value->type, new PHPStan\Analyser\NameScope(null, []));
            $tags[] = [$tag->name, $type instanceof ConstantStringType ? bin2hex($type->getValue()) : shapeKeys($type)];
        }
        $docRows[] = ['source_hex' => bin2hex($input), 'tags' => $tags];
    }
    $rawLateRows = [];
    $input = "array{K: 'template', '\xff': 'ff', '\xfe': 'fe', '\xef\xbf\xbd': 'replacement'}";
    foreach (["\xff", "\xfe", "\xef\xbf\xbd", '1'] as $replacement) {
        $type = $resolver->resolve($input, $scope);
        $resolved = TemplateTypeHelper::resolveTemplateTypes($type, new TemplateTypeMap(['K' => new ConstantStringType($replacement)]), TemplateTypeVarianceMap::createEmpty(), TemplateTypeVariance::createStatic());
        $rawLateRows[] = ['source_hex' => bin2hex($input), 'replacement' => bin2hex($replacement), 'entries' => shapeKeys($resolved)];
    }
    echo json_encode(['values' => $rows, 'pairs' => $pairs, 'escapes' => $escapes, 'identity_neighbors' => $neighbors, 'callable_neighbors' => $callableNeighbors, 'shape_keys' => $shapeRows, 'late_shape_keys' => $lateRows, 'raw_types' => $rawRows, 'raw_prefixes' => $prefixRows, 'raw_errors' => $errorRows, 'raw_docs' => $docRows, 'raw_late_shape_keys' => $rawLateRows, 'numeric_shape_keys' => $numericRows, 'numeric_late_shape_keys' => $numericLateRows], JSON_PRETTY_PRINT | JSON_INVALID_UTF8_SUBSTITUTE | JSON_THROW_ON_ERROR), "\n";
} finally {
    $items = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root, FilesystemIterator::SKIP_DOTS), RecursiveIteratorIterator::CHILD_FIRST);
    foreach ($items as $item) {
        $item->isDir() ? rmdir($item->getPathname()) : unlink($item->getPathname());
    }
    rmdir($root);
}
