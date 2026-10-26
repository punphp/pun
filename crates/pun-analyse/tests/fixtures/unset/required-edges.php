<?php
namespace UnsetRequiredEdges;

class Native
{
    public readonly int $p;
    public function __construct() { $this->p = 1; }
}
class Hooked { public int $p { get => 1; } }
class Absent {}
interface Marker {}
interface WithProperty { public int $p { get; } }
/** @phpstan-require-extends Native&Marker */
interface SinglePropertyIntersection {}
/** @phpstan-require-extends Native&WithProperty */
interface TwoPropertyIntersection {}
function intersections(SinglePropertyIntersection $single, TwoPropertyIntersection $both): void
{
    unset($single->p);
    unset($both->p);
}

/** @phpstan-require-extends Native */
interface ZReadonly {}
/** @phpstan-require-extends Hooked */
interface AHooked {}
interface ReadonlyFirst extends ZReadonly, AHooked {}
interface HookedFirst extends AHooked, ZReadonly {}
/** @phpstan-require-extends Hooked */
interface OwnRequirement extends ZReadonly {}
/** @phpstan-require-extends Absent */
interface ParentFallback extends ZReadonly {}
function order(ReadonlyFirst $readonly, HookedFirst $hooked, OwnRequirement $own, ParentFallback $fallback): void
{
    unset($readonly->p);
    unset($hooked->p);
    unset($own->p);
    unset($fallback->p);
}

/** @phpstan-require-extends Native|Hooked
 * @phpstan-require-extends Native
 */
interface FirstNonNativeUnion {}
/** @property int $p */
#[\AllowDynamicProperties]
class Magic {}
/** @phpstan-require-extends Magic
 * @phpstan-require-extends Native
 */
interface FirstNonNativeAnnotation {}
function noNativeFallback(FirstNonNativeUnion $union, FirstNonNativeAnnotation $magic): void
{
    unset($union->p);
    unset($magic->p);
}

/** @phpstan-readonly */
class OwnImmutable
{
    public int $p;
    public function __construct() { $this->p = 1; }
}
/** @phpstan-require-extends OwnImmutable */
interface NeedsOwnImmutable {}
readonly class OwnNativeReadonly
{
    public int $p;
    public function __construct() { $this->p = 1; }
}
/** @phpstan-require-extends OwnNativeReadonly */
interface NeedsOwnNativeReadonly {}
/** @final */
class OwnFinal { public int $p = 1; }
/** @phpstan-require-extends OwnFinal */
interface NeedsOwnFinal {}
function ownMetadata(NeedsOwnImmutable $doc, NeedsOwnNativeReadonly $native, NeedsOwnFinal $final): void
{
    unset($doc->p);
    unset($native->p);
    unset($final->p);
}

function explicitIntersection(ZReadonly&Native $o): void { unset($o->p); }
/** @template T of ZReadonly
 * @param T $o
 */
function template($o): void { unset($o->p); }
function returned(): ZReadonly { throw new \RuntimeException(); }
function returnedReceiver(): void { unset(returned()->p); }

use UnsetRequiredEdges\Native as ImportedRequired;
/** @phpstan-require-extends ImportedRequired */
interface Imported {}
function imported(Imported $o): void { unset($o->p); }
