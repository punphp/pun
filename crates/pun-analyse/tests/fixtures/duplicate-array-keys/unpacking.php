<?php

namespace UnpackedDuplicateKeys;

/**
 * @param array<string, int> $strings
 * @param iterable<string, int> $iterableStrings
 * @param list<int> $unknown
 */
function unknownKeys(array $strings, iterable $iterableStrings, array $unknown, mixed $mixed): void
{
    $a = [...$strings, 'x', 0 => 'duplicate'];
    $b = [3 => 'first', ...$strings, 'x', 4 => 'duplicate'];
    $c = [...$iterableStrings, 'x', 0 => 'duplicate'];
    $d = [...$unknown, 5 => 'explicit', 'unknown', 6 => 'not duplicate'];
    $e = [...$mixed, 5 => 'explicit', 'unknown', 6 => 'not duplicate'];
    $f = [...$unknown, ...['x' => 1], 'x' => 2];
}

function literals(): void
{
    $a = [...[1, 2], 0 => 'duplicate'];
    $b = [7 => 'first', ...[1, 2], 8 => 'duplicate', 'implicit', 10 => 'duplicate'];
    $c = [...['s' => 1], 's' => 'duplicate'];
    $d = ['s' => 'first', ...(['s' => 1])];
    $e = [1, ...[], 1 => 'not duplicate', 0 => 'duplicate'];
    $f = [...['2' => 1, '02' => 2, 3 => 3], 0 => 'duplicate', 1 => 'duplicate'];
    $g = array(...array('s' => 1), 's' => 'duplicate');
    $h = [...[4 => 1, 4 => 2], 1 => 'not duplicate'];
}

/** @return array{s: int, 9: int, 2: int} */
function shape(): array
{
    return ['s' => 1, 9 => 2, 2 => 3];
}

/** @param array{s: int, 9: int, 2: int} $shape */
function nonliteralStringOverrides(array $shape): void
{
    $local = ['s' => 1, 9 => 2, 2 => 3];
    $a = [...$shape, 's' => 'override', 0 => 'duplicate', 1 => 'duplicate'];
    $b = ['s' => 'first', ...$local, 's' => 'duplicate explicit keys'];
    $c = [...shape(), 's' => 'override', 0 => 'duplicate', 1 => 'duplicate'];
    $d = [5 => 'first', ...$local, 'implicit', 8 => 'duplicate'];
}

/**
 * @param array{0?: int} $optional
 * @param array{a: int, ...<int, int>} $open
 * @param array{a: int}|array{0: int, 1: int} $alternatives
 * @param array{a: int}|array<string, int> $shapeOrStrings
 * @param array{a: int}|false $notAlwaysArray
 */
function uncertainShapes(array $optional, array $open, array $alternatives, array $shapeOrStrings, array|false $notAlwaysArray): void
{
    $a = [...$optional, 5 => 'explicit', 'unknown', 6 => 'not duplicate'];
    $b = [...$open, 5 => 'explicit', 'unknown', 6 => 'not duplicate'];
    $c = [...$alternatives, 5 => 'explicit', 'unknown', 6 => 'not duplicate'];
    $d = [...$shapeOrStrings, 'implicit', 0 => 'not duplicate'];
    $e = [...$notAlwaysArray, 'implicit', 0 => 'not duplicate'];
}

/**
 * @template T of array{int, int}
 * @param T $list
 * @param callable&array $callable
 */
function templateAndCallable(array $list, array $callable): void
{
    $a = [...$list, 0 => 'duplicate'];
    $b = [...$callable, 1 => 'duplicate'];
}

/** @param array<string, int> $strings */
function overflow(array $strings, int $unknown): void
{
    $a = [9223372036854775806 => 1, ...[2], 9223372036854775807 => 'duplicate'];
    $b = [9223372036854775806 => 1, ...[2, 3], 9223372036854775807 => 'not duplicate'];
    $c = [9223372036854775807 => 1, ...[2], 'implicit', 0 => 'not duplicate'];
    $d = [9223372036854775807 => 1, ...['s' => 1], 's' => 'duplicate'];
    $e = [9223372036854775807 => 1, ...$strings, 9223372036854775807 => 'duplicate'];
    $f = [$unknown => 1, ...[2], 0 => 'not duplicate'];
}

function unpackedFirstOccurrence(): void
{
    $a = [
        ...[
            's' => 1,
        ],
        's' => 2,
    ];
}

/**
 * @param array{a?: int} $optionalStrings
 * @param array{a: int, ...<string, int>} $openStrings
 * @param array{int, int, ...<never, never>} $explicitlySealed
 */
function sealedness(array $optionalStrings, array $openStrings, array $explicitlySealed): void
{
    $a = [...$optionalStrings, 'implicit', 0 => 'not duplicate'];
    $b = [...$openStrings, 'implicit', 0 => 'not duplicate'];
    $c = [...$explicitlySealed, 0 => 'duplicate'];
    $d = [...[], 'implicit', 0 => 'duplicate'];
}

/** @param list<int> $unknown */
function unknownIndexSkipsEvenStringUnpacks(array $unknown): void
{
    $a = ['s' => 0, ...$unknown, ...['s' => 1], 's' => 2];
}

/**
 * @template T of (callable&array)
 * @param T&array $callback
 */
function genericCallableArray(array $callback): void
{
    $a = [...$callback, 1 => 'duplicate'];
}

/**
 * @template T of array
 * @param T&callable $callback
 */
function templateArrayWithCallableAccessory(array $callback): void
{
    $a = [...$callback, 1 => 'duplicate'];
}
