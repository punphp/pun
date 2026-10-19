<?php

namespace CoverageUses;

/**
 * @template T
 */
trait Generic
{
}

/**
 * @template T = int
 */
trait AllDefault
{
}

/**
 * @template A
 * @template B = int
 */
trait SomeDefault
{
}

class Uses
{
    /** @use Generic<int> */
    use Generic, AllDefault, SomeDefault, MissingTrait;
}

enum UsingEnum
{
    use Generic;
}

interface UsingInterface
{
    use Generic;
}

/**
 * @template T = \stdClass
 * @template U = Generic
 */
class Defaults
{
}

/**
 * @template T of \CoverageGenerics\Pair<int, int>
 * @template U of int
 */
interface Bounded
{
}

/**
 * @implements Bounded<int, int>
 */
class NotObject implements Bounded
{
}

/**
 * @implements Bounded<\stdClass, int>
 */
class OtherObject implements Bounded
{
}

/**
 * @implements Bounded<*, *>
 */
class Wildcards implements Bounded
{
}

/**
 * @implements Bounded<never, int>
 */
class NeverBound implements Bounded
{
}

/**
 * @implements Bounded<\CoverageGenerics\Pair<int, int>, int>
 */
class SameBound implements Bounded
{
}

/**
 * @extends \CoverageGenerics\Pair<$this, int>
 */
class ThisArgument extends \CoverageGenerics\Pair
{
}
