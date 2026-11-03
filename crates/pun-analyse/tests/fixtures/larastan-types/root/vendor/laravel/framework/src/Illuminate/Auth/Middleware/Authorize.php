<?php

namespace Illuminate\Auth\Middleware;

use Closure;
use Illuminate\Contracts\Auth\Access\Gate;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Collection;

use function Illuminate\Support\enum_value;

class Authorize
{
    /**
     * The gate instance.
     *
     * @var \Illuminate\Contracts\Auth\Access\Gate
     */
    protected $gate;

    /**
     * Create a new middleware instance.
     *
     * @param  \Illuminate\Contracts\Auth\Access\Gate  $gate
     */
    public function __construct(Gate $gate)
    {}

    /**
     * Specify the ability and models for the middleware.
     *
     * @param  \UnitEnum|string  $ability
     * @param  string  ...$models
     * @return string
     */
    public static function using($ability, ...$models)
    {}

    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @param  string  $ability
     * @param  array|null  ...$models
     * @return mixed
     *
     * @throws \Illuminate\Auth\AuthenticationException
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    public function handle($request, Closure $next, $ability, ...$models)
    {}

    /**
     * Get the arguments parameter for the gate.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  array|null  $models
     * @return array
     */
    protected function getGateArguments($request, $models)
    {}

    /**
     * Get the model to authorize.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  string  $model
     * @return \Illuminate\Database\Eloquent\Model|string|null
     */
    protected function getModel($request, $model)
    {}

    /**
     * Checks if the given string looks like a fully-qualified class name.
     *
     * @param  string  $value
     * @return bool
     */
    protected function isClassName($value)
    {}
}
