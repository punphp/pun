<?php

namespace ArraySlice;

use function PHPStan\dumpType;

function insufficient(): void
{
    array_slice();
    array_slice([]);
}

function invalidString(): void
{
    dumpType(array_slice('not an array', 0));
}

function invalidNull(): void
{
    dumpType(array_slice(null, 0));
}

function invalidInteger(): void
{
    dumpType(array_slice(123, 0));
}

/**
 * @param array<int, string> $items
 * @param true|0 $possiblyTrue
 */
function invalidPreserveKeys(array $items, int $integer, mixed $mixed, bool|int $possiblyTrue): void
{
    dumpType(array_slice($items, 1, null, 0));
    dumpType(array_slice($items, 1, null, 1));
    dumpType(array_slice($items, 1, null, null));
    dumpType(array_slice($items, 1, null, '1'));
    dumpType(array_slice($items, 1, null, $integer));
    dumpType(array_slice($items, 1, null, $mixed));
    dumpType(array_slice($items, 1, null, $possiblyTrue));
}
