<?php

namespace OffsetReadDeclarationUnknown;

function implicit($value): void
{
    $local = $value['left'];
}

function explicit(mixed $value): void
{
    $local = $value['left'];
}

/**
 * @template T
 * @param T $value
 */
function generic($value): void
{
    $local = $value['left'];
}

function objectRead(object $value): void
{
    $local = $value['left'];
}

/** @param array<string, int>|object $value */
function objectUnion($value): void
{
    $local = $value['left'];
}
