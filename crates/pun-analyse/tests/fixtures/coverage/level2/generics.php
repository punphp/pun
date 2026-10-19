<?php

namespace CoverageGenerics;

/**
 * @template A
 * @template B = int
 */
class Pair
{
}

/**
 * @template T
 */
trait GenericTrait
{
}

/**
 * @template T
 */
interface GenericInterface
{
}

enum Plain
{
}

/**
 * @template T
 * @template U = Missing
 * @template V = GenericTrait
 * @template W of static
 */
class Templates
{
}

/**
 * @template A
 * @template B
 * @template C = int
 */
class Triple
{
}

/**
 * @extends Triple<int>
 */
class TooFew extends Triple
{
}

/**
 * @extends Pair<int, int, int>
 */
class TooMany extends Pair
{
}

/**
 * @implements GenericInterface<*>
 */
class Wild implements GenericInterface
{
}

/**
 * @template T of Unknown<int>
 * @template U of Plain<int>
 * @template V of GenericTrait<int>
 */
class Bounds
{
}

/**
 * @phpstan-import-type Nope from Missing
 * @phpstan-import-type Loop from Second as Renamed
 * @phpstan-type Loop int
 */
class First
{
}

/**
 * @phpstan-import-type Loop from First
 */
class Second
{
}

/**
 * @extends Pair
 * @extends Pair<int>
 * @extends Pair<string>
 */
class Extends_ extends Pair
{
}

/**
 * @template T
 */
class Orphan extends MissingParent
{
}

/**
 * @implements GenericInterface<int>
 */
class Implements_ implements GenericInterface
{
}

/**
 * @template T of Pair<int, int>
 * @template U of int
 */
interface Bounded
{
}

/**
 * @implements Bounded<Pair<string, int>, string>
 */
class Bound implements Bounded
{
}

/**
 * @template T
 * @template T of int
 * @param T $value
 * @return T
 */
function twice($value)
{
    return $value;
}

/**
 * @template A
 * @template B
 * @template C = int
 * @template D
 */
class Quad
{
}

/**
 * @extends Quad<int>
 */
class QuadFew extends Quad
{
}
