<?php

namespace CoverageMore;

/**
 * @template T
 */
trait First
{
    /** @var T */
    public $first;
}

/**
 * @template U
 */
trait Second
{
    /** @var U */
    public $second;
}

class UsesBoth
{
    /** @use First<array> */
    use First, Second;
}

class TemplatedParent
{
    /**
     * @template T of array
     * @param T $value
     * @return T
     */
    public function same($value)
    {
        return $value;
    }
}

class TemplatedChild extends TemplatedParent
{
    public function same($value)
    {
        return $value;
    }
}

trait WithAttribute
{
    /**
     * @param array<int, int> $a
     */
    #[\Deprecated]
    abstract public function attributed($a);
}

class UsesAttribute
{
    use WithAttribute;

    public function attributed($a)
    {
    }
}

class ConstGrand
{
    const PLAIN = [];

    /** @var array<int, int> */
    const DEEP = [];
}

class ConstBase extends ConstGrand
{
    /** @var array<int, int> */
    const DOCUMENTED = [];
    const UNDOCUMENTED = [];
    const PLAIN = [];
    const DEEP = [];
}

class ConstLeaf extends ConstBase
{
    const UNDOCUMENTED = [];
    const PLAIN = [];
    const DEEP = [];
}
