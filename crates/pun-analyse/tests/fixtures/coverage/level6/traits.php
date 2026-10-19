<?php

namespace CoverageTraits;

trait Inner
{
    /** @var array */
    public $innerProp = [];

    /** @var array */
    const INNER = [];

    public function inner($a)
    {
    }
}

trait Outer
{
    use Inner;
}

class UsesOuter
{
    use Outer;
}
