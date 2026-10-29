<?php

namespace ExtendedRequirements;

class Base
{
    public function run(int $old): void {}
}

class Child extends Base
{
    public function run(int $new): void {}
}

/** @mixin Base */
class Proxy {}

/** @phpstan-require-extends Base */
interface NativeRequirement {}

/** @phpstan-require-extends Proxy */
interface ExtendedRequirement {}

interface InheritedRequirement extends ExtendedRequirement {}

function reported(NativeRequirement $native, ExtendedRequirement $extended, InheritedRequirement $inherited): void
{
    $native->run(old: 1);
    $extended->run(old: 1);
    $inherited->run(old: 1);
}

class OtherBase
{
    public function run(int $other): void {}
}

class OtherChild extends OtherBase
{
    public function run(int $changed): void {}
}

/** @mixin OtherBase */
class OtherProxy {}

/**
 * @template T of object = OtherBase
 * @mixin T
 */
class GenericProxy {}

/** @phpstan-require-extends GenericProxy<Base> */
interface GenericRequirement {}

/** @phpstan-require-extends GenericProxy<Child> */
interface ChildRequirement {}

/**
 * @template T of Base = Base
 * @mixin T
 */
class BoundedProxy {}

/** @phpstan-require-extends BoundedProxy<Child> */
interface BoundedRequirement {}

function suppliedArguments(GenericRequirement $base, ChildRequirement $child, BoundedRequirement $bounded): void
{
    $base->run(old: 1);
    $child->run(new: 1);
    $bounded->run(new: 1);
    $child->run(new: 'wrong');
}

/** @template T */
class Holder
{
    /** @param T $value */
    public function value($value): void {}
}

/** @phpstan-require-extends Holder<int> */
interface NativeGenericRequirement {}

function nativeArguments(NativeGenericRequirement $holder): void
{
    $holder->value(1);
    $holder->value('wrong');
}

/**
 * @phpstan-require-extends OtherProxy
 * @psalm-require-extends Proxy
 */
interface PsalmFirst {}

/**
 * @phpstan-require-extends Proxy
 * @psalm-require-extends OtherProxy
 */
interface PsalmOtherFirst {}

/**
 * @phpstan-require-extends OtherProxy
 * @phpstan-require-extends Proxy
 */
interface DeclarationFirst {}

/**
 * @psalm-require-extends Proxy
 * @psalm-require-extends OtherProxy
 */
interface PsalmDeclarationFirst {}

function order(PsalmFirst $psalm, PsalmOtherFirst $other, DeclarationFirst $declaration, PsalmDeclarationFirst $psalmDeclaration): void
{
    $psalm->run(old: 1);
    $other->run(other: 1);
    $declaration->run(other: 1);
    $psalmDeclaration->run(old: 1);
}

/** @mixin \Countable */
class EmptyProxy {}

/**
 * @phpstan-require-extends EmptyProxy
 * @phpstan-require-extends Proxy
 */
interface ContinueRequirement {}

/** @phpstan-require-extends EmptyProxy */
interface ContinueInherited extends ExtendedRequirement {}

interface FirstBranch extends ExtendedRequirement {}

/** @phpstan-require-extends OtherProxy */
interface SecondBranch {}

interface BranchOrder extends FirstBranch, SecondBranch {}

function continuation(ContinueRequirement $own, ContinueInherited $parent, BranchOrder $branches): void
{
    $own->run(old: 1);
    $own->absent();
    $parent->run(old: 1);
    $branches->run(old: 1);
}

/** @phpstan-require-extends Proxy */
class NotAnInterface {}

function nonInterface(NotAnInterface $class): void
{
    $class->run(old: 1);
}

/** @phpstan-require-extends Proxy */
interface NativePrecedence
{
    public function run(int $native): void;
}

/**
 * @method void run(int $tag)
 * @phpstan-require-extends Proxy
 */
interface AnnotationPrecedence {}

/**
 * @mixin OtherProxy
 * @phpstan-require-extends Proxy
 */
interface MixinPrecedence {}

/** @method void run(int $tag) */
class TaggedProxy {}

/** @phpstan-require-extends TaggedProxy */
interface AnnotationRequirement {}

function precedence(NativePrecedence $native, AnnotationPrecedence $tag, MixinPrecedence $mixin, AnnotationRequirement $requiredTag): void
{
    $native->run(native: 1);
    $tag->run(tag: 1);
    $mixin->run(other: 1);
    $requiredTag->run(tag: 1);
    $requiredTag->run(old: 1);
}

/** @phpstan-require-extends CycleProxy */
interface CycleRequirement {}

/** @mixin CycleRequirement */
class CycleProxy {}

/** @phpstan-require-extends FallbackProxy */
interface FallbackRequirement {}

/**
 * @mixin FallbackRequirement
 * @mixin Proxy
 */
class FallbackProxy {}

/**
 * @phpstan-require-extends CycleProxy
 * @phpstan-require-extends Proxy
 */
interface ContinueCycleRequirement {}

function cycles(CycleRequirement $missing, FallbackRequirement $fallback, ContinueCycleRequirement $continuation): void
{
    $missing->absent();
    $fallback->run(old: 1);
    $continuation->run(old: 1);
}

function ordinary(Base $base, Child $child, Proxy $proxy): void
{
    $base->run(old: 1);
    $child->run(new: 1);
    $proxy->run(old: 1);
}

/** @phpstan-require-extends GenericProxy<Base&\Countable> */
interface IntersectionRequirement {}

class TextHolder
{
    public function value(string $value): void {}
}

/** @phpstan-require-extends GenericProxy<Holder<int>|TextHolder> */
interface CompoundRequirement {}

function compounds(IntersectionRequirement $intersection, CompoundRequirement $union): void
{
    $intersection->run(old: 1);
    // The union's semantic signature is int|string, not an arbitrary surviving leaf's int.
    // Ordinary compound type/reference checking remains guarded; this is not activation of that boundary.
    $union->value('text');
    $union->value(1);
    $union->value(extra: 1);
}

/** @phpstan-require-extends mixed */
interface UnknownSignatureRequirement {}

function unknownSignature(UnknownSignatureRequirement $unknown): void
{
    $unknown->absent();
}

/** @phpstan-require-extends OtherProxy */
interface ShadowingRequirement extends ExtendedRequirement {}

// BetterReflection removes the redundant immediate parent before the recursive extension walk.
interface RedundantParent extends ExtendedRequirement, ShadowingRequirement {}

interface BuiltinParent extends \Countable, ExtendedRequirement {}

function nominalParents(RedundantParent $redundant, BuiltinParent $builtin, BranchOrder $branches): void
{
    $redundant->run(other: 1);
    $builtin->run(old: 1);
    $builtin->count();
    $branches->absent();
}

/**
 * @template T
 * @extends Holder<T>
 */
class InheritedHolder extends Holder {}

/** @phpstan-require-extends InheritedHolder<int> */
interface InheritedGenericRequirement {}

function inheritedArguments(InheritedGenericRequirement $holder): void
{
    $holder->value(1);
    $holder->value('wrong');
}
