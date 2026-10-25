<?php

namespace UnsetGenericKeyIdentities;

/** @template K of int<0, 1>
 * @param array<K, int> $array
 */
function exhaustedTemplate(array $array): void
{
    unset($array[0]);
    \PHPStan\dumpType($array);
    unset($array[1]);
    \PHPStan\dumpType($array);
    unset($array[1]);
}

/** @template K of string
 * @param array<K, int> $array
 * @param K $key
 */
function stringTemplate(array $array, string $key): void
{
    unset($array['ordinary']);
    \PHPStan\dumpType($array);
    unset($array['']);
    \PHPStan\dumpType($array);
    unset($array['ordinary']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($key);
}

/** @template T of array<string, int>
 * @param T $array
 * @param T $original
 */
function arrayIdentity(array $array, array $original, string $unknown): void
{
    unset($array[$unknown]);
    \PHPStan\dumpType($array);
    unset($array[1]);
    \PHPStan\dumpType($array);
    unset($array['ordinary']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($original);
}

/** @param array<int, int> $array */
function normalizedNumericString(array $array): void
{
    unset($array['0']);
    \PHPStan\dumpType($array);
    unset($array[0]);
}

/** @param array<'01'|'02', int> $array */
function nonnumericStrings(array $array): void
{
    unset($array['01']);
    \PHPStan\dumpType($array);
    unset($array['01']);
    unset($array['02']);
    \PHPStan\dumpType($array);
}

/** @template V of object
 * @param array<int, V> $array
 */
function valueIdentity(array $array): void
{
    unset($array[0]);
    \PHPStan\dumpType($array);
}

/** @param array{a: int, ...<int, int>} $array */
function openArrayDoesNotSubtractExtraKeys(array $array): void
{
    unset($array['a']);
    unset($array[0]);
    unset($array[0]);
}
