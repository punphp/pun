<?php

namespace Illuminate\Foundation\Http\Middleware;

use Closure;
use ErrorException;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Foundation\Http\MaintenanceModeBypassCookie;
use Illuminate\Foundation\Http\Middleware\Concerns\ExcludesPaths;
use Illuminate\Support\Arr;
use Symfony\Component\HttpKernel\Exception\HttpException;
use TypeError;

class PreventRequestsDuringMaintenance
{
    use ExcludesPaths;

    /**
     * The application implementation.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The URIs that should be excluded.
     *
     * @var array<int, string>
     */
    protected $except = [];

    /**
     * The URIs that should be accessible during maintenance.
     *
     * @var array
     */
    protected static $neverPrevent = [];

    /**
     * Create a new middleware instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct(Application $app)
    {}

    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @return mixed
     *
     * @throws \Symfony\Component\HttpKernel\Exception\HttpException
     * @throws \ErrorException
     */
    public function handle($request, Closure $next)
    {}

    /**
     * Determine if the incoming request has a maintenance mode bypass cookie.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  array  $data
     * @return bool
     */
    protected function hasValidBypassCookie($request, array $data)
    {}

    /**
     * Redirect the user to their intended destination with a maintenance mode bypass cookie.
     *
     * @param  string  $secret
     * @return \Illuminate\Http\RedirectResponse
     */
    protected function bypassResponse(string $secret)
    {}

    /**
     * Get the headers that should be sent with the response.
     *
     * @param  array  $data
     * @return array
     */
    protected function getHeaders($data)
    {}

    /**
     * Get the URIs that should be excluded.
     *
     * @return array
     */
    public function getExcludedPaths()
    {}

    /**
     * Indicate that the given URIs should always be accessible.
     *
     * @param  array|string  $uris
     * @return void
     */
    public static function except($uris)
    {}

    /**
     * Flush the state of the middleware.
     *
     * @return void
     */
    public static function flushState()
    {}
}
