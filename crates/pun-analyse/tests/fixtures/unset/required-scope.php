<?php
namespace UnsetRequiredScope;

class Root
{
    protected readonly int $p;
    public function __construct() { $this->p = 1; }
}
/** @property int $p */
class Required extends Root {}
/** @phpstan-require-extends Required */
interface Contract {}
class Caller extends Root
{
    public function throughInterface(Contract $o): void { unset($o->p); }
    public function throughClass(Required $o): void { unset($o->p); }
}

/** @property int $p */
#[\AllowDynamicProperties]
class DynamicRequired extends Root {}
/** @phpstan-require-extends DynamicRequired */
interface DynamicContract {}
class DynamicCaller extends Root
{
    public function throughInterface(DynamicContract $o): void { unset($o->p); }
}

/** @property int $p */
#[\AllowDynamicProperties]
class PrivateOverlay { private int $p = 1; public function inside(self $o): void { unset($o->p); } }
/** @phpstan-require-extends PrivateOverlay */
interface PrivateContract {}
function privateOverlay(PrivateOverlay $class, PrivateContract $contract): void
{
    unset($class->p);
    unset($contract->p);
}

/** @property int $p */
#[\AllowDynamicProperties]
class PrivateFinalOverlay { /** @final */ private int $p = 1; }
/** @phpstan-require-extends PrivateFinalOverlay */
interface PrivateFinalContract {}
function privateFinal(PrivateFinalOverlay $class, PrivateFinalContract $contract): void
{
    unset($class->p);
    unset($contract->p);
}

#[\AllowDynamicProperties]
class PrivateNative { private int $p = 1; }
/** @phpstan-require-extends PrivateNative */
interface PrivateNativeContract {}
function privateNative(PrivateNative $class, PrivateNativeContract $contract): void
{
    unset($class->p);
    unset($contract->p);
}

class PrivateReadonly
{
    private readonly int $p;
    public function __construct() { $this->p = 1; }
}
/** @phpstan-require-extends PrivateReadonly */
interface PrivateReadonlyContract {}
function privateReadonly(PrivateReadonly $class, PrivateReadonlyContract $contract): void
{
    unset($class->p);
    unset($contract->p);
}
