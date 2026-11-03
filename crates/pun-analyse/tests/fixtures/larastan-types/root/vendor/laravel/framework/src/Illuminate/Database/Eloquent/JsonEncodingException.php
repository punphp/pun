<?php

namespace Illuminate\Database\Eloquent;

use RuntimeException;

class JsonEncodingException extends RuntimeException
{
    /**
     * Create a new JSON encoding exception for the model.
     *
     * @param  mixed  $model
     * @param  string  $message
     * @return static
     */
    public static function forModel($model, $message)
    {}

    /**
     * Create a new JSON encoding exception for the resource.
     *
     * @param  \Illuminate\Http\Resources\Json\JsonResource  $resource
     * @param  string  $message
     * @return static
     */
    public static function forResource($resource, $message)
    {}

    /**
     * Create a new JSON encoding exception for an attribute.
     *
     * @param  mixed  $model
     * @param  mixed  $key
     * @param  string  $message
     * @return static
     */
    public static function forAttribute($model, $key, $message)
    {}
}
