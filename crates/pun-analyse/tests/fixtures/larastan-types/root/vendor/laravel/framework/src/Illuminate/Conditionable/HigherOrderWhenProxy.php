<?php

namespace Illuminate\Support;

class HigherOrderWhenProxy
{
    /**
     * The target being conditionally operated on.
     *
     * @var mixed
     */
    protected $target;

    /**
     * The condition for proxying.
     *
     * @var bool
     */
    protected $condition;

    /**
     * Indicates whether the proxy has a condition.
     *
     * @var bool
     */
    protected $hasCondition = false;

    /**
     * Determine whether the condition should be negated.
     *
     * @var bool
     */
    protected $negateConditionOnCapture;

    /**
     * Create a new proxy instance.
     *
     * @param  mixed  $target
     */
    public function __construct($target)
    {}

    /**
     * Set the condition on the proxy.
     *
     * @param  bool  $condition
     * @return $this
     */
    public function condition($condition)
    {}

    /**
     * Indicate that the condition should be negated.
     *
     * @return $this
     */
    public function negateConditionOnCapture()
    {}

    /**
     * Proxy accessing an attribute onto the target.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}

    /**
     * Proxy a method call on the target.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
