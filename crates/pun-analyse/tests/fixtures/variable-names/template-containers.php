<?php

namespace VariableNameTemplateContainers;

use function PHPStan\dumpType;

/** @template T */
class Box {}

/**
 * @template Name of 'duplicate'
 * @param Name|'duplicate' $name
 * @param array{value: Name|'duplicate'} $shape
 * @param array<string, Name|'duplicate'> $array
 * @param iterable<int, Name|'duplicate'> $iterable
 * @param Box<Name|'duplicate'> $object
 * @param callable(): (Name|'duplicate') $callback
 */
function containers($name, array $shape, array $array, iterable $iterable, Box $object, callable $callback): void
{
    dumpType($name, $shape, $array, $iterable, $object);
    $copy = $shape;
    dumpType($copy);
    compact($shape);
    $shape['value'] = 'existing';
    dumpType($shape);
    $existing = 1;
    compact($shape);
    $name = 'existing';
    dumpType($name);
    compact($name);

    /** @var Name $annotation */
    $annotation = 'duplicate';
    dumpType($annotation);
    /** @var Name|'duplicate' $annotatedUnion */
    $annotatedUnion = 'duplicate';
    dumpType($annotatedUnion);
    compact($annotatedUnion);
    $alias = $annotatedUnion;
    dumpType($alias);
    compact($alias);
    /** @var array{value: Name|'duplicate'} $annotatedShape */
    $annotatedShape = ['value' => 'duplicate'];
    dumpType($annotatedShape);
    compact($annotatedShape);
    $callbackName = $callback();
    compact($callbackName);
}

/**
 * @template Name of 'duplicate'|'other'
 * @param Name|'duplicate' $name
 */
function narrowing($name): void
{
    dumpType($name);
    if ($name === 'duplicate') {
        compact($name);
    }
    $name = 'existing';
    dumpType($name);
    $existing = 1;
    compact($name);
}

/**
 * @template Names of ('duplicate'|'other')
 * @template Child of Names
 * @param Child|'duplicate' $name
 */
function templateBounds($name): void
{
    dumpType($name);
    compact($name);
}
