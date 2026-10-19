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

