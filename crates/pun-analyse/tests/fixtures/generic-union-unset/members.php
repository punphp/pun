<?php

namespace GenericUnionUnsetMembers;

/** @template T of list<int>|null
 * @param T $value
 */
function listMember($value): void
{
    unset($value[0]);
    \PHPStan\dumpType($value);
}

/** @template T of list<int>|null
 * @param T $value
 */
function missingListKey($value): void
{
    unset($value['absent']);
    \PHPStan\dumpType($value);
}

/** @template T of list{int, string}|null
 * @param T $value
 */
function tupleMember($value): void
{
    unset($value[0]);
    \PHPStan\dumpType($value);
}

/** @template T of list{int, ...<string>}|null
 * @param T $value
 */
function openListMember($value): void
{
    unset($value[0]);
    \PHPStan\dumpType($value);
}

/** @template T of array{left: int}|null
 * @param T $value
 */
function weakKey($value, string $key): void
{
    unset($value[$key]);
    \PHPStan\dumpType($value);
}

/** @template T of array<int, int>|null
 * @param T $value
 */
function missingArrayKey($value): void
{
    unset($value['absent']);
    \PHPStan\dumpType($value);
}

/** @template U of array{left: int, kept: string}
 * @template T of U|null
 * @param T $value
 */
function nestedTemplate($value): void
{
    \PHPStan\dumpType($value);
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T
 * @param T $value
 */
function unknownTemplate($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|false
 * @param T $value
 */
function booleanMember($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|\Closure(): void
 * @param T $value
 */
function closureMember($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of object
 * @param T&\ArrayAccess<string, int> $value
 */
function intersectedObject($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}
