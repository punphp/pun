<?php

namespace ArrayMetadata;

use function PHPStan\dumpType;
use function PHPStan\dumpPhpDocType;

/**
 * @param array{z: int, a: string, ...<string, int|string>} $open
 * @param array{int, int, ...<never, never>} $sealed
 * @param callable&array $callback
 * @param array{string, string} $unrelated
 */
function declarations(array $open, array $sealed, array $callback, array $unrelated): void
{
    dumpType($open, $sealed, $callback, $unrelated);
    dumpPhpDocType($open, $sealed);
    $copy = $open;
    dumpType($copy);
    $copy = ['a' => 7];
    dumpType($copy);
    $callback = $unrelated;
    dumpType($callback);
}

/**
 * @template T of (callable&array)
 * @param T&array $callback
 */
function templateBound(array $callback): void
{
    dumpType($callback);
    $copy = $callback;
    dumpType($copy);
}

/**
 * @param array{a: int|string, z: bool, ...<never, never>} $closed
 */
function narrowedClosed(array $closed): void
{
    assert(is_int($closed['a']));
    dumpType($closed, $closed['a']);
}

/** @param array<string, int|string> $values */
function refinedGeneralArray(array $values): void
{
    assert(is_int($values['present']));
    dumpPhpDocType($values);
    dumpType($values['present']);
}

/** @param array{value: int, ...<never, never>}|callable(): void $value */
function unrelatedUnionMember(array|callable $value): void
{
    assert(!is_callable($value));
    dumpType($value);
}

/**
 * @param array{z: int, a: int, ...<string, int>} $open
 * @param array{string, string, ...<never, never>} $sealed
 */
function reassignedArrays(array $open, array $sealed): void
{
    $open = $sealed;
    dumpType($open);
    $sealed = ['different' => false];
    dumpType($sealed);
}

/** @param array{z: int|string, a: int|string, ...<string, int|string>} $open */
function asymmetricOpenValues(array $open): void
{
    assert(is_int($open['z']));
    assert(is_string($open['a']));
    dumpType($open);
}

/** @param array{z?: int, a?: string, ...<string, int|string>} $open */
function requiredOptionalKeys(array $open): void
{
    assert(isset($open['z']));
    assert(isset($open['a']));
    dumpType($open);
}

/** @template T of (callable&array) */
function annotatedTemplate($value): void
{
    /** @var T $callback */
    $callback = $value;
    dumpType($callback);
}
