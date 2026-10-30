<?php
namespace RefinedWrittenCoalesce;

function observe(): void {}

function unknownRead($items): void {
    $items['foo'] ?? 5;
    observe(); // holders: $items['foo']
}

function unknownWrite($items): void {
    $items['foo'] ??= 5; // input: read
    observe(); // holders: $items['foo']
    \PHPStan\dumpType($items['foo']);
}

function errorRead(): void {
    $items = missingReadReceiver();
    $items['foo'] ?? 5;
    observe(); // holders: $items['foo']
}

function errorWrite(): void {
    $items = missingWriteReceiver();
    $items['foo'] ??= 5; // input: read
    observe(); // holders: $items['foo']
    \PHPStan\dumpType($items['foo']);
}

function nullableWrite($items): void {
    $items['foo'] ??= null;
    observe(); // holders: $items['foo']
    \PHPStan\dumpType($items['foo']);
}

/** @param array{foo?: int} $items */
function optionalRead(array $items): void {
    $items['foo'] ?? 5;
    observe(); // holders: $items['foo']
}

/** @param array{foo?: int} $items */
function optionalWrite(array $items): void {
    $items['foo'] ??= 5;
    observe(); // holders: $items['foo']
    \PHPStan\dumpType($items['foo']);
}

/** @param array<string, int> $items */
function openRead(array $items): void {
    $items['foo'] ?? 5;
    observe(); // holders: $items['foo']
}

/** @param array<string, int> $items */
function openWrite(array $items, string $key): void {
    $items['foo'] ??= 5;
    observe(); // holders: $items['foo']
    $items[$key] ??= 6;
    observe(); // holders: $items[$key]
    \PHPStan\dumpType($items[$key]);
}

function requiredError(): void {
    $items = ['foo' => missingValue()];
    $items['foo'] ??= 5; // input: failed read
    observe(); // holders: $items['foo']
}

function nestedUnknown($items): void {
    $items['parent']['leaf'] ??= 5;
    observe(); // holders: $items['parent']['leaf']
    \PHPStan\dumpType($items['parent']['leaf']);
}

function rawKey($items): void {
    $items['0'] ??= 5;
    observe(); // holders: $items['0'] | $items[0]
}

function indexOnce($items): void {
    $index = 0;
    $items[$index++] ??= 5;
    observe(); // holders: $items[$index++] | $items[0] | $index
}

function stringRead(string $text): void {
    $text[0] ?? 'x';
    observe(); // holders: $text[0]
}

function stringWrite(string $text): void {
    $text[0] ??= 'x';
    observe(); // holders: $text[0]
    \PHPStan\dumpType($text[0]);
}

function unknownReadValue($items): void {
    \PHPStan\dumpType($items['foo'] ?? 5);
    observe(); // holders: $items['foo']
}

function errorReadValue(): void {
    $items = missingValueReceiver();
    \PHPStan\dumpType($items['foo'] ?? 5);
    observe(); // holders: $items['foo']
}
