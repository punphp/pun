<?php

namespace UnsetWritableHooks;

/** @property-read string $p */
class VirtualGet
{
    public int|string $p { get => 'value'; }
}

/** @property-read string $p */
class BackedGet
{
    public int|string $p { get => $this->p; }
}

/** @property-read string $p */
class ImplicitSet
{
    public int|string $p { get => 'value'; set {} }
}

/** @property-read string $p */
class ExplicitSet
{
    public string $p { get => 'value'; set(int|string $value) {} }
}

/** @property-read string $p */
class DocSet
{
    /** @var string */
    public int|string $p {
        get => 'value';
        /** @param string $value */
        set(int|string $value) {}
    }
}

/** @property-read string $p */
class UntypedSet
{
    public mixed $p { get => 'value'; set(mixed $value) {} }
}

/** @property-read string $p */
class InheritedSet extends ExplicitSet {}

/** @property string $p */
class WritableGet
{
    public int|string $p { get => 'value'; }
}

function writes(
    VirtualGet $virtual,
    BackedGet $backed,
    ImplicitSet $implicit,
    ExplicitSet $explicit,
    DocSet $doc,
    UntypedSet $untyped,
    InheritedSet $inherited,
    WritableGet $writable,
): void {
    $virtual->p = 1;
    $virtual->p = 'not writable';
    $backed->p = 1;
    $backed->p = 'allowed';
    $implicit->p = 1;
    $implicit->p = 'allowed';
    $implicit->p = false;
    $explicit->p = 1;
    $explicit->p = 'allowed';
    $explicit->p = false;
    $doc->p = 1;
    $doc->p = 'allowed';
    $untyped->p = 1;
    $untyped->p = 'allowed';
    $inherited->p = 1;
    $inherited->p = 'allowed';
    $inherited->p = false;
    $writable->p = 1;
    $writable->p = 'not writable';
}

/** @property-read string $p */
class RedeclaredGet extends ExplicitSet
{
    public string $p { get => 'value'; }
}

function inheritedHookWrites(RedeclaredGet $redeclared): void
{
    $redeclared->p = 1;
    $redeclared->p = 'allowed';
    $redeclared->p = false;
}

/** @property-read string $p */
class OwnHook
{
    public string $p {
        get => 'value';
        set(int|string $value) {
            $this->p = 1;
            $this->p = 'allowed';
        }
    }
}

/** @property-read int $p */
class VarSet
{
    /** @var string */
    public mixed $p { get => 'value'; set(mixed $value) {} }
}

/** @property-read string $p */
class ImplicitVarSet
{
    /** @var string */
    public int|string $p { get => 'value'; set {} }
}

function varHookWrites(VarSet $var, ImplicitVarSet $implicit): void
{
    $var->p = 1;
    $var->p = 'allowed';
    $implicit->p = 1;
    $implicit->p = 'allowed';
    $implicit->p = false;
}
