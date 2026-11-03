<?php

namespace Illuminate\Routing;

use Illuminate\Contracts\Routing\UrlRoutable;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Routing\Exceptions\BackedEnumCaseNotFoundException;
use Illuminate\Support\Reflector;
use Illuminate\Support\Str;

class ImplicitRouteBinding
{
    /**
     * Resolve the implicit route bindings for the given route.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  \Illuminate\Routing\Route  $route
     * @return void
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<\Illuminate\Database\Eloquent\Model>
     * @throws \Illuminate\Routing\Exceptions\BackedEnumCaseNotFoundException
     */
    public static function resolveForRoute($container, $route)
    {}

    /**
     * Resolve the Backed Enums route bindings for the route.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @param  array  $parameters
     * @return \Illuminate\Routing\Route
     *
     * @throws \Illuminate\Routing\Exceptions\BackedEnumCaseNotFoundException
     */
    protected static function resolveBackedEnumsForRoute($route, $parameters)
    {}

    /**
     * Return the parameter name if it exists in the given parameters.
     *
     * @param  string  $name
     * @param  array  $parameters
     * @return string|null
     */
    protected static function getParameterName($name, $parameters)
    {}
}
