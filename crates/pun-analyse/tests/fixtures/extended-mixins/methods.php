<?php

namespace ExtendedMixins;

class Base
{
    public function run(int $old): void {}
}

class Child extends Base
{
    public function run(int $new): void {}
}

/**
 * @template T of Base
 * @mixin T
 */
class GenericProxy {}

/** @mixin Base */
class Proxy {}

/** @mixin Proxy */
class NestedProxy {}

/** @mixin Base&\Countable */
class IntersectionProxy {}

/** @param GenericProxy<Base> $proxy */
function generic(GenericProxy $proxy): void
{
    $proxy->run(old: 1);
}

function reflected(Proxy $plain, NestedProxy $nested, IntersectionProxy $intersection): void
{
    $plain->run(old: 1);
    $nested->run(old: 1);
    $intersection->run(old: 1);
}

/** @mixin Base|\Countable */
class PartialUnionProxy {}

function missing(Proxy $plain, NestedProxy $nested, PartialUnionProxy $union): void
{
    $plain->absent();
    $nested->absent();
    $union->run(old: 1);
}

/** @mixin CycleB */
class CycleA {}

/** @mixin CycleA */
class CycleB {}

function cycle(CycleA $proxy): void
{
    $proxy->absent();
}

/**
 * @mixin SelfProxy
 * @mixin Base
 */
class SelfProxy {}

function cycleWithMethod(SelfProxy $proxy): void
{
    $proxy->run(old: 1);
}

/**
 * @method void run(int $tag)
 * @mixin Base
 */
class TaggedProxy {}

/** @mixin TaggedProxy */
class NestedTaggedProxy {}

function annotations(TaggedProxy $tagged, NestedTaggedProxy $nested): void
{
    $tagged->run(tag: 1);
    $nested->run(tag: 1);
    $tagged->run(old: 1);
    $nested->run(old: 1);
}

/** @mixin Base */
class NativeProxy
{
    public function run(int $native): void {}
}

class NativeChild extends NativeProxy
{
    public function run(int $changed): void {}
}

function ordinary(Base $base, Child $child, NativeProxy $native): void
{
    $base->run(old: 1);
    $child->run(new: 1);
    $native->run(native: 1);
}

/**
 * @template T of object
 * @mixin T
 */
class FlexibleProxy {}

/**
 * @param FlexibleProxy<Base> $base
 * @param FlexibleProxy<Child> $child
 * @param FlexibleProxy<\Countable> $missing
 */
function arguments(FlexibleProxy $base, FlexibleProxy $child, FlexibleProxy $missing): void
{
    $base->run('wrong');
    $child->run('wrong');
    $missing->run(1);
}

/** @extends FlexibleProxy<Base> */
class InheritedProxy extends FlexibleProxy {}

function inherited(InheritedProxy $proxy): void
{
    $proxy->run('wrong');
}

/** @mixin Base */
trait Methods {}

class TraitProxy
{
    use Methods;
}

function traits(TraitProxy $proxy): void
{
    $proxy->run(old: 1);
}

/**
 * @template T of object
 * @mixin T
 */
trait GenericMethods {}

class GenericTraitProxy
{
    /** @use GenericMethods<Base> */
    use GenericMethods;
}

/** @mixin NativeProxy */
class ParentProxy {}

class OrderedProxy extends ParentProxy
{
    use Methods;
}

/** @mixin NativeProxy */
class OwnProxy extends OrderedProxy {}

/** @mixin FlexibleProxy<Base> */
class NestedGenericProxy {}

/**
 * @template T of object = Base
 * @mixin T
 */
class DefaultProxy {}

function ordering(OrderedProxy $traitsFirst, OwnProxy $ownFirst): void
{
    $traitsFirst->run(old: 1);
    $ownFirst->run(native: 1);
}

function resolved(GenericTraitProxy $trait, NestedGenericProxy $nested, DefaultProxy $default): void
{
    $trait->run('wrong');
    $nested->run('wrong');
    $default->run('wrong');
}

/** @mixin Base|NativeProxy */
class CompleteUnionProxy {}

/** @mixin \Countable&\Stringable */
class MissingIntersectionProxy {}

function compounds(CompleteUnionProxy $complete, MissingIntersectionProxy $missing): void
{
    $complete->run(1);
    $missing->run(1);
}

trait BranchA
{
    use Methods;
}

trait BranchB
{
    use Methods;
}

class DiamondProxy
{
    use BranchA, BranchB;
}

function diamond(DiamondProxy $proxy): void
{
    $proxy->run(old: 1);
}

/**
 * @template T of Proxy
 * @param T $proxy
 */
function template(Proxy $proxy): void
{
    $proxy->run('wrong');
}

/** @mixin Base */
class ThisProxy
{
    /** @param $this $proxy */
    public function test(self $proxy): void
    {
        $proxy->run('wrong');
    }
}

function closure(): void
{
    $callback = static fn(): int => 1;
    $callback->absent();
}

/** @mixin mixed */
class UnknownSignatureProxy {}

function unknownSignature(UnknownSignatureProxy $proxy): void
{
    $proxy->absent();
}

/** @mixin UnknownSignatureProxy */
class NestedUnknownSignatureProxy {}

/** @mixin Ghost */
class UnknownClassProxy {}

function unknowns(NestedUnknownSignatureProxy $signature, UnknownClassProxy $class): void
{
    $signature->absent();
    $class->absent();
}

function ordinaryUnion(Proxy|NativeProxy $proxy): void
{
    $proxy->run(1);
}
