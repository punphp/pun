<?php

namespace IssetProperties;

class Named
{
    public int $value = 1;
    public ?int $nullable = null;
    public int $uninitialized;
    public static int $shared = 1;
}

function singleton(Named $object): void
{
    $name = 'value';
    echo isset($object->$name);
    echo $object->$name ?? 0;
    $object->$name ??= 0;
}

/** @param 'nullable'|'uninitialized'|'missing' $name */
function dynamicControls(Named $object, string $name): void
{
    echo isset($object->$name);
    $nullable = 'nullable';
    echo isset($object->$nullable);
    $uninitialized = 'uninitialized';
    echo isset($object->$uninitialized);
    $missing = 'missing';
    echo isset($object->$missing);
    echo isset($object->missing);
}

function staticControls(): void
{
    echo isset(Named::$shared);
    $name = 'shared';
    echo isset(Named::$$name);
}

/** @template T */
class Box
{
    /** @var T */
    public $value;
}

/** @param Box<int> $object */
function generic(Box $object): void
{
    echo isset($object->value);
    $name = 'value';
    echo isset($object->$name);
    echo $object->value ?? 0;
    $object->value ??= 0;
}

/** @param Box<int|null> $object */
function nullableGeneric(Box $object): void
{
    echo isset($object->value);
    echo $object->value ?? 0;
}

/**
 * @template U
 * @extends Box<U>
 */
class InheritedBox extends Box {}

/** @param InheritedBox<int> $object */
function inheritedGeneric(InheritedBox $object): void
{
    echo isset($object->value);
    $name = 'value';
    echo isset($object->$name);
}

/** @extends Box<int> */
class IntBox extends Box {}

function fixedGeneric(IntBox $object): void
{
    echo isset($object->value);
}

class Other
{
    public int $value = 1;
}

function unionControl(Named|Other $object): void
{
    echo isset($object->value);
}

/** @property int $value */
class Documented
{
    public function __get(string $name): int { return 1; }
    public function __isset(string $name): bool { return true; }
}

function annotationControl(Documented $object): void
{
    echo isset($object->value);
}

class Writable
{
    public ?int $value {
        get => 1;
        /** @param int $value */
        set (?int $value) {}
    }
}

class NullableWritable
{
    public int $value {
        get => 1;
        set (?int $value) {}
    }
}

function asymmetric(Writable $object, NullableWritable $nullable): void
{
    echo isset($object->value);
    echo isset($nullable->value);
}
