<?php

namespace GenericUnionUnsetScopes;

/** @template T of array<string, int>|string
 * @param T $value
 */
function copies($value): void
{
    $copy = $value;
    unset($value['left']);
    \PHPStan\dumpType($value);
    \PHPStan\dumpType($copy);
    $assigned = $value;
    \PHPStan\dumpType($assigned);
    $value = $copy;
    \PHPStan\dumpType($value);
    unset($copy['right']);
    \PHPStan\dumpType($copy);
}

/** @template T of array{left: int}|null
 * @param T $value
 */
function branches($value, bool $condition): void
{
    if ($condition) {
        unset($value['left']);
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|string
 * @param T $value
 */
function aliases($value): void
{
    $alias =& $value;
    unset($alias['left']);
    \PHPStan\dumpType($alias);
    \PHPStan\dumpType($value);
    unset($value['right']);
    \PHPStan\dumpType($alias);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|string
 * @param T $value
 */
function replaced($value): void
{
    $value = ['left' => 1, 'kept' => 2];
    unset($value['left']);
    \PHPStan\dumpType($value);
}

/** @template T of array<string, int>|string
 * @param T $value
 */
function chainedAliases($value): void
{
    $first =& $value;
    $second =& $first;
    $copy = $value;
    unset($second['left']);
    \PHPStan\dumpType($first);
    \PHPStan\dumpType($second);
    \PHPStan\dumpType($value);
    \PHPStan\dumpType($copy);
}
