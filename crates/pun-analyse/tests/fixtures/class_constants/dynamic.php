<?php

namespace DynamicConstants;

class Ancestor {
    public const PRESENT = 1;
    private const PRIVATE_NAME = 2;
    protected const PROTECTED_NAME = 3;
}

class Child extends Ancestor {
    /** @var 'MISSING'|'PRESENT'|'PROTECTED_NAME' */
    public string $name;

    public function names(string $arbitrary, object $object): void {
        $name = 'MISSING';
        echo self::{$name};
        echo parent::{$name};
        echo static::{$name};
        echo self::{$this->name};
        echo self::{$arbitrary};
        echo self::{$object};
        $private = 'PRIVATE_NAME';
        echo parent::{$private};
        $protected = 'PROTECTED_NAME';
        echo parent::{$protected};
        $class = 'cLaSs';
        echo self::{$class};
    }
}

trait SharedTrait { public const PRESENT = 1; }
enum Choice { case Present; }

/** @param 'MISSING'|'PRESENT'|'PRIVATE_NAME'|'PROTECTED_NAME' $name */
function writtenClasses(string $name): void {
    echo Ancestor::{$name};
    echo CHILD::{$name};
    echo UnknownClass::{$name};
    echo SharedTrait::{$name};
    echo \DateTime::{$name};
    echo Choice::{$name};
}

/** @param 'MISSING'|'Present' $name */
function enumCases(string $name): void { echo Choice::{$name}; }

/** @param 'MISSING'|null $name */
function nullableName($name): void { echo Ancestor::{$name}; }

/** @param 'MISSING'|int $name */
function nonStringUnion($name): void { echo Ancestor::{$name}; }

/**
 * @template T of 'MISSING'
 * @param T $name
 */
function templateName($name): void { echo Ancestor::{$name}; }

function guards(): void {
    $name = 'MISSING';
    if (defined('DynamicConstants\Ancestor::MISSING')) {
        echo Ancestor::MISSING;
        echo Ancestor::{$name};
    }
    echo Ancestor::{$name};
    if (class_exists('DynamicConstants\UnknownClass')) {
        echo UnknownClass::{$name};
    }
    echo UnknownClass::{$name};
    $name = 'CLASS';
    echo UnknownClass::{$name};
}

/** @param 'MISSING'|'PRESENT' $name */
function branches(string $name): void {
    if ($name === 'PRESENT') {
        echo Ancestor::{$name};
    } else {
        echo Ancestor::{$name};
    }
    echo Ancestor::{$name};
}

function outsideClass(): void {
    $name = 'MISSING';
    echo self::{$name};
    echo static::{$name};
    echo parent::{$name};
}

class NoParent {
    public function read(): void {
        $name = 'MISSING';
        echo parent::{$name};
    }
}

/**
 * @template T of Ancestor
 * @param class-string<T> $class
 * @param T $object
 * @param Ancestor|\stdClass $union
 */
function expressionReceivers(string $class, Ancestor $object, Ancestor|\stdClass $union): void {
    $name = 'MISSING';
    echo $class::{$name};
    echo $object::{$name};
    echo $union::{$name};
}

/** @param 'DynamicConstants\Ancestor' $name */
function accessoryName($name): void {
    if (method_exists($name, 'extra')) {
        echo Ancestor::{$name};
    }
}
