<?php

namespace ArraySlice;

use function PHPStan\dumpType;

/**
 * @param array<int, bool> $ints
 * @param array<non-negative-int, mixed> $positive
 * @param array<string, int> $strings
 * @param array<int|string, string> $keys
 * @param list<string> $list
 * @param non-empty-array<int, string> $nonempty
 * @param non-empty-list<string> $nelist
 * @param positive-int $length
 */
function general(array $ints, array $positive, array $strings, array $keys, array $list, array $nonempty, array $nelist, bool $preserve, int $offset, ?int $size, int $length, mixed $mixed): void
{
    dumpType(array_slice($ints, 1));
    dumpType(array_slice($ints, -1, -2, false));
    dumpType(array_slice($ints, 0, null, true));
    dumpType(array_slice($ints, 1, 2, $preserve));
    dumpType(array_slice($positive, 0, 5));
    dumpType(array_slice($positive, $offset, $size, true));
    dumpType(array_slice($positive, 0, null, $preserve));
    dumpType(array_slice($strings, 1));
    dumpType(array_slice($strings, -2, -1, true));
    dumpType(array_slice($strings, $offset, $size, $preserve));
    dumpType(array_slice($keys, 1));
    dumpType(array_slice($keys, -1, null, true));
    dumpType(array_slice($keys, 0, 1, $preserve));
    dumpType(array_slice($list, 1));
    dumpType(array_slice($list, 0, 1, true));
    dumpType(array_slice($list, 1, 2, true));
    dumpType(array_slice($list, 0, null, $preserve));
    dumpType(array_slice($list, $offset, $size, $preserve));
    dumpType(array_slice($list, -2, -1));
    dumpType(array_slice($nonempty, 0));
    dumpType(array_slice($nonempty, 1));
    dumpType(array_slice($nonempty, 0, 1));
    dumpType(array_slice($nelist, 0));
    dumpType(array_slice($nelist, 0, 1));
    dumpType(array_slice($nelist, 0, $length, true));
    dumpType(array_slice($nelist, 1));
    dumpType(array_slice($nelist, 0, -1));
    dumpType(array_slice($ints, 1, 0));
    dumpType(array_slice($strings, 0, 0, true));
    dumpType(array_slice($mixed, 0));
    dumpType(array_slice($mixed, 0, 0));
    dumpType(array_slice(array: $positive, offset: 0, length: 5));
    dumpType(array_slice(preserve_keys: true, length: null, offset: 0, array: $list));
}

/** @param array<int, int>|array<string, bool> $arrays */
function alternatives(array $arrays): void
{
    dumpType(array_slice($arrays, 1));
}
