<?php

namespace OffsetReadDeclarationObjects;

final class Closed
{
}

class Open
{
}

interface PossibleAccess
{
}

/** @param array<string, int>|Closed $value */
function closedRead($value): void
{
    $local = $value['left'];
}

/** @param array<string, int>|Open $value */
function openRead($value): void
{
    $local = $value['left'];
}

/** @param array<string, int>|PossibleAccess $value */
function interfaceRead($value): void
{
    $local = $value['left'];
}

/**
 * @template T of array<string, int>|Closed
 * @param T $value
 */
function closedUnion($value): void
{
    $local = $value['left'];
}

/**
 * @template T of array<string, int>|Open
 * @param T $value
 */
function openUnion($value): void
{
    $local = $value['left'];
}

/** @param \ArrayAccess<string, int> $value */
function accessRead(\ArrayAccess $value): void
{
    $local = $value['left'];
}

/**
 * @template T of array<string, int>|\ArrayAccess<string, int>|null
 * @param T $value
 */
function accessUnion($value): void
{
    $local = $value['left'];
}
