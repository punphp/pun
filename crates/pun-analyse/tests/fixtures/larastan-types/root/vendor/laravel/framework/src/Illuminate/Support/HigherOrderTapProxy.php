<?php

namespace Illuminate\Support;

/**
 * @template TTarget
 */
class HigherOrderTapProxy
{
    /**
     * The target being tapped.
     *
     * @var TTarget
     */
    public $target;

    /**
     * Create a new tap proxy instance.
     *
     * @param  TTarget  $target
     */
    public function __construct($target)
    {}

    /**
     * Dynamically pass method calls to the target.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return TTarget
     */
    public function __call($method, $parameters)
    {}
}
