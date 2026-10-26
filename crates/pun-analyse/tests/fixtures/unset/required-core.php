<?php
namespace UnsetRequired;

class Ancestor
{
    public readonly int $native;
    /** @readonly */
    public int $documented;
    public int $virtual { get => 1; }
    public int $backed { set { $this->backed = $value; } }
    public int $ordinary = 1;
    final public int $final = 1;
    /** @final */
    public int $documentedFinal = 1;
    public private(set) int $privateSet = 1;
    protected readonly int $protectedNative;
    private readonly int $privateNative;
    public static int $static = 1;
    public function __construct()
    {
        $this->native = $this->documented = $this->protectedNative = $this->privateNative = 1;
    }
}
class Required extends Ancestor {}
/** @phpstan-require-extends Required */
interface Direct {}
interface Child extends Direct {}
/** @psalm-require-extends Required */
interface Psalm {}

function direct(Direct $o): void
{
    unset($o->native);
    unset($o->documented);
    unset($o->virtual);
    unset($o->backed);
    unset($o->ordinary);
    unset($o->final);
    unset($o->documentedFinal);
    unset($o->privateSet);
    unset($o->protectedNative);
    unset($o->privateNative);
    unset($o->static);
    unset($o->missing);
}
function child(Child $o): void { unset($o->native, $o->virtual, $o->ordinary); }
function psalm(Psalm $o): void { unset($o->native); }

/** @property int $native
 * @property int $virtual
 * @property int $ordinary
 * @property int $final
 */
class RequiredOverlay extends Ancestor {}
/** @phpstan-require-extends RequiredOverlay */
interface WithOverlay {}
function requiredOverlay(WithOverlay $o): void
{
    unset($o->native);
    unset($o->virtual);
    unset($o->ordinary);
    unset($o->final);
}

/** @phpstan-require-extends Required
 * @property int $native
 * @property int $virtual
 * @property int $ordinary
 */
interface InterfaceOverlay {}
function interfaceOverlay(InterfaceOverlay $o): void
{
    unset($o->native);
    unset($o->virtual);
    unset($o->ordinary);
}

#[\AllowDynamicProperties]
class DynamicRequired extends Ancestor {}
/** @phpstan-require-extends DynamicRequired
 * @property int $native
 * @property int $virtual
 * @property int $ordinary
 */
interface DynamicInterfaceOverlay {}
function dynamicInterfaceOverlay(DynamicInterfaceOverlay $o): void
{
    unset($o->native);
    unset($o->virtual);
    unset($o->ordinary);
}

/** @property-read int $native */
#[\AllowDynamicProperties]
class AnnotationOnly {}
/** @phpstan-require-extends AnnotationOnly */
interface RequiresAnnotationOnly {}
function annotationOnly(RequiresAnnotationOnly $o): void { unset($o->native); }

/** @phpstan-require-extends Required */
trait NeedsClass
{
    public function inTrait(): void
    {
        unset($this->native);
        unset($this->virtual);
        unset($this->ordinary);
    }
}
class ValidConsumer extends Required { use NeedsClass; }
class InvalidConsumer { use NeedsClass; }
function traitReceiver(NeedsClass $o): void { unset($o->native, $o->virtual, $o->ordinary); }
/** @phpstan-require-extends Required */
trait UnusedNeedsClass
{
    public function unused(): void { unset($this->native, $this->virtual); }
}

/** @phpstan-require-extends Required */
class InvalidClassRequirement {}
function invalidClassRequirement(InvalidClassRequirement $o): void { unset($o->native, $o->virtual); }

/** @phpstan-require-implements Direct */
trait ImplementsOnly {}
class ImplementsConsumer { use ImplementsOnly; }
function implementsConsumer(ImplementsConsumer $o): void { unset($o->native, $o->virtual); }

class BaseOwnReadonly { public readonly int $p; public function __construct() { $this->p = 1; } }
/** @phpstan-require-extends BaseOwnReadonly */
interface OwnHook { public int $p { get; } }
function ownHook(OwnHook $o): void { unset($o->p); }

/** @phpstan-require-extends MissingClass */
interface UnknownRequired {}
function unknownRequired(UnknownRequired $o): void { unset($o->native); }
