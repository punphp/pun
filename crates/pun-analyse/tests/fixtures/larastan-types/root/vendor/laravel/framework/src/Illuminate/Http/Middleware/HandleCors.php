<?php

namespace Illuminate\Http\Middleware;

use Closure;
use Fruitcake\Cors\CorsService;
use Illuminate\Contracts\Container\Container;
use Illuminate\Http\Request;

class HandleCors
{
    /**
     * The container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The CORS service instance.
     *
     * @var \Fruitcake\Cors\CorsService
     */
    protected $cors;

    /**
     * All of the registered skip callbacks.
     *
     * @var array<int, \Closure(\Illuminate\Http\Request): bool>
     */
    protected static $skipCallbacks = [];

    /**
     * Create a new middleware instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @param  \Fruitcake\Cors\CorsService  $cors
     */
    public function __construct(Container $container, CorsService $cors)
    {}

    /**
     * Handle the incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @return \Illuminate\Http\Response
     */
    public function handle($request, Closure $next)
    {}

    /**
     * Get the path from the configuration to determine if the CORS service should run.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return bool
     */
    protected function hasMatchingPath(Request $request): bool
    {}

    /**
     * Get the CORS paths for the given host.
     *
     * @param  string  $host
     * @return array
     */
    protected function getPathsByHost(string $host)
    {}

    /**
     * Register a callback that instructs the middleware to be skipped.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public static function skipWhen(Closure $callback)
    {}

    /**
     * Flush the middleware's global state.
     *
     * @return void
     */
    public static function flushState()
    {}
}
