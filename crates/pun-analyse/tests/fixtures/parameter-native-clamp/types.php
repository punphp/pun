<?php

namespace ParameterNativeClamp;

class Box { public int $value = 1; public ?int $nullable = null; }
class Child extends Box {}
class Other {}

/** @param Box|null $box */
function widened(Box $box): void
{
    $box; // observe
    $box?->value; // observe
    isset($box?->value); // observe
    $box?->nullable ?? 0; // observe
}

/** @param Box $box */
function nullableNative(?Box $box): void
{
    $box; // observe
    $box?->value; // observe
    isset($box?->value); // observe
}

/** @param Child|null $box */
function subtype(Box $box): void
{
    $box; // observe
    $box?->value; // observe
}

/** @param Child $box */
function nullableSubtype(?Box $box): void
{
    $box; // observe
    $box?->value; // observe
}

/** @param Box $box */
function widerDoc(Child $box): void
{
    $box; // observe
}

/** @param Other|null $box */
function unrelatedDoc(Box $box): void
{
    $box; // observe
    $box?->value; // observe
}

/** @param object|null $box */
function objectDoc(Box $box): void
{
    $box; // observe
}

/** @param mixed $box */
function mixedDoc(Box $box): void
{
    $box; // observe
}

/** @param null $box */
function nullDoc(Box $box): void
{
    $box; // observe
}

/** @param Box $box */
function nullNative(null $box): void
{
    $box; // observe
}

/** @param Child $box */
function unionDoc(Box|Other $box): void
{
    $box; // unavailable
}

function quiet(Box $box, ?Box $nullable): void
{
    $box; // observe
    $nullable; // observe
}

/** @param Box|null $box */
function defaultNull(Box $box = null): void
{
    $box; // observe
}
