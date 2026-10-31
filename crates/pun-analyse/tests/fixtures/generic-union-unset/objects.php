<?php

namespace GenericUnionUnsetObjects;

final class Closed {}
class Open {}
interface Possible {}

/** @template T of array<string, int>|Closed
 * @param T $value
 */
function closedMember($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|Open
 * @param T $value
 */
function openMember($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array{left: int}|Possible
 * @param T $value
 */
function possibleMember($value): void
{
    unset($value['missing']);
    \PHPStan\dumpType($value);
}

/** @template T of array{left: int}|\ArrayAccess<string, int>
 * @param T $value
 */
function accessMember($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array{left: int}|\ArrayAccess<string, int>
 * @param T $value
 */
function unchangedAccess($value): void
{
    unset($value['missing']);
    \PHPStan\dumpType($value);
}

/** @implements \ArrayAccess<string, int> */
final class Access implements \ArrayAccess
{
    public function offsetExists(mixed $offset): bool { return true; }
    public function offsetGet(mixed $offset): int { return 1; }
    public function offsetSet(mixed $offset, mixed $value): void {}
    public function offsetUnset(mixed $offset): void {}
}

/** @template T of array<string, int>|Access
 * @param T $value
 */
function finalAccess($value): void
{
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array<int, int>|\FFI\CData
 * @param T $value
 */
function extraOffsetClass($value): void
{
    unset($value[0]);
    \PHPStan\dumpType($value);
}
