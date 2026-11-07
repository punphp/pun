<?php

declare(strict_types=1);

/*
 * parametersSchema validation as PHPStan 2.3.5 does it (ContainerFactory::validateParameters(), called through
 * reflection on NeonAdapter-loaded NEON), for schemas and values beyond PHPStan's own parametersSchema. Writes
 * crates/pun-phpstan/tests/fixtures/schema.json: {neon, messages} (ValidationException::getMessages()) or
 * {neon, exception} (anything else the schema building throws).
 *
 *   php scripts/export-schema-fixtures.php
 */

$root = dirname(__DIR__);
Phar::loadPhar($root.'/.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar', 'phpstan.phar');
require 'phar://phpstan.phar/vendor/autoload.php';

use PHPStan\DependencyInjection\ContainerFactory;
use PHPStan\DependencyInjection\NeonAdapter;

// [schema entries, parameter entries], one NEON line each (tab-indented under the section).
$cases = [
    [['a: schema(int(), nullable())'], ['a: null'], ['a: 1'], ['a: x']],
    [['a: schema(int())'], []],
    [['a: schema(5)']],
    [['a: schema()']],
    [['?a: int()'], [], ['a: x']],
    [['a: anyOf(1, 2, string())'], ['a: 3'], ['a: x'], ['a: 2']],
    [['a: anyOf(1, 2)'], ['a: 3'], []],
    [['a: schema(anyOf(1, 2), firstIsDefault())'], []],
    [['a: schema(anyOf(string(), int()), firstIsDefault())'], []],
    [['a: schema(anyOf(1, 2), nullable())'], ['a: null'], ['a: 5']],
    [['a: anyOf(listOf(int()), string())'], ['a: [x]'], ['a: 1.5']],
    [['a: anyOf(structure([b: int()]), int())'], ['a: {b: x}'], ['a: {c: 1}']],
    [['a: structure([b: int(), ?c: string()])'], ['a: {b: 1}'], ['a: {b: x, c: 5}'], ['a: {b: 1, d: 2}'], ['a: {b: 1, bb: 2}'], ['a: 5'], []],
    [['a: schema(structure([b: int()]), otherItems(string()))'], ['a: {b: 1, c: x}'], ['a: {b: 1, c: 1}']],
    [['a: schema(structure([b: int()]), otherItems())'], ['a: {b: 1, c: [1]}']],
    [['a: schema(structure([b: int()]), min(2))'], ['a: {b: 1}']],
    [['a: structure([b: structure([c: int()])])'], ['a: {b: {c: x}}'], ['a: {}']],
    [['a: structure(5)']],
    [['a: arrayOf(string())'], ['a: [x, 1]'], ['a: {k: x}'], ['a: x']],
    [['a: arrayOf(int(), string())'], ['a: {k: 1}'], ['a: [1]']],
    [['a: listOf(int())'], ['a: [1, x]'], ['a: {k: 1}']],
    [['a: listOf(structure([b: int()]))'], ['a: [{b: x}]']],
    [['a: arrayOf(listOf(string()))'], ['a: {k: [x, 1]}']],
    [['a: arrayOf()']],
    [['a: type("int|string")'], ['a: x'], ['a: 1.5']],
    [['a: type()'], ['a: 1']],
    [['a: int(5)'], [], ['a: 1']],
    [['a: string("x")'], [], ['a: 1']],
    [['a: schema(int(), min(1), max(3))'], ['a: 0'], ['a: 2'], ['a: 4']],
    [['a: schema(float(), min(1.5))'], ['a: 1.0'], ['a: 2.5']],
    [['a: schema(int(), max(3))'], ['a: 4']],
    [['a: schema(string(), min(2), max(3))'], ['a: x'], ['a: xyzw'], ['a: xy']],
    [['a: schema(unicode(), max(2))'], ['a: ééé'], ['a: éé']],
    [['a: schema(listOf(int()), min(1))'], ['a: []']],
    [['a: schema(string(), pattern("\\\\d+"))'], ['a: "12"'], ['a: "12a"'], ['a: "12\n"']],
    [['a: schema(arrayOf(int()), mergeDefaults(false))'], ['a: [1]']],
    [['a: schema(arrayOf(int()), mergeDefaults())'], ['a: [1]']],
    [['a: schema(listOf(int()), default([1, 2]))'], [], ['a: [3]'], ['a!: [3]']],
    [['a: schema(listOf(int()), default([1]), mergeDefaults(false))'], ['a: [3]']],
    [['a: schema(int(), required())'], [], ['a: 1']],
    [['a: schema(int(), required(false))'], []],
    [['a: schema(int(), default(3))'], []],
    [['a: schema(int(), default())'], []],
    [['a: schema(int(), deprecated("old"))'], ['a: 1']],
    [['a: schema(int(), fooBar())']],
    [['a: schema(anyOf(1), pattern("x"))']],
    [['a: bool()'], ['a: yes'], ['a: 1']],
    [['a: boolean()'], ['a: true']],
    [['a: integer()'], ['a: 1.0']],
    [['a: number()'], ['a: 1.5'], ['a: x']],
    [['a: scalar()'], ['a: [1]'], ['a: x']],
    [['a: null()'], ['a: null'], ['a: 0']],
    [['a: mixed()'], ['a: Foo(1)']],
    [['a: list()'], ['a: {k: 1}'], ['a: [1]']],
    [['a: array()'], ['a: 1']],
    [['a: object()'], ['a: Foo(1)'], ['a: 1']],
    [['a: type("?int")'], ['a: null'], ['a: x']],
    [['a: type("Nette\\\\DI\\\\Definitions\\\\Statement")'], ['a: Foo(1)'], ['a: x']],
    [['a: int()'], ['a: "a very long string value that will be truncated"'], ['a: "averyveryverylongstringwithoutspaces"'], ['a: [1, 2]'], ['a: Foo(1)'], ['a: 2024-01-02'], ['a: 1.5'], ['a: false'], ['a: null'], ['a: 1e100'], ['a: -0.0']],
    [['a: int()', 'b: string()', 'c: bool()'], ['a: x', 'b: 1', 'c: 2']],
    [['a: listOf(int())'], ['a: {k: x}']],
    [['getFoo: int()', 'isBar: int()'], ['foo: 1'], ['setFoo: 1'], ['bar: 1']],
    [['a: schema(int(), min(1e999))'], ['a: 1']],
    [['a: listOf("int")'], ['a: [x]']],
    [['a: arrayOf("string", "int")'], ['a: {k: x}'], ['a: [x]']],
    [['a: listOf(5)']],
    [['a: structure([b: "int"])'], ['a: {b: x}']],
    [['a: anyOf([int()])'], ['a: [1]']],
    [['a: schema(structure([b: int()]), pattern("x"))']],
    [['a: structure([b: int()])'], ['a!: {b: 1}'], ['a!: {b: x}']],
    [['a: arrayOf(int())'], ['a: null']],
    [['?a: structure([b: int()])'], []],
    [['?a: schema(structure([b: int()]), required(false))'], []],
    [['"": int()', 'b: int()'], ['b: 1', '"": x']],
    [['a: anyOf({x: 1, y: 2})'], ['a: {y: 2, x: 1}'], ['a: {x: 1, y: 2}']],
    [['abc: int()'], ['ab: 1'], ['abcdefghijk: 1'], ['x: 1']],
];

function neonOf(array $schema, array $params): string
{
    return "parametersSchema:\n\t__validate: bool()\n".implode('', array_map(fn ($l) => "\t$l\n", $schema)).'parameters:'.($params === [] ? " []\n" : "\n".implode('', array_map(fn ($l) => "\t$l\n", $params)));
}

$factory = (new ReflectionClass(ContainerFactory::class))->newInstanceWithoutConstructor();
$validate = new ReflectionMethod(ContainerFactory::class, 'validateParameters');
$tmp = $root.'/target-lintcfg/tmp/schema.neon';
$out = [];
foreach ($cases as $case) {
    $schema = array_shift($case);
    foreach ($case === [] ? [[]] : $case as $params) {
        $neon = neonOf($schema, $params);
        file_put_contents($tmp, $neon);
        $data = (new NeonAdapter([]))->load($tmp);
        $r = ['neon' => $neon];
        try {
            $validate->invoke($factory, ($data['parameters'] ?? []) + ['__validate' => true], $data['parametersSchema']);
            $r['messages'] = [];
        } catch (_PHPStan_95d9e8b05\Nette\Schema\ValidationException $e) {
            $r['messages'] = $e->getMessages();
        } catch (Throwable $e) {
            $r['exception'] = $e->getMessage();
        }
        $out[] = $r;
    }
}
unlink($tmp);
file_put_contents($root.'/crates/pun-phpstan/tests/fixtures/schema.json', json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)."\n");
echo count($out), " cases\n";
