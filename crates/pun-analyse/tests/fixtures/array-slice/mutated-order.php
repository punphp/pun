<?php

namespace ArraySlice;

use function PHPStan\dumpType;

/** @param array{z: mixed, gone: bool} $items */
function survivingShapeOrder(array $items): void
{
    unset($items['gone']);
    dumpType(array_slice($items, 0, null, true));
    dumpType(array_slice($items, 0, 0, true));
    dumpType(array_slice($items, 99, null, true));
}

function survivingLiteralOrder(): void
{
    $items = ['z' => 1, 'gone' => false];
    unset($items['gone']);
    dumpType(array_slice($items, 0, null, true));
    dumpType(array_slice($items, 0, 0, true));
    dumpType(array_slice($items, 99, null, true));
}

/** @param array{2: string, 1: bool} $items */
function numericKeyOrder(array $items): void
{
    unset($items[1]);
    dumpType(array_slice($items, 0, null, true));
}
