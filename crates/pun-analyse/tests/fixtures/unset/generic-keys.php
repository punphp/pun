<?php

namespace UnsetGenericKeys;

/** @param array<int, string> $array */
function integers(array $array): void
{
    unset($array[0]);
    \PHPStan\dumpType($array);
    unset($array[-2]);
    \PHPStan\dumpType($array);
    unset($array[0]);
    \PHPStan\dumpType($array);
    unset($array['missing']);
}

/** @param array<int<0, 2>, int> $array */
function endpoints(array $array): void
{
    unset($array[1]);
    \PHPStan\dumpType($array);
    unset($array[0]);
    \PHPStan\dumpType($array);
    unset($array[2]);
    \PHPStan\dumpType($array);
    unset($array[2]);
}

/** @param array<'a'|'b'|3, int> $array */
function finiteKeys(array $array): void
{
    unset($array['a']);
    \PHPStan\dumpType($array);
    unset($array[3]);
    \PHPStan\dumpType($array);
    unset($array['b']);
    \PHPStan\dumpType($array);
}

/** @param array<string, int> $array */
function strings(array $array): void
{
    unset($array['']);
    \PHPStan\dumpType($array);
    unset($array['ordinary']);
    \PHPStan\dumpType($array);
    unset($array['01']);
    \PHPStan\dumpType($array);
    unset($array[0]);
}

/** @param array<1|2, int> $array
 * @param 1|2 $union
 */
function nonliteralOffsets(array $array, int $integer, int $union): void
{
    unset($array[$integer]);
    \PHPStan\dumpType($array);
    unset($array[$union]);
    \PHPStan\dumpType($array);
    unset($array[3]);
    \PHPStan\dumpType($array);
}

/** @template K of int
 * @param array<K, string> $array
 * @param K $key
 */
function templateKeys(array $array, int $key): void
{
    unset($array[0]);
    \PHPStan\dumpType($array);
    unset($array[-1]);
    \PHPStan\dumpType($array);
}

/** @template T of array<int, string>
 * @param T $array
 */
function templateArray(array $array, int $unknown): void
{
    unset($array[$unknown]);
    \PHPStan\dumpType($array);
    unset($array['missing']);
    \PHPStan\dumpType($array);
    unset($array[0]);
    \PHPStan\dumpType($array);
}

/** @param array<int|string, int> $explicit
 * @param array<array-key, int> $benevolent
 * @param array $implicit
 */
function mixedKeys(array $explicit, array $benevolent, array $implicit): void
{
    unset($explicit[0], $benevolent[0], $implicit[0]);
    \PHPStan\dumpType($explicit);
    \PHPStan\dumpType($benevolent);
    \PHPStan\dumpType($implicit);
}

/** @param array{z: int, ...<int, int>} $array */
function openShape(array $array): void
{
    unset($array[0]);
    \PHPStan\dumpType($array);
    unset($array['z']);
    \PHPStan\dumpType($array);
}

/** @param array<int, string> $array */
function narrowedArray(array $array): void
{
    if (isset($array[5])) {
        unset($array[0]);
        \PHPStan\dumpType($array);
    }
}

/** @param array<int<0, max>, int> $positive
 * @param array<int<min, 0>, int> $negative
 */
function limits(array $positive, array $negative): void
{
    unset($positive[9223372036854775807]);
    \PHPStan\dumpType($positive);
    unset($negative[-9223372036854775807 - 1]);
    \PHPStan\dumpType($negative);
}

/** @template T of array{a: int, ...<int, int>}
 * @param T $array
 */
function openTemplateDoesNotSubtractExtraKeys(array $array): void
{
    unset($array['a']);
    unset($array[0]);
    unset($array[0]);
}
