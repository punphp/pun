<?php

namespace OffsetCallDeclarationUnknown;

function implicit($value): void
{
    unset($value['left']);
}

function explicit(mixed $value): void
{
    unset($value['left']);
}

/**
 * @template T
 * @param T $value
 */
function generic($value): void
{
    unset($value['left']);
}

function objectUnset(object $value): void
{
    unset($value['left']);
}

/** @param array<string, int>|object $value */
function objectUnion($value): void
{
    unset($value['left']);
}
