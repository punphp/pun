<?php

namespace Illuminate\Routing;

use BackedEnum;
use BadMethodCallException;
use Closure;
use Illuminate\Support\Arr;
use Illuminate\Support\Reflector;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;

/**
 * @method \Illuminate\Routing\Route any(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\Route delete(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\Route get(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\Route options(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\Route patch(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\Route post(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\Route put(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\Route query(string $uri, \Closure|array|string|null $action = null)
 * @method \Illuminate\Routing\RouteRegistrar as(string $value)
 * @method \Illuminate\Routing\RouteRegistrar can(\UnitEnum|string  $ability, array|string $models = [])
 * @method \Illuminate\Routing\RouteRegistrar controller(string $controller)
 * @method \Illuminate\Routing\RouteRegistrar domain(\BackedEnum|string $value)
 * @method \Illuminate\Routing\RouteRegistrar metadata(array $metadata)
 * @method \Illuminate\Routing\RouteRegistrar middleware(array|string|null $middleware)
 * @method \Illuminate\Routing\RouteRegistrar missing(\Closure $missing)
 * @method \Illuminate\Routing\RouteRegistrar name(\BackedEnum|string $value)
 * @method \Illuminate\Routing\RouteRegistrar namespace(string|null $value)
 * @method \Illuminate\Routing\RouteRegistrar prefix(string $prefix)
 * @method \Illuminate\Routing\RouteRegistrar scopeBindings()
 * @method \Illuminate\Routing\RouteRegistrar where(array $where)
 * @method \Illuminate\Routing\RouteRegistrar withoutMiddleware(array|string $middleware)
 * @method \Illuminate\Routing\RouteRegistrar withoutScopedBindings()
 */
class RouteRegistrar
{
    use CreatesRegularExpressionRouteConstraints;
    use Macroable {
        __call as macroCall;
    }

    /**
     * The router instance.
     *
     * @var \Illuminate\Routing\Router
     */
    protected $router;

    /**
     * The attributes to pass on to the router.
     *
     * @var array
     */
    protected $attributes = [];

    /**
     * The methods to dynamically pass through to the router.
     *
     * @var string[]
     */
    protected $passthru = [
        'get', 'post', 'put', 'patch', 'delete', 'options', 'query', 'any',
    ];

    /**
     * The attributes that can be set through this class.
     *
     * @var string[]
     */
    protected $allowedAttributes = [
        'as',
        'can',
        'controller',
        'domain',
        'metadata',
        'middleware',
        'missing',
        'name',
        'namespace',
        'prefix',
        'scopeBindings',
        'where',
        'withoutMiddleware',
        'withoutScopedBindings',
    ];

    /**
     * The attributes that are aliased.
     *
     * @var array
     */
    protected $aliases = [
        'name' => 'as',
        'scopeBindings' => 'scope_bindings',
        'withoutScopedBindings' => 'scope_bindings',
        'withoutMiddleware' => 'excluded_middleware',
    ];

    /**
     * Create a new route registrar instance.
     *
     * @param  \Illuminate\Routing\Router  $router
     */
    public function __construct(Router $router)
    {}

    /**
     * Set the value for a given attribute.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function attribute($key, $value)
    {}

    /**
     * Route a resource to a controller.
     *
     * @param  string  $name
     * @param  string  $controller
     * @param  array  $options
     * @return \Illuminate\Routing\PendingResourceRegistration
     */
    public function resource($name, $controller, array $options = [])
    {}

    /**
     * Route an API resource to a controller.
     *
     * @param  string  $name
     * @param  string  $controller
     * @param  array  $options
     * @return \Illuminate\Routing\PendingResourceRegistration
     */
    public function apiResource($name, $controller, array $options = [])
    {}

    /**
     * Route a singleton resource to a controller.
     *
     * @param  string  $name
     * @param  string  $controller
     * @param  array  $options
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function singleton($name, $controller, array $options = [])
    {}

    /**
     * Route an API singleton resource to a controller.
     *
     * @param  string  $name
     * @param  string  $controller
     * @param  array  $options
     * @return \Illuminate\Routing\PendingSingletonResourceRegistration
     */
    public function apiSingleton($name, $controller, array $options = [])
    {}

    /**
     * Create a route group with shared attributes.
     *
     * @param  \Closure|array|string  $callback
     * @return $this
     */
    public function group($callback)
    {}

    /**
     * Register a new route with the given verbs.
     *
     * @param  array|string  $methods
     * @param  string  $uri
     * @param  \Closure|array|string|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function match($methods, $uri, $action = null)
    {}

    /**
     * Add metadata to routes registered by the registrar.
     *
     * @param  array  $metadata
     * @return $this
     */
    public function metadata(array $metadata)
    {}

    /**
     * Register a new route with the router.
     *
     * @param  string  $method
     * @param  string  $uri
     * @param  \Closure|array|string|null  $action
     * @return \Illuminate\Routing\Route
     */
    protected function registerRoute($method, $uri, $action = null)
    {}

    /**
     * Compile the action into an array including the attributes.
     *
     * @param  \Closure|array|string|null  $action
     * @return array
     */
    protected function compileAction($action)
    {}

    /**
     * Dynamically handle calls into the route registrar.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return \Illuminate\Routing\Route|$this
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
