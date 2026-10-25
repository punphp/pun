<?php

namespace UnsetOffsetMetadata;

/** @param array{...<int, string>} $array */
function emptyOpenShape(array $array): void
{
    \PHPStan\dumpType($array);
    unset($array[0]);
    \PHPStan\dumpType($array);
}

/** @param array{a: int, ...<int, string>} $array */
function shapeCopies(array $array): void
{
    $copy = $array;
    unset($copy['a']);
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($array);
    unset($copy[0]);
    \PHPStan\dumpType($copy);
    $array = ['new' => 7];
    \PHPStan\dumpType($array);
}

/** @param array<int, string> $array
 * @param 0|5 $union
 */
function integerRefinement(array $array, int $union, int $unknown): void
{
    if (isset($array[5])) {
        \PHPStan\dumpType($array);
        unset($array[0]);
        \PHPStan\dumpType($array);
        $copy = $array;
        unset($copy[$union]);
        \PHPStan\dumpType($copy);
        unset($array[$unknown]);
        \PHPStan\dumpType($array);
        unset($array[5]);
        \PHPStan\dumpType($array);
    }
}

/** @param array<string, int|null> $array
 * @param 'a'|'other' $union
 */
function stringRefinement(array $array, string $union): void
{
    if (isset($array['a'])) {
        unset($array['']);
        \PHPStan\dumpType($array);
        unset($array[$union]);
        \PHPStan\dumpType($array);
        unset($array['a']);
        \PHPStan\dumpType($array);
    }
}

/** @param array<int, int|string> $array */
function asymmetricNarrowing(array $array): void
{
    if (isset($array[5]) && is_string($array[5])) {
        unset($array[0]);
        \PHPStan\dumpType($array);
        \PHPStan\dumpType($array[5]);
    }
}

/** @param array{a: int, ...<int, string>} $array */
function shapeJoin(array $array, bool $choose): void
{
    $other = $array;
    unset($array['a']);
    $joined = $choose ? $array : $other;
    \PHPStan\dumpType($joined);
    $reverse = $choose ? $other : $array;
    \PHPStan\dumpType($reverse);
}

/** @template T of array<int, string>
 * @param T $array
 */
function templateRefinement(array $array): void
{
    if (isset($array[5])) {
        unset($array[0]);
        \PHPStan\dumpType($array);
    }
}

/** @param array{a: int, ...<string, string>} $array */
function emptyStringShape(array $array): void
{
    unset($array['a']);
    \PHPStan\dumpType($array);
}

function localAnnotations(): void
{
    /** @var array{a: int, ...<int, string>} $shape */
    $shape = [];
    unset($shape['a']);
    \PHPStan\dumpType($shape);

    /** @var array{...<int, string>} $generic */
    $generic = [];
    unset($generic[0]);
    \PHPStan\dumpType($generic);
}

/** @param array<mixed, string> $array */
function mixedKeyCopies(array $array, int $runtime): void
{
    unset($array[0]);
    \PHPStan\dumpType($array);
    $copy = $array;
    unset($copy[7]);
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($array);
    unset($array[$runtime]);
    \PHPStan\dumpType($array);
    unset($copy['']);
    \PHPStan\dumpType($copy);
}
