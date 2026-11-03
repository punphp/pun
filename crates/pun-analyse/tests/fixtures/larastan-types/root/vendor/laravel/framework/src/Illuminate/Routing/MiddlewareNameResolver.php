<?php

namespace Illuminate\Routing;

use Closure;
use LogicException;
use Throwable;

class MiddlewareNameResolver
{
    /**
     * Resolve the middleware name to a class name(s) preserving passed parameters.
     *
     * @param  \Closure|string  $name
     * @param  array  $map
     * @param  array  $middlewareGroups
     * @return \Closure|string|array
     */
    public static function resolve($name, $map, $middlewareGroups)
    {}

    /**
     * Parse the middleware group and format it for usage.
     *
     * @param  string  $name
     * @param  array  $map
     * @param  array  $middlewareGroups
     * @return array
     *
     * @throws Throwable
     */
    protected static function parseMiddlewareGroup($name, $map, $middlewareGroups)
    {}
}
