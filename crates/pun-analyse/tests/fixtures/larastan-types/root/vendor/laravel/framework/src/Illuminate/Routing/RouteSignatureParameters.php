<?php

namespace Illuminate\Routing;

use Illuminate\Support\Reflector;
use Illuminate\Support\Str;
use ReflectionFunction;
use ReflectionMethod;

class RouteSignatureParameters
{
    /**
     * Extract the route action's signature parameters.
     *
     * @param  array  $action
     * @param  array  $conditions
     * @return array
     */
    public static function fromAction(array $action, $conditions = [])
    {}

    /**
     * Get the parameters for the given class / method by string.
     *
     * @param  string  $uses
     * @return array
     *
     * @throws \ReflectionException
     */
    protected static function fromClassMethodString($uses)
    {}
}
