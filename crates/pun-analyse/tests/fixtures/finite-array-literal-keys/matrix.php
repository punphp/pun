<?php

namespace FiniteArrayLiteralKeys;

function observe(array $value): void {}

/** @param 'left'|'right' $key */
function asymmetric(string $key): void
{
    observe([$key => 1]);
    observe(['left' => 'old', $key => 2]);
    observe([$key => 3, 'left' => 'last']);
    observe([$key => 4, $key => 'again']);
    observe(['left' => 5, 'right' => false, $key => 'both']);
    observe([$key => 6, 'left' => 7, $key => true]);
    observe([$key => 8, 'fixed' => null, 'tail']);
}

/** @param 'left'|'middle'|'right' $key
 *  @param 'left'|'right' $overlap
 */
function three(string $key, string $overlap): void
{
    observe(['middle' => false, $key => 'three', 'right' => 9]);
    observe([$key => 1, $overlap => 'second']);
    observe([$overlap => 1, $key => 'second']);
}

/** @param 0|2 $key */
function integers(int $key): void
{
    observe([$key => 'key', 'append', 'next']);
    observe([$key => 'key', 1 => 'fixed', 'append']);
    observe([$key => 'key', 2 => 'fixed', 'append']);
    observe([0 => 'old', $key => 'key', 'append']);
    observe([$key => 'key', 0 => 'fixed', 'append', 'next']);
    observe([$key => 'key', 7 => 'fixed', 'append']);
}

/** @param '0'|'2'|'02'|'+2' $key */
function conversion(string $key): void
{
    observe([$key => 'value', 'append']);
    observe([0 => 1, '02' => false, $key => 'value', 2 => 'two', 'tail']);
}

/** @param -2|-1 $key */
function negatives(int $key): void
{
    observe([$key => 1, 'append']);
}

/** @param 1|9223372036854775807 $key */
function integerMax(int $key): void
{
    observe([$key => 1, 'append']);
}

/** @param int<2,4> $key */
function rangeKeys(int $key): void
{
    observe([$key => 1, 'append']);
}

/** @param int<0,128> $key */
function rangeLimit(int $key): void
{
    observe([$key => 1]);
}

/** @param int<0,129> $key */
function beyondRangeLimit(int $key): void
{
    observe([$key => 1]);
}

/** @param int<0,127>|int<256,382> $key */
function scalarLimitBelow(int $key): void
{
    observe([$key => 1]);
}

/** @param int<0,127>|int<256,383> $key */
function scalarLimitAt(int $key): void
{
    observe([$key => 1]);
}

/** @param int<0,128> $a
 *  @param int<256,384> $b
 */
function totalLimit(int $a, int $b): void
{
    observe([$a => 1, $b => 2, 'tail' => 3]);
}

/** @param string $key */
function openControl(string $key): void
{
    observe(['left' => 1, $key => 2]);
    observe([$key => 1, 'left' => 2]);
}

/** @param 'left'|'right' $key */
function finiteThenOpen(string $key, string $open): void
{
    observe([$key => 1, $open => 2, 'tail' => 3]);
}

/** @param 2|'2' $key */
function collapsedConversion(int|string $key): void
{
    observe([$key => 'key', 'append']);
}

/** @param 'left'|'right' $key
 *  @param 0|2 $integer
 *  @param array{left?: 1, right?: 2} $optional
 */
function unpacks(string $key, int $integer, array $optional): void
{
    observe([...[$key => 1]]);
    observe(['left' => 'old', ...[$key => 1], 'right' => false]);
    observe([...[$integer => 1], 'append']);
    observe([$integer => 1, ...['append'], 'next']);
    observe([...$optional]);
    observe(['left' => false, ...$optional]);
}

/** @param int<0,128> $a
 *  @param int<256,384> $b
 */
function unpackLimit(int $a, int $b): void
{
    observe([...[$a => 1], ...[$b => 2], 'tail' => 3]);
}

function literalControls(): void
{
    observe(['left' => 1, 'right' => false]);
    observe([1, 2, 3]);
    observe([0 => 'low', 2 => 'high', 'tail']);
}

/** @param 'left'|'right' $key
 *  @param 0|2 $integer
 *  @param list<string> $list
 *  @param array<string, int> $open
 */
function unknownUnpacks(string $key, int $integer, array $list, array $open): void
{
    observe([$key => 1, ...$list]);
    observe([$key => 1, ...$open]);
    observe([$key => 1, ...$list, 'tail']);
    observe([$integer => 1, ...$open, 'tail', 'next']);
    observe([...$open, $key => 1]);
    observe([...['left' => 'old'], $key => 1]);
}

/** @param int<0,127> $a
 *  @param int<256,383> $b
 */
function accumulatedSlotLimit(int $a, int $b): void
{
    observe([$a => 1, $b => 2]);
    observe([$a => 1, $b => 2, 'tail' => 3]);
}

/** @param 'left'|'right' $key */
function exhaustedAutoIndexes(string $key, string $open): void
{
    observe(['9223372036854775807' => 'max', $key => 1, 'ignored']);
    observe(['9223372036854775807' => 'max', $key => 1, $open => 2, 'ignored']);
}
