<?php

namespace Illuminate\Routing;

use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Macroable;

class PendingSingletonResourceRegistration
{
    use CreatesRegularExpressionRouteConstraints, Macroable;

    /**
     * The resource registrar.
     *
     * @var \Illuminate\Routing\ResourceRegistrar
     */
    protected $registrar;

    /**
     * The resource name.
     *
     * @var string
     */
    protected $name;

    /**
     * The resource controller.
     *
     * @var string
     */
    protected $controller;

    /**
     * The resource options.
     *
     * @var array
     */
    protected $options = [];

    /**
     * The resource's registration status.
     *
     * @var bool
     */
    protected $registered = false;

    /**
     * Create a new pending singleton resource registration instance.
     *
     * @param  \Illuminate\Routing\ResourceRegistrar  $registrar
     * @param  string  $name
     * @param  string  $controller
     * @param  array  $options
     */
    public function __construct(ResourceRegistrar $registrar, $name, $controller, array $options)
    {}

    /**
     * Set the methods the controller should apply to.
     *
     * @param  mixed  $methods
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function only($methods)
    { func_get_args(); }

    /**
     * Set the methods the controller should exclude.
     *
     * @param  mixed  $methods
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function except($methods)
    { func_get_args(); }

    /**
     * Indicate that the resource should have creation and storage routes.
     *
     * @return $this
     */
    public function creatable()
    {}

    /**
     * Indicate that the resource should have a deletion route.
     *
     * @return $this
     */
    public function destroyable()
    {}

    /**
     * Set the route names for controller actions.
     *
     * @param  array|string  $names
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function names($names)
    {}

    /**
     * Set the route name for a controller action.
     *
     * @param  string  $method
     * @param  string  $name
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function name($method, $name)
    {}

    /**
     * Override the route parameter names.
     *
     * @param  array|string  $parameters
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function parameters($parameters)
    {}

    /**
     * Override a route parameter's name.
     *
     * @param  string  $previous
     * @param  string  $new
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function parameter($previous, $new)
    {}

    /**
     * Add middleware to the resource routes.
     *
     * @param  mixed  $middleware
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function middleware($middleware)
    {}

    /**
     * Specify middleware that should be added to the specified resource routes.
     *
     * @param  array|string  $methods
     * @param  array|string  $middleware
     * @return $this
     */
    public function middlewareFor($methods, $middleware)
    {}

    /**
     * Specify middleware that should be removed from the resource routes.
     *
     * @param  array|string  $middleware
     * @return $this
     */
    public function withoutMiddleware($middleware)
    {}

    /**
     * Specify middleware that should be removed from the specified resource routes.
     *
     * @param  array|string  $methods
     * @param  array|string  $middleware
     * @return $this
     */
    public function withoutMiddlewareFor($methods, $middleware)
    {}

    /**
     * Add "where" constraints to the resource routes.
     *
     * @param  mixed  $wheres
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function where($wheres)
    {}

    /**
     * Add metadata to the registered singleton resource routes.
     *
     * @param  array  $metadata
     * @return $this
     */
    public function metadata(array $metadata)
    {}

    /**
     * Register the singleton resource route.
     *
     * @return \Illuminate\Routing\RouteCollection
     */
    public function register()
    {}

    /**
     * Handle the object's destruction.
     *
     * @return void
     */
    public function __destruct()
    {}
}
