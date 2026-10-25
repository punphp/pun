<?php
namespace GenericDefaultsMixed;

/** @template T */
class ConstructorParent
{
    /** @param T $value */
    public function __construct(public mixed $value) {}
}

/**
 * @template T = string
 * @template U = list<T>
 * @extends ConstructorParent<T>
 */
class InheritedDefaults extends ConstructorParent
{
    /** @var U */
    public $items;
}

function inferredMixed(mixed $unknown): void
{
    $mixed = new InheritedDefaults($unknown);
    \PHPStan\dumpType($mixed->items);
    \PHPStan\dumpType($mixed->value);
}
