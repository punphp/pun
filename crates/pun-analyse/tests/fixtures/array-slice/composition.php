<?php

namespace ArraySlice;

use function PHPStan\dumpType;

/**
 * An unsupported refinement must retain the native array return, not replace it with never/error. Its length
 * remains a non-negative integer, and ordinary arrays can still be narrowed by an outer zero-length slice.
 *
 * @template T of array<int, string>
 * @param T $items
 */
function genericComposition(array $items): void
{
    dumpType(count(array_slice($items, 1)));
}

/** @param array{0?: 'optional'} $items */
function optionalComposition(array $items): void
{
    dumpType(array_slice(array_slice($items, 0, 1), 0, 0));
}

function orderedComposition(): void
{
    dumpType(array_slice(array_slice(['b' => 2, 'a' => 1], 1), 0, 0));
    dumpType(array_slice(array_slice([10, 20, 30], -2), 0, 0));
}
