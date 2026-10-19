<?php

// Project classes for the type-scope.json probes (scripts/export-phpstan-symbols.php probes); tests/scope.rs gives
// Pun the same classes through symbols::with_classes().

namespace PunProbe;

/**
 * @template TKey
 * @template TValue
 */
class Coll
{
    const A = 1;
    const B = 'b';
}

/**
 * @template T
 * @extends Coll<int, T>
 */
class Child extends Coll
{
    const C = 2;
}

class Plain extends Coll
{
}

/**
 * @template A
 * @template B
 * @template C = int
 * @template D
 */
class Defaults
{
}
