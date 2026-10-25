<?php

namespace ArraySlice;

use function PHPStan\dumpType;

function shapes(bool $preserve, int $offset, ?int $length): void
{
    dumpType(array_slice([], 0));
    dumpType(array_slice([10, 20, 30, 40], 1, 2));
    dumpType(array_slice([10, 20, 30, 40], 1, 2, true));
    dumpType(array_slice([10, 20, 30, 40], -2, null, true));
    dumpType(array_slice([10, 20, 30, 40], -2, 1));
    dumpType(array_slice([10, 20, 30, 40], 0, -1));
    dumpType(array_slice([10, 20, 30, 40], 1, -1));
    dumpType(array_slice([10, 20, 30, 40], -3, -1, true));
    dumpType(array_slice([10, 20, 30, 40], -1, -1));
    dumpType(array_slice([10, 20, 30, 40], -99, 2));
    dumpType(array_slice([10, 20, 30, 40], 99));
    dumpType(array_slice([10, 20, 30, 40], 0, -99));
    dumpType(array_slice([10, 20, 30, 40], 0, 99));
    dumpType(array_slice([10, 20, 30, 40], 1, 2, $preserve));
    dumpType(array_slice([10, 20, 30, 40], $offset));
    dumpType(array_slice([10, 20, 30, 40], 0, $length));
    dumpType(array_slice([10, 20, 30, 40], $offset, 0));
    dumpType(array_slice(['a' => 10], 0));
    dumpType(array_slice(['a' => 10], 1));
    dumpType(array_slice(['a' => 10], $offset));
    dumpType(array_slice([4 => 10], 0));
    dumpType(array_slice([4 => 10], 0, null, true));
    dumpType(array_slice([4 => 10], 0, null, $preserve));
    dumpType(array_slice(array: [10, 20, 30], length: 1, offset: 1));
}
