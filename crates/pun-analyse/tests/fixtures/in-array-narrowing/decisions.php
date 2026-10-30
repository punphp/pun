<?php

namespace InArrayNarrowing;

function finiteBoolean(bool $needle): bool
{
    return in_array($needle, [true, false], true);
}

function reversedBoolean(bool $needle): bool
{
    return in_array($needle, [false, true], true);
}

function singleBoolean(bool $needle): bool
{
    return in_array($needle, [true], true);
}

function singleTrue(true $needle): bool
{
    return in_array($needle, [true], true);
}

function singleFalse(false $needle): bool
{
    return in_array($needle, [true], true);
}

function constantNeedle(): bool
{
    return in_array(true, [true, false], true);
}

function assignedBoolean(bool $needle): bool
{
    $haystack = [true, false];
    return in_array($needle, $haystack, true);
}

/** @param 1|2 $needle */
function finiteIntegers(int $needle): bool
{
    return in_array($needle, [1, 2], true);
}

/** @param true|1 $needle */
function finiteMixed(bool|int $needle): bool
{
    return in_array($needle, [true, 1], true);
}

/** @param true|1 $needle */
function reversedMixed(bool|int $needle): bool
{
    return in_array($needle, [1, true], true);
}

/** @param 1|'one' $needle */
function finiteScalars(int|string $needle): bool
{
    return in_array($needle, [1, 'one'], true);
}

function nullableBoolean(?bool $needle): bool
{
    return in_array($needle, [null, true, false], true);
}

/** @param array{true, false} $haystack */
function shapedBoolean(bool $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

/** @param array{0: true, 1?: false} $haystack */
function optionalBoolean(bool $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

/** @param array{0: true, 1?: false} $haystack */
function optionalTrue(true $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

/** @param array{true, false}|array{false, true, 7} $haystack */
function unionBoolean(bool $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

/** @param array{true}|array{false} $haystack */
function unionIncomplete(bool $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

/** @param non-empty-array<true> $haystack */
function generalTrue(true $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

/** @param non-empty-array<true> $haystack */
function generalFalse(false $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

/** @param non-empty-array<bool> $haystack */
function generalBoolean(bool $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}

function disjointBoolean(bool $needle): bool
{
    return in_array($needle, [1, 'one'], true);
}

function emptyBoolean(bool $needle): bool
{
    return in_array($needle, [], true);
}

function looseBoolean(bool $needle): bool
{
    return in_array($needle, [true, false]);
}

function falseStrictBoolean(bool $needle): bool
{
    return in_array($needle, [true, false], false);
}

function unpackedBoolean(bool $needle): bool
{
    return in_array($needle, [...[true, false]], true);
}

function assignedNeedle(bool $needle): bool
{
    return in_array($copy = $needle, [true, false], true);
}

function unrelated(string $value): bool
{
    return is_int($value);
}

function singleFalseItem(bool $needle): bool
{
    return in_array($needle, [false], true);
}

function typedTruePair(true $needle): bool
{
    return in_array($needle, [true, false], true);
}

function literalInteger(): bool
{
    return in_array(1, [1, 2], true);
}

/** @param 1 $needle */
function typedInteger(int $needle): bool
{
    return in_array($needle, [1, 2], true);
}

function itemVariables(bool $needle, true $first, false $second): bool
{
    return in_array($needle, [$first, $second], true);
}

function typedItemVariables(true $needle, true $first, false $second): bool
{
    return in_array($needle, [$first, $second], true);
}

function repeatedItem(true $needle, true $item): bool
{
    return in_array($needle, [$item, $item], true);
}

function falseLiteralPair(): bool
{
    return in_array(true, [1, 2], true);
}

function nestedAssignment(bool $needle): bool
{
    return in_array($outer = $inner = $needle, [true, false], true);
}

/** @param 1.0|2.0 $needle */
function finiteFloats(float $needle): bool
{
    return in_array($needle, [1.0, 2.0], true);
}

/** @param 1|2 $needle */
function keyedIntegers(int $needle): bool
{
    return in_array($needle, ['left' => 1, 'right' => 2], true);
}

function legacyBoolean(bool $needle): bool
{
    return in_array($needle, array(true, false), true);
}

function nonfiniteItems(string $first, string $second): bool
{
    return in_array(1, [$first, $second], true);
}

function dependentItems(string $first, string $second): bool
{
    return in_array(1, [$first . 'a', $second . 'b'], true);
}

/** @param array{value: never} $haystack */
function impossibleValue(bool $needle, array $haystack): bool
{
    return in_array($needle, $haystack, true);
}
