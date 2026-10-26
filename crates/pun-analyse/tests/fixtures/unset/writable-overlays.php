<?php

namespace UnsetWritableOverlays;

/** @property-read string $p */
class Compatible { public int|string $p = 1; }

/** @property-read string $p */
class Incompatible { public int $p = 1; }

/** @property-read string|null $p */
class Nullable { public int|string|null $p = 1; }

/** @property-read string $p */
class Untyped { public $p; }

/** @property-read string $p */
class VarOverlay
{
    /** @var int */
    public int|string $p = 1;
}

/** @property-read string $p */
class PrivateOverlay
{
    private int|string $p = 1;
    public function __get(string $name): mixed { return null; }
    public function own(): void
    {
        $this->p = 1;
        $this->p = 'native';
    }
}

/** @property-read bool $p */
class ProtectedOverlay
{
    protected int|string $p = 1;
    public function __get(string $name): mixed { return null; }
    public function own(): void
    {
        $this->p = 1;
        $this->p = 'native';
        $this->p = false;
    }
}

/** @property-read string $p */
class AnnotationOnly
{
    public function __get(string $name): mixed { return null; }
}

/** @property string $p */
class WritablePrivate
{
    private int|string $p = 1;
    public function __get(string $name): mixed { return null; }
}

function writes(
    Compatible $compatible,
    Incompatible $incompatible,
    Nullable $nullable,
    Untyped $untyped,
    VarOverlay $var,
    PrivateOverlay $private,
    ProtectedOverlay $protected,
    AnnotationOnly $annotation,
    WritablePrivate $writable,
): void {
    $compatible->p = 1;
    $compatible->p = 'allowed';
    $incompatible->p = 1;
    $incompatible->p = 'rejected';
    $nullable->p = 1;
    $nullable->p = null;
    $nullable->p = 'allowed';
    $untyped->p = 1;
    $untyped->p = 'allowed';
    $var->p = 1;
    $var->p = 'allowed';
    $private->p = 1;
    $private->p = 'not writable';
    $protected->p = 1;
    $protected->p = false;
    $annotation->p = 1;
    $annotation->p = 'not writable';
    $writable->p = 1;
    $writable->p = 'allowed';
}

/**
 * @property-read string $p
 * @property-write int $p
 */
class SplitOverlay { public int|string $p = 1; }

function splitWrites(SplitOverlay $split): void
{
    $split->p = 1;
    $split->p = 'rejected';
}

function unionWrites(Compatible|Untyped $union): void
{
    $union->p = 1;
    $union->p = 'allowed';
}

/** @property-write string $p */
class WriteOnly { public int $p = 1; }

function writeOnlyWrites(WriteOnly $write): void
{
    $write->p = 1;
    $write->p = 'allowed';
}
