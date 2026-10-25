<?php

namespace DebugDeclarations;

use function PHPStan\dumpType;
use function PHPStan\dumpPhpDocType;
use function array_slice as slice;

/**
 * @param array<mixed> $explicit
 * @param list $implicitList
 * @param list<mixed> $explicitList
 * @param array<array{id: string, bool: bool}> $rows
 * @param array{bool: bool, id: string} $reverse
 */
function values(array $implicit, array $explicit, array $implicitList, array $explicitList, array $rows, array $reverse, $unknown, mixed $known): void
{
    dumpType($implicit, $explicit, $implicitList, $explicitList);
    dumpType(slice($implicit, 0), slice($explicit, 0), slice($implicitList, 0), slice($explicitList, 0));
    dumpType(array_slice($unknown, 0), array_slice($known, 0));
    dumpType(array_slice(array: $known, offset: 0));
    $copy = ($explicitList);
    $copy = array_unique($copy);
    $copy = array_slice($copy, 0, 2);
    dumpType($copy);
    dumpType($rows, $reverse);
    dumpPhpDocType($rows, $reverse);
    $rows = $reverse;
    dumpType($rows);
    // The same keys in an unrelated declaration do not establish this literal's order.
    dumpType(['id' => 'x', 'bool' => false]);
    dumpType(['bool' => false, 'id' => 'x']);
}

/** @return array{z: string, a: bool} */
function ordered(): array { return ['z' => 'x', 'a' => true]; }

/** @return list<mixed> */
function explicitReturn(): array { return []; }

function implicitReturn(): array { return []; }

dumpType(ordered());
dumpType(array_slice(explicitReturn(), 0), array_slice(implicitReturn(), 0));

/**
 * @template TKey
 * @template TValue
 * @param iterable<TKey, TValue> $rows
 * @param callable(TKey, TValue): void $callback
 */
function each(iterable $rows, callable $callback): void {}

function annotated(): void
{
    /** @var array<array{id: string, bool: bool}> */
    $rows = [['id' => 'x', 'bool' => true]];
    dumpType($rows);
    each($rows, static function ($key, array $row): void { dumpType($row); });
    each(callback: static function ($key, array $row): void { dumpPhpDocType($row); }, rows: $rows);
    /** @var array{z: string, a: bool} $named */
    $named = ordered();
    $alias = $named;
    dumpType($alias);
}

/** @template T */
function templateAnnotation($value): void
{
    /** @var array{z: T, a: bool} */
    $shape = ['z' => $value, 'a' => true];
    dumpType($shape);
}
