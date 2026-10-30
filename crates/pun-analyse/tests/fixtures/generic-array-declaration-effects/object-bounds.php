<?php

namespace GenericArrayDeclarationBoundaries;

/**
 * @template T of \ArrayAccess<string, int>
 * @param T $value
 */
function access($value): void
{
    unset($value['left']);
}

/**
 * @template T of array<string, int>|\ArrayAccess<string, int>
 * @param T $value
 */
function possibleAccess($value): void
{
    unset($value['left']);
}

/**
 * @template T of \ArrayAccess<string, int>
 * @param T $value
 */
function read($value): void
{
    $local = $value['left'];
}

/**
 * @template T of array<string, int>|\ArrayAccess<string, int>
 * @param T $value
 */
function coalesceOffset($value): void
{
    $value['left'] ??= 1;
}

/**
 * @template T of array<string, int>|\ArrayAccess<string, int>
 * @param T $value
 */
function coalesceRoot($value): void
{
    $value ??= [];
    $value['left'] = 1;
}

/**
 * @template T of array{left?: int}
 * @param T $value
 */
function byReference(array &$value): void
{
    unset($value['left']);
}

/** @param array<string, int> $value */
function escape(array &$value): void
{
    $value['left'] = 1;
}

/**
 * @template T of array{left?: int}
 * @param T $value
 */
function escapingAlias(array $value): void
{
    $alias = &$value;
    escape($alias);
    unset($value['left']);
}

/** @template T of array{left?: int} */
class Box
{
    /** @var T */
    public array $value;

    /** @param T $value */
    public function __construct(array $value)
    {
        $this->value = $value;
    }

    public function remove(): void
    {
        unset($this->value['left']);
    }
}
