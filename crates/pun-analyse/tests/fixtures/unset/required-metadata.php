<?php
namespace UnsetRequiredMetadata;

class ParentProperties
{
    public int $ordinary;
    public int $hooked { get => 1; }
    public readonly int $native;
    public function __construct() { $this->ordinary = $this->native = 1; }
}
/** @phpstan-readonly */
class ImmutableRequired extends ParentProperties {}
/** @phpstan-require-extends ImmutableRequired */
interface NeedsImmutable {}
function immutable(NeedsImmutable $o): void
{
    unset($o->ordinary);
    unset($o->hooked);
    unset($o->native);
}
function directImmutable(ImmutableRequired $o): void { unset($o->ordinary, $o->hooked); }

final class FinalRequired extends ParentProperties {}
/** @phpstan-require-extends FinalRequired */
interface NeedsFinal {}
function finalRequired(NeedsFinal $o): void { unset($o->ordinary, $o->hooked); }
function directFinal(FinalRequired $o): void { unset($o->ordinary, $o->hooked); }

/** @final */
class DocFinalRequired extends ParentProperties {}
/** @phpstan-require-extends DocFinalRequired */
interface NeedsDocFinal {}
function docFinalRequired(NeedsDocFinal $o): void { unset($o->ordinary, $o->hooked); }

/** @readonly
 * @phpstan-require-extends ParentProperties
 */
interface ImmutableInterface {}
function immutableInterface(ImmutableInterface $o): void { unset($o->ordinary, $o->hooked); }

#[\AllowDynamicProperties]
class DynamicParent extends ParentProperties {}
/** @phpstan-require-extends DynamicParent
 * @property int $native
 */
interface AnnotatedDynamicInterface {}
interface AnnotatedDynamicChild extends AnnotatedDynamicInterface {}
function dynamicParent(AnnotatedDynamicInterface $o): void { unset($o->native, $o->hooked); }
function dynamicChild(AnnotatedDynamicChild $o): void { unset($o->native, $o->hooked); }

trait NativeTrait
{
    public readonly int $native;
    public int $hooked { get => 1; }
    public function __construct() { $this->native = 1; }
}
class TraitRequired { use NativeTrait; }
class TraitRequiredChild extends TraitRequired {}
/** @phpstan-require-extends TraitRequiredChild */
interface NeedsTraitProperties {}
function traitProperties(NeedsTraitProperties $o): void { unset($o->native, $o->hooked); }

/** @phpstan-require-extends ParentProperties */
interface Contract {}
class InvalidImplementor implements Contract {}
function invalidImplementor(InvalidImplementor $o): void { unset($o->native, $o->hooked); }

/** @template T */
class GenericRequired
{
    public readonly int $native;
    public function __construct() { $this->native = 1; }
}
/** @phpstan-require-extends GenericRequired<string> */
interface NeedsGeneric {}
function generic(NeedsGeneric $o): void { unset($o->native); }

class Hooked { public int $p { get => 1; } }
class ReadonlyProperty { public readonly int $p; public function __construct() { $this->p = 1; } }
class Other { public int $unrelated = 1; }
/** @phpstan-require-extends ReadonlyProperty|Hooked */
interface UnionRequirement {}
/** @phpstan-require-extends ReadonlyProperty&Hooked */
interface IntersectionRequirement {}
/** @phpstan-require-extends ReadonlyProperty&Other */
interface SingleIntersectionRequirement {}
function compounds(UnionRequirement $union, IntersectionRequirement $both, SingleIntersectionRequirement $single): void
{
    unset($union->p);
    unset($both->p);
    unset($single->p);
}

/** @phpstan-require-extends Hooked
 * @psalm-require-extends ReadonlyProperty
 */
interface AliasPriority {}
/** @phpstan-require-extends Other
 * @phpstan-require-extends Hooked
 */
interface FirstMatching {}
function ordering(AliasPriority $alias, FirstMatching $second): void
{
    unset($alias->p);
    unset($second->p);
}
