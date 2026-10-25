<?php
namespace GenericDefaultsControls;

/** @template T of int|string */
class BoundWithoutDefault {}
/** @template T of int|string */
class EmptyWithoutDefault
{
    public function __construct() {}
}

function controls(): void
{
    \PHPStan\dumpType(new BoundWithoutDefault());
    \PHPStan\dumpType(new EmptyWithoutDefault());
    \PHPStan\dumpType(new \SplObjectStorage());
    \PHPStan\dumpType(new \WeakMap());
}
