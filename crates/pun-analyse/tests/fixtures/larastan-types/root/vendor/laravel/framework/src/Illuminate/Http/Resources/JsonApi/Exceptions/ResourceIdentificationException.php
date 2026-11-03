<?php

namespace Illuminate\Http\Resources\JsonApi\Exceptions;

use RuntimeException;

class ResourceIdentificationException extends RuntimeException
{
    /**
     * Create an exception indicating we were unable to determine the resource ID for the given resource.
     *
     * @param  mixed  $resource
     * @return self
     */
    public static function attemptingToDetermineIdFor($resource)
    {}

    /**
     * Create an exception indicating we were unable to determine the resource type for the given resource.
     *
     * @param  mixed  $resource
     * @return self
     */
    public static function attemptingToDetermineTypeFor($resource)
    {}
}
