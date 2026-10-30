<?php

namespace GenericArrayDeclarationEffects;

/**
 * @template T of array{left?: int}
 * @param T $value
 */
function remove(array $value): void
{
    unset($value['left']);
}

/**
 * @template T of array{left: int}
 * @param T $value
 */
function read(array $value): void
{
    $local = $value['left'];
}

/**
 * @template T of array{left?: int}
 * @param T $value
 */
function coalesceOffset(array $value): void
{
    $value['left'] ??= 1;
    unset($value['left']);
}

/**
 * @template T of array{left?: int}
 * @param T $value
 */
function coalesceRoot(array $value): void
{
    $value ??= [];
    $value['left'] = 1;
}

/**
 * @template T of array{left?: int}|array{right?: string}
 * @param T $value
 */
function arrayUnion(array $value): void
{
    unset($value['left']);
}

/**
 * @template T of array{left?: int}
 * @template U of T
 * @param U $value
 */
function dependent(array $value): void
{
    unset($value['left']);
}

/**
 * @template T of array{left?: int}
 * @template U of array{right?: string}
 * @param T|U $value
 */
function templateUnion(array $value): void
{
    unset($value['left']);
}

/**
 * @template T of array<string, int>
 * @param T $value
 */
function dictionary(array $value): void
{
    unset($value['left']);
}

/**
 * @template T of list<int>
 * @param T $value
 */
function vector(array $value): void
{
    unset($value[0]);
}

/**
 * @template T of array{left?: \stdClass}
 * @param T $value
 */
function objectValue(array $value): void
{
    unset($value['left']);
}

/**
 * @template T of array{left?: int}
 * @param T $value
 */
function documentedOnly($value): void
{
    unset($value['left']);
}
