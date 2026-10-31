<?php

namespace OffsetVirtualWriteScope;

/**
 * @template T of array<string, int>|string
 * @param T $value
 */
function repeated($value): void
{
    \PHPStan\dumpType($value);
    unset($value['left']);
    \PHPStan\dumpType($value);
    unset($value['right']);
    \PHPStan\dumpType($value);
}
