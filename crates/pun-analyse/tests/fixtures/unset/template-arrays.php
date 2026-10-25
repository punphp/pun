<?php

namespace UnsetTemplateArrays;

/**
 * @template T of array<string, int>
 * @param T&array{a: int} $array
 * @param T $original
 */
function rebuiltShape(array $array, array $original): void
{
    unset($array['missing']);
    unset($original[10]);
    unset($array['a']);
    unset($array['missing']);
}

/**
 * @template T of array{a?: int}
 * @param T&non-empty-array<string, int> $array
 */
function preservedShape(array $array): void
{
    unset($array['missing']);
    unset($array['a']);
    unset($array['missing']);
}

/**
 * @template T of array<int, int|string>
 * @param T&array<int|string, int> $array
 * @param T $original
 */
function crossedRefinements(array $array, array $original): void
{
    unset($array['missing']);
    unset($array[0]);
    unset($array['missing']);
}

/**
 * @template T of array<string, int>
 * @param T&array{a: 1, b?: 2} $array
 * @param T $original
 */
function retainedLiteralValues(array $array, array $original): void
{
    unset($array['missing']);
    unset($array['b']);
    unset($array['missing']);
}

/**
 * @template T of array{a: int}
 * @param T&array<string, int> $array
 */
function narrowerTemplate(array $array): void
{
    unset($array['missing']);
    unset($array['a']);
    unset($array['missing']);
}
