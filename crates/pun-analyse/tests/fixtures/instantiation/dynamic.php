<?php

namespace InstantiationReflection;

class Concrete {
    public function __construct(int $value) {}
}
abstract class AbstractClass {}
interface Contract {}
trait SharedTrait {}
enum Choice { case One; }
class PrivateParent { private function __construct(int $value) {} }
class PrivateChild extends PrivateParent {}

/** @param class-string<Concrete> $class */
function concreteClass(string $class): void {
    new $class();
    new $class(1);
}

/** @param class-string<AbstractClass> $class */
function abstractClass(string $class): void { new $class(1); }

/** @param class-string<Contract> $class */
function interfaceClass(string $class): void { new $class(1); }

/** @param class-string<Concrete|AbstractClass> $class */
function concreteUnion(string $class): void { new $class(); }

/**
 * @template T of Concrete
 * @param class-string<T> $class
 */
function genericClass(string $class): void { new $class(); }

/** @param class-string<Concrete&Contract> $class */
function intersectionClass(string $class): void { new $class(); }

/** @param class-string $class */
function anyClass(string $class): void { new $class(1); }

/** @param class-string<Missing> $class */
function missingClassString(string $class): void { new $class(); }

function objectClasses(Concrete $concrete, AbstractClass $abstract, Contract $interface, Choice $enum): void {
    new $concrete();
    new $abstract(1);
    new $interface(1);
    new $enum();
}

function missingObject(Missing $object): void {
    new $object();
    $copy = $object;
    new $copy();
    $object = new Concrete(1);
    new $object();
}

function fromUnknownNew(): void {
    $object = (new Missing());
    $copy = $object;
    new $copy();
}

/** @param Concrete|Missing $object */
function unknownUnion($object): void { new $object(); }

/** @param Concrete|null $object */
function nullableObject($object): void { new $object(); }

/** @param Concrete|'MissingString' $object */
function stringObjectUnion($object): void { new $object(); }

/** @param 'InstantiationReflection\Concrete'|'InstantiationReflection\Missing' $class */
function literalUnion($class): void { new $class(); }

function literalClasses(): void {
    $abstract = AbstractClass::class;
    new $abstract();
    $interface = Contract::class;
    new $interface();
    $trait = SharedTrait::class;
    new $trait();
    $enum = Choice::class;
    new $enum();
    $private = PrivateChild::class;
    new $private();
    $class = 'instantiationreflection\\concrete';
    new $class();
    if (class_exists('GuardedUnknown')) {
        $class = 'GuardedUnknown';
        new $class();
    }
}

class KeywordParent { protected function __construct(int $value) {} }
final class KeywordChild extends KeywordParent {
    public static function build(): void {
        $self = 'self';
        new $self();
        $parent = 'parent';
        new $parent();
        $static = 'StAtIc';
        new $static();
    }
}

function keywordsOutside(): void {
    $class = 'self';
    new $class();
    $class = 'static';
    new $class();
    $class = 'parent';
    new $class();
}

/**
 * @template T of class-string<Concrete>
 * @param T $class
 */
function classStringTemplate($class): void { new $class(); }

/** @param class-string<Concrete> $class */
function guardedClassString(string $class): void {
    if (method_exists($class, 'extra')) {
        new $class();
    }
}

function mixedObject($object): void {
    new $object();
    $copy = $object;
    new $copy();
}

function anonymousObject(): void {
    $object = new class(1) { public function __construct(int $value) {} };
    new $object();
}

/**
 * @template T of string
 * @param T $class
 */
function stringTemplate($class): void { new $class(); }

/** @param class-string<Concrete>|class-string<Contract> $class */
function separateClassStrings($class): void { new $class(); }
