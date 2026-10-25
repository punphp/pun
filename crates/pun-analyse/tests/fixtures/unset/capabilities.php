<?php

namespace UnsetCapabilities;

function excludingZero(mixed $value): void
{
    if ($value === 0) {
        return;
    }
    unset($value['x']);
}

/** @template T
 * @param T $condition
 * @param (T is int ? array{a: int} : array{b: int}) $value
 */
function conditionalReceiver(mixed $condition, mixed $value): void
{
    unset($value['a']);
    unset($value['missing']);
}

/** @template T of array{inner: array{a: int}}
 * @param T['inner'] $value
 */
function lateReceiver(mixed $value): void
{
    unset($value['a']);
    unset($value['missing']);
}

/** @template T of array{key: 1}
 * @param T['key'] $key
 * @param array{1: int} $value
 */
function lateKey(mixed $key, array $value): void
{
    unset($value[$key]);
}

/** @param array<mixed, int> $values */
function removedMixedKey(array $values): void
{
    unset($values[0]);
    foreach ($values as $key => $value) {
        unset($key[0]);
    }
}

/** @template K
 * @param array<K, int> $values
 */
function removedTemplateMixedKey(array $values): void
{
    unset($values[0]);
    foreach ($values as $key => $value) {
        unset($key[0]);
    }
}

/** @param int|float|bool|null|resource $value */
function nonOffsetDomain(mixed $value): void
{
    unset($value['x']);
}

/** @param array{a: int}|int $value */
function unionReceiver(mixed $value): void
{
    unset($value['a']);
    unset($value['missing']);
}

/** @param array{a: int}&\Countable $value */
function intersectionReceiver(mixed $value): void
{
    unset($value['a']);
    unset($value['missing']);
}

function stringReceiver(string $value): void
{
    unset($value[0]);
    unset($value['missing']);
}
