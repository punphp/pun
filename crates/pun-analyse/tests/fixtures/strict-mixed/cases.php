<?php

namespace StrictMixedControls;

final class Good {
    public const VALUE = 1;
    public int $property = 1;
    public function run(): void {}
}

final class Bare {}

function explicit(mixed $value): void {
    echo $value->property;
    isset($value->property);
    empty($value->property);
    echo $value::VALUE;
    $value(...);
    clone $value;
    \PHPStan\dumpType($value);
}

function implicitForeach($value): void {
    foreach ($value as $item) {}
}

function explicitMethod(mixed $value): void {
    $value->run();
}

function explicitCall(mixed $value): void {
    $value();
}

function explicitStatic(mixed $value): void {
    echo $value::$property;
    $value::run();
}

function implicitStatic($value): void {
    echo $value::$property;
    $value::run();
}

function explicitGuarded(mixed $value): void {
    if ($value !== null) {
        echo $value::VALUE;
        $value->run();
        echo $value->property;
    }
}

function implicitGuarded($value): void {
    if ($value !== null) {
        echo $value::VALUE;
        $value->run();
        echo $value->property;
    }
}

function implicit($value): void {
    echo $value->property;
    isset($value->property);
    echo $value::VALUE;
    if (!is_array($value)) {
        $value + [];
    }
    $value->run();
}

function explicitArrayExclusion(mixed $value): void {
    if (!is_array($value)) {
        $value + [];
    }
}

/**
 * @template T = string
 * @param T $value
 */
function template($value): void {
    $value->run();
    echo $value->property;
    isset($value->property);
    echo $value::VALUE;
    $value();
    $value(...);
    foreach ($value as $item) {}
    clone $value;
    \PHPStan\dumpType($value);
    ++$value;
}

/**
 * @template U = int
 * @template T of U = U
 * @param T $value
 */
function nested($value): void {
    $value->run();
    echo $value->property;
    echo $value::VALUE;
    \PHPStan\dumpType($value);
}

/**
 * @template T
 * @param T|Good $value
 */
function unionReceiver($value): void {
    $value->run();
    echo $value->property;
    echo $value::VALUE;
    \PHPStan\dumpType($value);
}

/**
 * @template T
 * @param T|Bare $value
 */
function unionRetainsError($value): void {
    $value->run();
    echo $value->property;
    echo $value::VALUE;
}

/**
 * @template T
 * @template U
 * @param T&U $value
 */
function intersectionReceiver($value): void {
    $value->run();
    echo $value->property;
    isset($value->property);
    echo $value::VALUE;
}

/**
 * @template T
 * @param T&callable $value
 */
function intersectionMaybe($value): void {
    isset($value->property);
    empty($value->property);
    $value->run();
    echo $value::VALUE;
}

/** @template T = int */
class Owner {
    /** @param T $value */
    public function inherited($value): void {
        $value->run();
        echo $value->property;
        echo $value::VALUE;
    }

    /**
     * @template T = string
     * @param T $value
     */
    public function shadow($value): void {
        $value->run();
        echo $value->property;
        echo $value::VALUE;
    }
}

/**
 * @template T
 * @param T $name
 */
function dynamicNames(Good $object, $name): void {
    $object->$name();
    echo $object->$name;
    echo $object::{$name};
}

/** @param callable<T of int = Missing>(T): void $callback */
function defaultIsNotAReferencedClass(callable $callback): void {}

function maybeReceiver(callable $value): void {
    isset($value->property);
    empty($value->property);
    $value->run();
}

/** @template T of int = parent::VALUE */
class MissingParentDefault {
    /** @var T */
    public $property;
}

function unknownObjectMethod(object $value): void {
    $value->run();
}
