<?php

namespace ArrayMetadata;

use function PHPStan\dumpType;

/** @param array{z: int|string, gone: bool, a?: string, m: int} $items */
function survivingDeclarationKeys(array $items): void
{
    unset($items['gone']);
    assert(is_int($items['z']));
    dumpType($items);
    $copy = $items;
    unset($items['z']);
    dumpType($items);
    dumpType($copy);
    $items = ['replacement' => false];
    dumpType($items);
}

function survivingLiteralKeys(): void
{
    $items = ['z' => 1, 'gone' => false, 'a' => 2, 'm' => 3];
    $copy = $items;
    unset($items['gone']);
    dumpType($items);
    dumpType($copy);
    unset($items['z']);
    dumpType($items);
    unset($items['a'], $items['m']);
    dumpType($items);
}

/** @param 'a'|'b' $key */
function nestedWeakWrites(string $key): void
{
    $items = ['a' => ['leaf' => 1, 'keptA' => 3], 'b' => ['leaf' => 2, 'keptB' => 4]];
    unset($items[$key]['leaf']);
    dumpType($items);
    dumpType($items['a']);
    dumpType($items['b']);
}

/** @param array{0: int, 1: string, 2?: bool} $items */
function listAndOptionality(array $items): void
{
    unset($items[0]);
    dumpType($items);
}

/** @param array{z: int, a: string, m?: bool, ...<string, int|string|bool>} $items */
function unsealedDeclaration(array $items): void
{
    unset($items['a']);
    dumpType($items);
}

function legacyAndDuplicateKeys(): void
{
    $items = array('z' => 1, 'a' => 2, 'z' => 3, 'm' => 4);
    unset($items['a']);
    dumpType($items);
}
