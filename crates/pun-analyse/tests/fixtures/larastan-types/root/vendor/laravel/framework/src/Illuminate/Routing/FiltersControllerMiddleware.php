<?php

namespace Illuminate\Routing;

trait FiltersControllerMiddleware
{
    /**
     * Determine if the given options exclude a particular method.
     *
     * @param  string  $method
     * @param  array  $options
     * @return bool
     */
    public static function methodExcludedByOptions($method, array $options)
    {}
}
