<?php

namespace NativeNullsafeTransfer;

class Box
{
    public int $value = 1;
    public ?int $nullable = null;
    /** @var positive-int */
    public int $documented = 1;
    public int $uninitialized;
    public array $items = [];
}
class Other { public int $value = 2; }

function nullable(?Box $box): void
{
    $box; // observe
    $box?->value; // observe
    ($box)?->nullable; // observe
    isset($box?->value); // observe
    $box?->value ?? 0; // observe
    $box?->nullable ?? 0; // observe
    $box; // observe
}

function nonnullable(Box $box): void
{
    $box?->value; // observe
    $box?->nullable; // observe
    $box?->documented; // observe
    isset($box?->uninitialized); // observe
    $box?->value ?? 0; // observe
}

/** @param Box $box */
function documentedNonnullable(?Box $box): void
{
    $box; // observe
    $box?->value; // observe
    isset($box?->value); // observe
    var_dump(isset($box?->value), isset($box?->nullable));
    $box?->value ?? 0; // observe
    $box?->nullable ?? 0; // observe
    $box; // observe
}

/** @param Box|null $box */
function widenedFullScopeBoundary(Box $box): void
{
    $box; // observe
    $box?->value; // observe
    isset($box?->value); // observe
    $box?->value ?? 0; // observe
}

function knownNull(null $box): void
{
    $box?->missing; // observe
    isset($box?->missing); // observe
    $box?->missing ?? 9; // observe
    $box; // observe
}

function receiverWrite(?Box $box): void
{
    ($box = null)?->value; // observe
    $box; // observe
}

/** @param Box $box */
function documentedMixed($box): void
{
    $box?->value; // observe
}

/** @param null $box */
function documentedNull($box): void
{
    $box; // observe
    $box?->absent; // observe
    isset($box?->absent); // observe
    $box?->absent ?? 9; // observe
}

function mixedReceiver($box): void
{
    $box?->value; // observe
}

function arrayMember(Box $box): void
{
    $box?->items; // unavailable
    $box; // unavailable
}

function unionReceiver(Box|Other|null $box): void
{
    $box; // unavailable
    $box?->value; // unavailable
}

function anyObject(object $box): void
{
    $box?->value; // unavailable
}

function nullableOrdinary(?Box $box): void
{
    $box->value; // unavailable
}

function missingMember(?Box $box): void
{
    $box?->absent; // unavailable
}

function dynamicMember(?Box $box, string $name): void
{
    $box?->{$name = 'value'}; // unavailable
    $name; // unavailable
}

function references(?Box $box): void
{
    $alias =& $box;
    $alias = null;
    $box?->value; // unavailable
}

/** @property int $magic */
class Magic {}
function magic(Magic $box): void
{
    $box?->magic; // unavailable
}

class Hooked { public int $value { get => 1; } }
function hooked(Hooked $box): void
{
    $box?->value; // unavailable
}
