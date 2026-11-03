<?php

namespace Illuminate\Support;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Traits\InteractsWithData;
use League\Uri\QueryString;
use Stringable;

class UriQueryString implements Arrayable, Stringable
{
    use InteractsWithData;

    /**
     * Create a new URI query string instance.
     */
    public function __construct(protected Uri $uri)
    {}

    /**
     * Retrieve all data from the instance.
     *
     * @param  mixed  $keys
     * @return array
     */
    public function all($keys = null)
    {}

    /**
     * Retrieve data from the instance.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    protected function data($key = null, $default = null)
    {}

    /**
     * Get a query string parameter.
     */
    public function get(?string $key = null, mixed $default = null): mixed
    {}

    /**
     * Get the URL decoded version of the query string.
     */
    public function decode(): string
    {}

    /**
     * Get the string representation of the query string.
     */
    public function value(): string
    {}

    /**
     * Convert the query string into an array.
     */
    public function toArray()
    {}

    /**
     * Get the string representation of the query string.
     */
    public function __toString(): string
    {}
}
