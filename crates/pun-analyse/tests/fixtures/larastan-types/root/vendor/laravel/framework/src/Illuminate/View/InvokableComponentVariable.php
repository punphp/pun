<?php

namespace Illuminate\View;

use ArrayIterator;
use Closure;
use Illuminate\Contracts\Support\DeferringDisplayableValue;
use Illuminate\Support\Enumerable;
use IteratorAggregate;
use Stringable;
use Traversable;

/**
 * @template-covariant TValue = mixed
 *
 * @mixin TValue
 */
class InvokableComponentVariable implements DeferringDisplayableValue, IteratorAggregate, Stringable
{
    /**
     * The callable instance to resolve the variable value.
     *
     * @var \Closure
     */
    protected $callable;

    /**
     * Create a new variable instance.
     *
     * @param  \Closure(): TValue  $callable
     */
    public function __construct(Closure $callable)
    {}

    /**
     * Resolve the displayable value that the class is deferring.
     *
     * @return \Illuminate\Contracts\Support\Htmlable|string
     */
    public function resolveDisplayableValue()
    {}

    /**
     * Get an iterator instance for the variable.
     *
     * @return \ArrayIterator
     */
    public function getIterator(): Traversable
    {}

    /**
     * Dynamically proxy attribute access to the variable.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}

    /**
     * Dynamically proxy method access to the variable.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}

    /**
     * Resolve the variable.
     *
     * @return TValue
     */
    public function __invoke()
    {}

    /**
     * Resolve the variable as a string.
     *
     * @return string
     */
    public function __toString()
    {}
}
