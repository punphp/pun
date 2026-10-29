<?php
namespace RefinedPreparedCoalesce;

function observe(): void {}

/** @param array{bar?: int} $items */
function missing(array $items): void {
    $items['foo'] ??= 5; // input: failed read
    \PHPStan\dumpType($items['foo']);
    observe(); // holders: $items['foo']
}

/** @param array{foo?: int} $items */
function optional(array $items): mixed {
    $items['foo'] ??= 5;
    \PHPStan\dumpType($items['foo']);
    return $items;
}

/** @param array{foo: int} $items */
function required(array $items): mixed {
    $items['foo'] ??= 5;
    \PHPStan\dumpType($items['foo']);
    return $items;
}

/** @param array{foo: ?int} $items */
function nullable(array $items): mixed {
    $items['foo'] ??= 5;
    \PHPStan\dumpType($items['foo']);
    return $items;
}

/** @param array{bar?: int} $items */
function missingParent(array $items): mixed {
    $items['foo']['leaf'] ??= 5;
    \PHPStan\dumpType($items['foo']['leaf']);
    return $items;
}

/** @param array{foo: array{bar?: int}} $items */
function missingLeaf(array $items): mixed {
    $items['foo']['leaf'] ??= 5;
    \PHPStan\dumpType($items['foo']['leaf']);
    return $items;
}

/** @param array{foo?: array{leaf: int}} $items */
function optionalParent(array $items): mixed {
    $items['foo']['leaf'] ??= 5;
    \PHPStan\dumpType($items['foo']['leaf']);
    return $items;
}

/** @param array{int} $items */
function missingListOffset(array $items): mixed {
    $items[1] ??= 5;
    \PHPStan\dumpType($items[1]);
    return $items;
}

/** @param array{int} $items */
function negativeListOffset(array $items): mixed {
    $items[-1] ??= 6;
    \PHPStan\dumpType($items[-1]);
    return $items;
}

/** @param array{int} $items */
function stringListOffset(array $items): mixed {
    $items['other'] ??= 7;
    \PHPStan\dumpType($items['other']);
    return $items;
}

/** @param array<string, int> $items */
function openArray(array $items, string $key): mixed {
    $items['foo'] ??= 5;
    \PHPStan\dumpType($items['foo']);
    $items[$key] ??= 6;
    \PHPStan\dumpType($items[$key]);
    return $items;
}

function existingError(): void {
    $items = ['foo' => missingValue()];
    $items['foo'] ??= 5; // input: failed read
    observe(); // holders: $items['foo']
}

function indexOnce(): void {
    $items = [];
    $index = 0;
    $items[$index++] ??= 5; // input: failed read
    \PHPStan\dumpType($items[0]);
    \PHPStan\dumpType($index);
    observe(); // holders: $items[$index++] | $items[0] | $index
}
