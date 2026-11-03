<?php

namespace Illuminate\Routing;

class ControllerMiddlewareOptions
{
    /**
     * The middleware options.
     *
     * @var array
     */
    protected $options;

    /**
     * Create a new middleware option instance.
     *
     * @param  array  $options
     */
    public function __construct(array &$options)
    {}

    /**
     * Set the controller methods the middleware should apply to.
     *
     * @param  mixed  $methods
     * @return $this
     */
    public function only($methods)
    {}

    /**
     * Set the controller methods the middleware should exclude.
     *
     * @param  mixed  $methods
     * @return $this
     */
    public function except($methods)
    {}
}
