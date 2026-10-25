<?php

namespace ConstantReceivers;

class Base {
    public const PRESENT = 1;
    private const PRIVATE_NAME = 2;
    protected const PROTECTED_NAME = 3;

    public function own(): void {
        echo $this::MISSING;
        echo $this::PRIVATE_NAME;
        echo $this::class;
    }
}

class Child extends Base {
    public function own(): void {
        echo $this::PRIVATE_NAME;
        echo $this::PROTECTED_NAME;
    }
}

class Other {
    private const PRESENT = 1;
    protected const PROTECTED_NAME = 2;
}

interface HasConstant { public const PRESENT = 1; }
enum Choice { case Present; }
trait SharedTrait { public const PRESENT = 1; }

/**
 * @param Base|Other $both
 * @param Base|\stdClass $partial
 * @param Base|int $scalar
 * @param Base|string $stringUnion
 * @param Base&HasConstant $intersection
 * @param class-string<Base> $class
 * @param class-string $anyClass
 */
function receivers(Base $base, $both, $partial, $scalar, $stringUnion, $intersection, string $class, string $anyClass, ?Base $nullable, mixed $explicit, $implicit, object $object, string $string, int $int): void {
    echo $base::MISSING;
    echo $base::PRESENT;
    echo $base::PRIVATE_NAME;
    echo $base::PROTECTED_NAME;
    echo $both::PRESENT;
    echo $both::PROTECTED_NAME;
    echo $partial::PRESENT;
    echo $scalar::PRESENT;
    echo $stringUnion::PRESENT;
    echo $intersection::PRESENT;
    echo $intersection::MISSING;
    echo $class::MISSING;
    echo $class::PRIVATE_NAME;
    echo $class::class;
    echo $anyClass::MISSING;
    echo $nullable::PRESENT;
    echo $explicit::MISSING;
    echo $implicit::MISSING;
    echo $object::MISSING;
    echo $object::class;
    echo $string::MISSING;
    echo $string::class;
    echo $int::MISSING;
    echo $int::class;
    echo null::MISSING;
    echo 'ConstantReceivers\Base'::class;
    echo 'ConstantReceivers\Missing'::MISSING;
    echo (new \stdClass)::MISSING;
    echo (new Choice)::Present;
    echo (new \DateTime)::MISSING;
    echo (new \DateTime)::ATOM;
}

/** @param Missing $missing */
function unknown($missing): void {
    echo $missing::MISSING;
    echo $missing::class;
    if (class_exists(Missing::class)) {
        echo $missing::MISSING;
    }
}

/** @param SharedTrait $trait */
function traitReceiver($trait): void { echo $trait::PRESENT; }

class Holder { public Base $base; }
function nullsafe(?Holder $holder): void { echo $holder?->base::PRESENT; }

/** @param 'MISSING'|'PRESENT'|'PRIVATE_NAME' $name */
function dynamic(Base $base, string $name): void { echo $base::{$name}; }

/**
 * @template T of Base
 * @param T $base
 */
function template($base): void { echo $base::MISSING; }

function versions(object $object): void {
    if (PHP_VERSION_ID >= 80000) { echo $object::class; }
    if (PHP_VERSION_ID < 80000) { echo $object::class; }
    echo $object::class;
}

final class FinalReceiver { public const PRESENT = 1; }
/**
 * @param iterable<int, Base> $iterable
 * @param object{value: int} $shape
 * @param Base|'ConstantReceivers\Base' $literalUnion
 * @param Base|'not-a-class' $badLiteralUnion
 * @param FinalReceiver|Base $finalUnion
 * @param \Closure(): void $closure
 */
function more(Choice $enum, Child $child, FinalReceiver $final, callable $callable, iterable $iterable, object $shape, $literalUnion, $badLiteralUnion, $finalUnion, \Closure $closure): void {
    echo $enum::Present;
    echo $enum::Missing;
    echo $child::PRESENT;
    echo $child::PROTECTED_NAME;
    echo $final::MISSING;
    echo $callable::MISSING;
    echo $iterable::MISSING;
    echo $shape::MISSING;
    echo $literalUnion::PRESENT;
    echo $badLiteralUnion::PRESENT;
    echo $finalUnion::MISSING;
    echo $closure::MISSING;
    $null = null;
    echo $null::MISSING;
    $literal = 'ConstantReceivers\Base';
    echo $literal::class;
    echo ('ConstantReceivers\Base')::class;
    if (defined('ConstantReceivers\Base::MISSING')) {
        echo $child::MISSING;
    }
}

/**
 * @template T
 * @param T $value
 */
function templateMixed($value): void { echo $value::MISSING; }

class NullableHolder {
    public ?Base $base;
    public function get(): ?Base { return $this->base; }
}
function nullableNullsafe(?NullableHolder $holder): void {
    echo $holder?->base::PRESENT;
    echo $holder?->get()::PRESENT;
}

/** @param 'PRESENT'|'MISSING' $name */
function correlated(string $name): void {
    $receiver = $name === 'PRESENT' ? new Base : new Other;
    echo $receiver::{$name};
}

/**
 * @template T = string
 * @param T $value
 */
function templateDefault($value): void { echo $value::MISSING; }

/** @template T = string */
class GenericReceiver {
    /** @param T $value */
    public function read($value): void { echo $value::MISSING; }
}

/** @param array-key $key */
function benevolent($key): void {
    echo $key::MISSING;
}

/** @param array<mixed> $array */
function benevolentForeach(array $array): void {
    foreach ($array as $key => $value) { echo $key::MISSING; }
}

/**
 * @template T of Base
 * @param T $receiver
 */
function templatePresent($receiver): void { echo $receiver::PRESENT; }

/**
 * @template T of array-key
 * @param T $receiver
 */
function templateKey($receiver): void { echo $receiver::MISSING; }

/**
 * @template T
 * @param T|Base $receiver
 */
function templateUnion($receiver): void { echo $receiver::PRESENT; }

/**
 * @template T of Missing
 * @param T $receiver
 */
function templateUnknown($receiver): void { echo $receiver::MISSING; }

function methodGuard(Base $receiver): void {
    if (method_exists($receiver, 'extra')) {
        echo $receiver::PRESENT;
        echo $receiver::MISSING;
    }
}

/**
 * @param (Base&HasConstant)|Other $intersection
 * @param Base|\Closure(): void $closure
 */
function compound($intersection, $closure): void {
    echo $intersection::MISSING;
    echo $closure::MISSING;
}

/**
 * @template T = string
 * @template U = int
 * @param T|U $receiver
 */
function templates($receiver): void { echo $receiver::MISSING; }

/**
 * @template T of int
 * @param T&Base $receiver
 */
function templateIntersection($receiver): void { echo $receiver::PRESENT; }

/**
 * @template T
 * @param T&Base $receiver
 */
function templateMixedIntersection($receiver): void { echo $receiver::MISSING; }

/**
 * @template T
 * @param T|Base $receiver
 */
function templateMissingUnion($receiver): void { echo $receiver::MISSING; }

/**
 * @template T
 * @param T&list<int> $receiver
 */
function templateList($receiver): void { echo $receiver::MISSING; }

class UnknownHolder { public Missing $base; }
function unknownNullsafe(?UnknownHolder $holder): void { echo $holder?->base::MISSING; }

/** @template T */
class GenericThis {
    public function read(): void { echo $this::MISSING; }
}

/** @param 1|2 $receiver */
function scalarLiterals(int $receiver): void { echo $receiver::MISSING; }

class LateReceiver extends Base {
    /** @param static $receiver */
    public function read($receiver): void { echo $receiver::MISSING; }
}

/** @param Choice::Present $receiver */
function enumCaseReceiver(Choice $receiver): void { echo $receiver::MISSING; }
