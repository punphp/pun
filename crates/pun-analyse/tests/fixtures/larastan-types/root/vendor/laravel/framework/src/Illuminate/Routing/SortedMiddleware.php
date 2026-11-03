<?php

namespace Illuminate\Routing;

use Illuminate\Support\Collection;

class SortedMiddleware extends Collection
{
    /**
     * Create a new Sorted Middleware container.
     *
     * @param  array  $priorityMap
     * @param  \Illuminate\Support\Collection|array  $middlewares
     */
    public function __construct(array $priorityMap, $middlewares)
    {}

    /**
     * Sort the middlewares by the given priority map.
     *
     * Each call to this method makes one discrete middleware movement if necessary.
     *
     * @param  array  $priorityMap
     * @param  array  $middlewares
     * @return array
     */
    protected function sortMiddleware($priorityMap, $middlewares)
    {}

    /**
     * Calculate the priority map index of the middleware.
     *
     * @param  array  $priorityMap
     * @param  string  $middleware
     * @return int|null
     */
    protected function priorityMapIndex($priorityMap, $middleware)
    {}

    /**
     * Resolve the middleware names to look for in the priority array.
     *
     * @param  string  $middleware
     * @return \Generator
     */
    protected function middlewareNames($middleware)
    { yield; }

    /**
     * Splice a middleware into a new position and remove the old entry.
     *
     * @param  array  $middlewares
     * @param  int  $from
     * @param  int  $to
     * @return array
     */
    protected function moveMiddleware($middlewares, $from, $to)
    {}
}
