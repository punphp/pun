<?php
namespace CompoundAssignmentReads;

/** @param array<string, int> $array */
function generic(array $array): mixed {
    $array['b'] += 'str'; // read: generic
    return $array;
}

/** @param array{b?: int} $array */
function optional(array $array): mixed {
    $array['b'] += 'str'; // read: optional
    return $array;
}

/** @param array{a: int} $array */
function sealed(array $array): mixed {
    $array['b'] += 'str'; // read: sealed
    return $array;
}

/** @param array{b: int} $array */
function known(array $array): mixed {
    $array['b'] += 'str'; // read: known
    return $array;
}

/** @param array<string, array<string, int>> $array */
function nested(array $array): mixed {
    $array['a']['b'] += 'str'; // read: nested
    return $array;
}

function malformed(int $array): mixed {
    $array['b'] += 'str'; // read: malformed
    return $array;
}

function unknown(mixed $array): mixed {
    $array['b'] += 'str'; // read: unknown
    return $array;
}

function missing(): mixed {
    $array['b'] += 'str'; // read: missing
    return $array;
}

/** @param array<string, int> $array */
function valid(array $array): mixed {
    $array['b'] += 1; // read: valid
    return $array;
}

function key(): string { return 'b'; }
function value(): string { return 'str'; }

/** @param array<string, int> $array */
function effects(array $array): mixed {
    $array[key()] += value(); // read: effects
    return $array;
}

class Box {
    public int $number = 1;
}

function property(Box $box): void {
    $box->number += 'str'; // read: property
}

/** @param array<string, int> $array */
function writeOnly(array $array): mixed {
    $array['b'] = 'str'; // write-only: no target read
    return $array;
}

/** @param array{0?: int} $array */
function optionalList(array $array): mixed {
    $array[0] += 'str'; // read: optional list
    return $array;
}

/** @param array{b?: int|null} $array */
function nullableOptional(array $array): mixed {
    $array['b'] += 'str'; // read: nullable optional
    return $array;
}

/** @param list<int> $array */
function genericList(array $array): mixed {
    $array[0] += 'str'; // read: generic list
    return $array;
}

/** @param array{b?: int} $array */
function coalesceOptional(array $array): mixed {
    $array['b'] ??= 'str'; // read: coalesce optional
    \PHPStan\dumpType($array['b']);
    return $array;
}

/** @param array{0?: int} $array */
function coalesceOptionalList(array $array): mixed {
    $array[0] ??= 'str'; // read: coalesce optional list
    \PHPStan\dumpType($array[0]);
    return $array;
}

function requiredLiteralList(int $value): mixed {
    $array = [$value];
    $array[0] += 'str'; // read: literal list
    return $array;
}

function optionalLiteralList(bool $condition, int $value): mixed {
    $array = $condition ? [$value] : [];
    $array[0] += 'str'; // read: joined optional literal list
    return $array;
}
