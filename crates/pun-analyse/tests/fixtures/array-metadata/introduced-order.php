<?php

namespace ArrayMetadata;

use function PHPStan\dumpType;

/** @param array{z: 1, a: 2, ...<string, array<string, int>>} $items */
function introducedParentOrder(array $items): void
{
    unset($items['m']['gone'], $items['b']['gone']);
    dumpType($items);
    $copy = $items;
    dumpType([$items]);
    unset($items['m']);
    unset($items['m']['gone']);
    dumpType($items);
    dumpType($copy);
}

/** @param array{z: array{keep: 1, ...<string, array<string, int>>}, a: 2} $items */
function nestedIntroducedParentOrder(array $items): void
{
    unset($items['z']['m']['gone'], $items['z']['b']['gone']);
    dumpType($items['z']);
}

function refinedIntroducedParentOrder(): void
{
    $items = ['z' => 1, 'a' => 2];
    unset($items['m']['gone']);
    assert(is_null($items['m']));
    dumpType($items);
}

/** @param array{1: array{z: 1, a: 2}, 0: 3} $items */
function numericParentOrder(array $items): void
{
    unset($items[1]['z']);
    dumpType($items);
}

/** @param array{z: 1, a: 2}|null $items */
function nullableSourceOrder(?array $items): void
{
    unset($items['a']);
    dumpType($items);
}

/**
 * @param array{z: 1, a: 2, ...<string, array<string, int>>} $items
 * @param 'gone'|'other' $key
 */
function weakLeafKeepsKnownParentOrder(array $items, string $key): void
{
    unset($items['m'][$key], $items['b'][$key]);
    dumpType($items);
}

final class ArrayOrderGuard
{
    public function touch(): void {}
}

function introducedOrderAfterNativeMethod(ArrayOrderGuard $guard): void
{
    $items = ['z' => 1, 'a' => 2];
    unset($items['m']['gone']);
    assert(is_null($items['m']));
    $guard->touch();
    dumpType($items);
}
