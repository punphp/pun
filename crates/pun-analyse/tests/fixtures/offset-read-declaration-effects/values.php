<?php

namespace OffsetReadDeclarationValues;

/**
 * @template T of array<int, int>|string
 * @param T $value
 */
function unionRead($value): void
{
    $local = $value[0];
}

/**
 * @template T of array{left: int}
 * @param T $value
 */
function nullableTemplate(?array $value): void
{
    $local = $value['left'];
}

/** @param array{left: int}|null $value */
function nullableRead(?array $value): void
{
    $local = $value['left'];
}

/** @param array<int, int>|string $value */
function repeatedRead($value): void
{
    $first = $value[0];
    $second = $value[1];
}

/** @param array<int, int>|string $value */
function impureDimension($value): void
{
    $local = $value[print 'key'];
}

/**
 * @phpstan-impure
 * @throws void
 */
function impureKey(): int
{
    echo 'key';
    return 0;
}

/** @param array<int, int>|string $value */
function impureCall($value): void
{
    $local = $value[impureKey()];
}

/** @param \ArrayAccess<int, int> $value */
function replacedByString(\ArrayAccess $value): void
{
    $value = 'text';
    $local = $value[0];
}

/** @param \ArrayAccess<array-key, int> $value */
function keyReplacesObject(\ArrayAccess $value): void
{
    $local = $value[($value = '0')];
}

/**
 * @param array<int, int>|string $value
 * @param \ArrayAccess<int, int> $other
 */
function replacedByAccess($value, \ArrayAccess $other): void
{
    $value = $other;
    $local = $value[0];
}

/**
 * @param array<int, int>|string $value
 * @param \ArrayAccess<int, int> $other
 */
function keyReplacesValue($value, \ArrayAccess $other): void
{
    $local = $value[($value = $other)[0]];
}
