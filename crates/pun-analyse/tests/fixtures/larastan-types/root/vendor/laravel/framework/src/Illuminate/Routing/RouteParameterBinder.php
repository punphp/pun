<?php

namespace Illuminate\Routing;

use Illuminate\Support\Arr;

class RouteParameterBinder
{
    /**
     * The route instance.
     *
     * @var \Illuminate\Routing\Route
     */
    protected $route;

    /**
     * Create a new Route parameter binder instance.
     *
     * @param  \Illuminate\Routing\Route  $route
     */
    public function __construct($route)
    {}

    /**
     * Get the parameters for the route.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array
     */
    public function parameters($request)
    {}

    /**
     * Get the parameter matches for the path portion of the URI.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array
     */
    protected function bindPathParameters($request)
    {}

    /**
     * Extract the parameter list from the host part of the request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  array  $parameters
     * @return array
     */
    protected function bindHostParameters($request, $parameters)
    {}

    /**
     * Combine a set of parameter matches with the route's keys.
     *
     * @param  array  $matches
     * @return array
     */
    protected function matchToKeys(array $matches)
    {}

    /**
     * Replace null parameters with their defaults.
     *
     * @param  array  $parameters
     * @return array
     */
    protected function replaceDefaults(array $parameters)
    {}
}
