<?php

namespace OffsetCallDeclarationValues;

/**
 * @template T of array<string, int>|string
 * @param T $value
 */
function unionUnset($value): void
{
    unset($value['left']);
}

/**
 * @template T of array{left?: int}
 * @param T $value
 */
function nullableTemplate(?array $value): void
{
    unset($value['left']);
}

/**
 * @template T of array<int, int>|string
 * @param T $value
 */
function unionRead($value): void
{
    $local = $value[0];
}

/** @param array{left?: int}|null $value */
function nullableUnset(?array $value): void
{
    unset($value['left']);
}

/** @param array{left: int}|null $value */
function nullableRead(?array $value): void
{
    $local = $value['left'];
}

/**
 * @template T of array<string, int>|string
 * @param T $value
 */
function repeatedUnset($value): void
{
    unset($value['left']);
    unset($value['right']);
}

/**
 * @template T of array<int, int>|string
 * @param T $value
 */
function impureDimension($value): void
{
    unset($value[print 'key']);
}

/**
 * @template T of array<string, int>|string
 * @param T &$value
 */
function referenced($value): void
{
    $alias = &$value;
    escape($alias);
    unset($value['left']);
}

/** @param array<string, int>|string $value */
function escape(&$value): void
{
    if ($value === '') {
        $value = [];
    } else {
        $value = '';
    }
}

/**
 * @template T of array<string, int>|string
 * @param T &$value
 */
function referenceParameter(&$value): void
{
    unset($value['left']);
}

/** @phpstan-impure @throws void */
function impureKey(): int
{
    echo 'key';
    return 0;
}

/**
 * @template T of array<int, int>|string
 * @param T $value
 */
function impureCall($value): void
{
    unset($value[impureKey()]);
}
