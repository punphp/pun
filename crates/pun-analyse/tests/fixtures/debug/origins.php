<?php

namespace DebugOrigins;

use function PHPStan\dumpType;

/** @var array{z: int, a: bool} */
$global = ['z' => 1, 'a' => true];
dumpType($global);

/**
 * @param array{z?: int, a: bool} $optional
 * @param iterable<string, array{z: int, a: bool}> $iterable
 */
function nested(array $optional, iterable $iterable): void
{
    dumpType($optional, $iterable);
    $optional = ['a' => false, 'z' => 2];
    dumpType($optional);
}

/**
 * @param array{a: bool, z: string} $left
 * @param array{z: string, a: bool} $right
 */
function branch(array $left, array $right, bool $flag): void
{
    if ($flag) {
        $left = $right;
    } else {
        dumpType($left);
    }
}

/** @param-out array{a: bool, z: string} $value */
function replace(array &$value): void { $value = ['a' => true, 'z' => 'x']; }

/** @param array{z: string, a: bool} $value */
function reference(array $value): void
{
    replace($value);
    dumpType($value);
}

/** @template T */
class Box
{
    /** @param T $value */
    function method($value): void
    {
        /** @var array{z: T, a: bool} */
        $shape = ['z' => $value, 'a' => true];
        dumpType($shape);
    }
}

/**
 * @template T
 * @param array<T> $input
 * @param callable(T): void $callback
 */
function each(array $input, callable $callback): void {}

/** @param array<array{z: string, a: bool}> $input */
function arrow(array $input): void
{
    each($input, static fn(array $row) => dumpType($row));
}

function noCallbackType(callable $callback): void {}
noCallbackType(static function (int $n): void { dumpType($n); });

/** @param array<array{z: string, a: bool}> $input */
function unknownArgument(array $input): void
{
    // An unrelated invalid argument cannot supply a template binding or erase the valid input's metadata.
    each($input, static function (array $row): void { dumpType($row); }, unexpected: true);
}
