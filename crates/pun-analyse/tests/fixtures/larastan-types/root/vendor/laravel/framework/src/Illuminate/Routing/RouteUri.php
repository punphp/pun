<?php

namespace Illuminate\Routing;

class RouteUri
{
    /**
     * The route URI.
     *
     * @var string
     */
    public $uri;

    /**
     * The fields that should be used when resolving bindings.
     *
     * @var array
     */
    public $bindingFields = [];

    /**
     * Create a new route URI instance.
     *
     * @param  string  $uri
     * @param  array  $bindingFields
     */
    public function __construct(string $uri, array $bindingFields = [])
    {}

    /**
     * Parse the given URI.
     *
     * @param  string  $uri
     * @return static
     */
    public static function parse($uri)
    {}
}
