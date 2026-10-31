<?php

namespace GenericUnionUnsetValues;

/** @template T of array{left?: int}|null
 * @param T $value
 */
function nullableShape($value): void
{
    \PHPStan\dumpType($value);
    unset($value['left']);
    \PHPStan\dumpType($value);
    unset($value['missing']);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|null
 * @param T $value
 */
function nullableArray($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
    unset($value['right']);
    \PHPStan\dumpType($value);
}

/** @template T of array{left: int}|null
 * @param T $value
 */
function unchangedUnion($value): void
{
    unset($value['missing']);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|string|null
 * @param T $value
 */
function stringMember($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|int
 * @param T $value
 */
function integerMember($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @param array{left: int, kept: string}|null $value */
function ordinaryNullable($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array{left: int, kept: string}
 * @param T $value
 */
function singleShape($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

function mixedAndError(mixed $value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
    $error = 1;
    unset($error['left']);
    \PHPStan\dumpType($error);
    unset($error['right']);
    \PHPStan\dumpType($error);
}
