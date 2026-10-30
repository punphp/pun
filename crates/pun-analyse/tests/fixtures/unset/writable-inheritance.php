<?php

namespace UnsetWritableInheritance;

/** @property-read string $p */
class ParentOverlay { public int|string $p = 1; }
class InheritsOverlay extends ParentOverlay {}

class ParentNative { public int|string $p = 1; }
/** @property-read string $p */
class ChildOverlay extends ParentNative {}

class RedeclaresNative extends ParentOverlay { public int|string $p = 1; }

trait NativeProperty { public int|string $p = 1; }
/** @property-read string $p */
class TraitOverlay { use NativeProperty; }

/**
 * @template T of string
 * @property-read T $p
 */
class GenericOverlay { public mixed $p; }

/** @extends GenericOverlay<non-empty-string> */
class InheritsGeneric extends GenericOverlay {}

/**
 * @param GenericOverlay<non-empty-string> $generic
 */
function writes(
    InheritsOverlay $inherited,
    ChildOverlay $child,
    RedeclaresNative $redeclared,
    TraitOverlay $trait,
    GenericOverlay $generic,
    InheritsGeneric $specialized,
): void {
    $inherited->p = 1;
    $inherited->p = 'allowed';
    $child->p = 1;
    $child->p = 'allowed';
    $redeclared->p = 1;
    $redeclared->p = 'allowed';
    $trait->p = 1;
    $trait->p = 'allowed';
    $generic->p = 1;
    $generic->p = '';
    $generic->p = 'allowed';
    $specialized->p = 1;
    $specialized->p = '';
    $specialized->p = 'allowed';
}
