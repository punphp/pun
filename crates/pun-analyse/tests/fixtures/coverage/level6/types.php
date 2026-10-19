<?php

namespace CoverageTypes;

/**
 * @template T
 */
enum GenericEnum
{
}

/**
 * @template T
 */
trait GenericTrait
{
}

/**
 * @template T = int
 */
class AllDefaults
{
}

/**
 * @template A
 * @template B = int
 */
class SomeDefaults
{
}

class ExtendsAllDefaults extends AllDefaults
{
}

class ExtendsSomeDefaults extends SomeDefaults
{
}

class Types
{
    /**
     * @template T
     * @param array&callable&iterable $a
     * @param GenericEnum $b
     * @param callable&non-empty-array $c
     * @param T&callable&array $d
     * @return ($a is int ? array : string)
     */
    public function f($a, $b, $c, $d)
    {
        return '';
    }

    /**
     * @return array{a: array}|array{b: array}
     */
    public function shapes(): array
    {
        return ['a' => []];
    }
}

/**
 * @property-read array $split
 * @property-write string $split
 * @property-read int $same
 * @property-write int $same
 */
class Split
{
}
