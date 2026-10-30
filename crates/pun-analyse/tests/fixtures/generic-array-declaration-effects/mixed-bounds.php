<?php

namespace GenericArrayDeclarationMixedBounds;

/**
 * @template T
 * @param T $value
 */
function implicitMixed($value): void
{
    unset($value['left']);
}

/**
 * @template T of mixed
 * @param T $value
 */
function explicitMixed($value): void
{
    unset($value['left']);
}
