<?php

namespace UnsetGenericParents;

/** @param array<string, array{leaf: int, kept: int}> $array */
function shapeValues(array $array, string $key): void
{
    $copy = $array;
    unset($array[$key]['leaf']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($copy);
}

/** @param array<string, int> $array */
function scalarValues(array $array, string $key): void
{
    unset($array[$key]['bad']);
    \PHPStan\dumpType($array);
}

/** @param non-empty-array<string, array{leaf: int}> $array */
function emptyWrittenShape(array $array, string $key): void
{
    unset($array[$key]['leaf']);
    \PHPStan\dumpType($array);
}

/** @param list<array{leaf: int, kept: int}> $array */
function listParent(array $array, int $key): void
{
    unset($array[$key]['leaf']);
    \PHPStan\dumpType($array);
}

/** @param array<string, array{first: int, second: string, kept: bool}> $array
 * @param 'first'|'second' $offset
 */
function optionalWrittenShape(array $array, string $key, string $offset): void
{
    unset($array[$key][$offset]);
    \PHPStan\dumpType($array);
}

/** @param array<string, array{first: int, keptA: bool}|array{first: string, keptB: float}> $array
 * @param 'first'|'missing' $offset
 */
function optionalWrittenUnion(array $array, string $key, string $offset): void
{
    unset($array[$key][$offset]);
    \PHPStan\dumpType($array);
}

/** @param array<string, array{int, string, bool}> $array */
function listValues(array $array, string $key): void
{
    unset($array[$key][0]);
    \PHPStan\dumpType($array);
}

/** @template T of array{leaf: int, kept: string}
 * @param array<string, T> $array
 */
function templateValues(array $array, string $key): void
{
    unset($array[$key]['leaf']);
    \PHPStan\dumpType($array);
}

/** @param array<string, array<string, int>> $array */
function genericValues(array $array, string $key): void
{
    unset($array[$key]['leaf']);
    \PHPStan\dumpType($array);
}

/** @param array<string, list{int, string, bool}> $array */
function explicitListValues(array $array, string $key): void
{
    unset($array[$key][0]);
    \PHPStan\dumpType($array);
}
