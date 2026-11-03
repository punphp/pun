<?php

namespace Illuminate\Routing\Exceptions;

use Exception;
use Illuminate\Routing\Route;
use Illuminate\Support\Str;

class UrlGenerationException extends Exception
{
    /**
     * Create a new exception for missing route parameters.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @param  array  $parameters
     * @return static
     */
    public static function forMissingParameters(Route $route, array $parameters = [])
    {}
}
