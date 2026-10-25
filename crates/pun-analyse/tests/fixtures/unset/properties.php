<?php

namespace UnsetProperties;

class ParentProperties
{
    public readonly int $native;
    /** @readonly */
    public int $documented;
    public int $ordinary;
    private int $private;
    final public int $final;
    public private(set) int $privateSet;
    /** @final */
    public int $documentedFinal;
    public int $hooked { get => 1; }
    /** @readonly */
    public int $readonlyHooked { get => 1; }
    public static int $static;

    public function own(string $name): void
    {
        unset($this->native);
        unset($this->documented);
        unset($this->ordinary);
        unset($this->private);
        unset($this->final);
        unset($this->privateSet);
        unset($this->documentedFinal);
        unset($this->hooked);
        unset($this->readonlyHooked);
        unset($this->$name);
        unset(self::$static);
    }
}

final class ChildProperties extends ParentProperties {}

/** @final */
class DocumentedFinal
{
    public int $ordinary;
}

/** @property-read int $magic */
class Magic {}

enum Example { case Value; }

function uses(ParentProperties $parent, ChildProperties $child, DocumentedFinal $final, Magic $magic, Example $enum, \stdClass $dynamic): void
{
    unset($parent->absent);
    unset($parent->ordinary);
    unset($child->native);
    unset($child->ordinary);
    unset($final->ordinary);
    unset($magic->magic);
    unset($enum->name);
    unset($dynamic->anything);
}

class Other
{
    public readonly int $native;
}

/** @param ParentProperties|Other $union */
function unionProperty(object $union): void
{
    unset($union->native);
}

interface Marker {}

/** @param ParentProperties&Marker $intersection */
function intersectionProperty(object $intersection): void
{
    unset($intersection->native);
}

trait NativeTrait
{
    public readonly int $native;
}

class WithTrait
{
    use NativeTrait;
}

function traitProperty(WithTrait $trait): void
{
    unset($trait->native);
}

class InheritsTrait extends WithTrait {}

trait NestedTrait { use NativeTrait; }

class WithNestedTrait { use NestedTrait; }

function inheritedTraitProperty(InheritsTrait $inherited, WithNestedTrait $nested): void
{
    unset($inherited->native);
    unset($nested->native);
}

/** @template T of ParentProperties
 * @param T $object
 */
function templateProperty(ParentProperties $object): void
{
    unset($object->native);
}

function builtinProperties(\DatePeriod $period): void
{
    unset($period->start);
    unset($period->recurrences);
}

function anonymousFinality(): object
{
    return new class {
        public int $ordinary = 0;
        public readonly int $native;
        public function __construct() { $this->native = 1; }
        public function test(): void
        {
            unset($this->ordinary);
            unset($this->native);
        }
    };
}

function narrowedReceiver(ParentProperties|Other $object): void
{
    if ($object instanceof ParentProperties) {
        unset($object->native);
    }
}
