<?php

namespace Illuminate\Routing;

use ArrayObject;
use Closure;
use Illuminate\Container\Container;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Routing\BindingRegistrar;
use Illuminate\Contracts\Routing\Registrar as RegistrarContract;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Contracts\Support\Responsable;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Routing\Events\PreparingResponse;
use Illuminate\Routing\Events\ResponsePrepared;
use Illuminate\Routing\Events\RouteMatched;
use Illuminate\Routing\Events\Routing;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\Tappable;
use JsonSerializable;
use Psr\Http\Message\ResponseInterface as PsrResponseInterface;
use ReflectionClass;
use stdClass;
use Symfony\Bridge\PsrHttpMessage\Factory\HttpFoundationFactory;
use Symfony\Component\HttpFoundation\Response as SymfonyResponse;

/**
 * @mixin \Illuminate\Routing\RouteRegistrar
 */
class Router implements BindingRegistrar, RegistrarContract
{
    use Macroable {
        __call as macroCall;
    }
    use Tappable;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The IoC container instance.
     *
     * @var \Illuminate\Container\Container
     */
    protected $container;

    /**
     * The route collection instance.
     *
     * @var \Illuminate\Routing\RouteCollectionInterface
     */
    protected $routes;

    /**
     * The currently dispatched route instance.
     *
     * @var \Illuminate\Routing\Route|null
     */
    protected $current;

    /**
     * The request currently being dispatched.
     *
     * @var \Illuminate\Http\Request
     */
    protected $currentRequest;

    /**
     * All of the short-hand keys for middlewares.
     *
     * @var array
     */
    protected $middleware = [];

    /**
     * All of the middleware groups.
     *
     * @var array
     */
    protected $middlewareGroups = [];

    /**
     * The priority-sorted list of middleware.
     *
     * Forces the listed middleware to always be in the given order.
     *
     * @var array
     */
    public $middlewarePriority = [];

    /**
     * The registered route value binders.
     *
     * @var array
     */
    protected $binders = [];

    /**
     * The globally available parameter patterns.
     *
     * @var array
     */
    protected $patterns = [];

    /**
     * The route group attribute stack.
     *
     * @var array
     */
    protected $groupStack = [];

    /**
     * The registered custom implicit binding callback.
     *
     * @var callable|null
     */
    protected $implicitBindingCallback;

    /**
     * All of the verbs supported by the router.
     *
     * @var string[]
     */
    public static $verbs = ['GET', 'HEAD', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS', 'QUERY'];

    /**
     * Create a new Router instance.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @param  \Illuminate\Container\Container|null  $container
     */
    public function __construct(Dispatcher $events, ?Container $container = null)
    {}

    /**
     * Register a new GET route with the router.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function get($uri, $action = null)
    {}

    /**
     * Register a new POST route with the router.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function post($uri, $action = null)
    {}

    /**
     * Register a new PUT route with the router.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function put($uri, $action = null)
    {}

    /**
     * Register a new PATCH route with the router.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function patch($uri, $action = null)
    {}

    /**
     * Register a new DELETE route with the router.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function delete($uri, $action = null)
    {}

    /**
     * Register a new OPTIONS route with the router.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function options($uri, $action = null)
    {}

    /**
     * Register a new QUERY route with the router.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function query($uri, $action = null)
    {}

    /**
     * Register a new route responding to all verbs.
     *
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function any($uri, $action = null)
    {}

    /**
     * Register a new fallback route with the router.
     *
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function fallback($action)
    {}

    /**
     * Create a redirect from one URI to another.
     *
     * @param  string  $uri
     * @param  string  $destination
     * @param  int  $status
     * @return \Illuminate\Routing\Route
     */
    public function redirect($uri, $destination, $status = 302)
    {}

    /**
     * Create a permanent redirect from one URI to another.
     *
     * @param  string  $uri
     * @param  string  $destination
     * @return \Illuminate\Routing\Route
     */
    public function permanentRedirect($uri, $destination)
    {}

    /**
     * Register a new route that returns a view.
     *
     * @param  string  $uri
     * @param  string  $view
     * @param  array  $data
     * @param  int|array  $status
     * @param  array  $headers
     * @return \Illuminate\Routing\Route
     */
    public function view($uri, $view, $data = [], $status = 200, array $headers = [])
    {}

    /**
     * Register a new route with the given verbs.
     *
     * @param  array|string  $methods
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function match($methods, $uri, $action = null)
    {}

    /**
     * Register an array of resource controllers.
     *
     * @param  array  $resources
     * @param  array  $options
     * @return void
     */
    public function resources(array $resources, array $options = [])
    {}

    /**
     * Register an array of resource controllers that can be soft deleted.
     *
     * @param  array  $resources
     * @param  array  $options
     * @return void
     */
    public function softDeletableResources(array $resources, array $options = [])
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
     * Register an array of API resource controllers.
     *
     * @param  array  $resources
     * @param  array  $options
     * @return void
     */
    public function apiResources(array $resources, array $options = [])
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
     * Register an array of singleton resource controllers.
     *
     * @param  array  $singletons
     * @param  array  $options
     * @return void
     */
    public function singletons(array $singletons, array $options = [])
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
     * Register an array of API singleton resource controllers.
     *
     * @param  array  $singletons
     * @param  array  $options
     * @return void
     */
    public function apiSingletons(array $singletons, array $options = [])
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
     * @param  array  $attributes
     * @param  \Closure|array|string  $routes
     * @return $this
     */
    public function group(array $attributes, $routes)
    {}

    /**
     * Update the group stack with the given attributes.
     *
     * @param  array  $attributes
     * @return void
     */
    protected function updateGroupStack(array $attributes)
    {}

    /**
     * Merge the given array with the last group stack.
     *
     * @param  array  $new
     * @param  bool  $prependExistingPrefix
     * @return array
     */
    public function mergeWithLastGroup($new, $prependExistingPrefix = true)
    {}

    /**
     * Load the provided routes.
     *
     * @param  \Closure|string  $routes
     * @return void
     */
    protected function loadRoutes($routes)
    {}

    /**
     * Get the prefix from the last group on the stack.
     *
     * @return string
     */
    public function getLastGroupPrefix()
    {}

    /**
     * Add a route to the underlying route collection.
     *
     * @param  array|string  $methods
     * @param  string  $uri
     * @param  array|string|callable|null  $action
     * @return \Illuminate\Routing\Route
     */
    public function addRoute($methods, $uri, $action)
    {}

    /**
     * Create a new route instance.
     *
     * @param  array|string  $methods
     * @param  string  $uri
     * @param  mixed  $action
     * @return \Illuminate\Routing\Route
     */
    protected function createRoute($methods, $uri, $action)
    {}

    /**
     * Determine if the action is routing to a controller.
     *
     * @param  mixed  $action
     * @return bool
     */
    protected function actionReferencesController($action)
    {}

    /**
     * Add a controller based route action to the action array.
     *
     * @param  array|string  $action
     * @return array
     */
    protected function convertToControllerAction($action)
    {}

    /**
     * Prepend the last group namespace onto the use clause.
     *
     * @param  string  $class
     * @return string
     */
    protected function prependGroupNamespace($class)
    {}

    /**
     * Prepend the last group controller onto the use clause.
     *
     * @param  string  $class
     * @return string
     */
    protected function prependGroupController($class)
    {}

    /**
     * Create a new Route object.
     *
     * @param  array|string  $methods
     * @param  string  $uri
     * @param  mixed  $action
     * @return \Illuminate\Routing\Route
     */
    public function newRoute($methods, $uri, $action)
    {}

    /**
     * Prefix the given URI with the last prefix.
     *
     * @param  string  $uri
     * @return string
     */
    protected function prefix($uri)
    {}

    /**
     * Add the necessary where clauses to the route based on its initial registration.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return \Illuminate\Routing\Route
     */
    protected function addWhereClausesToRoute($route)
    {}

    /**
     * Merge the group stack with the controller action.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return void
     */
    protected function mergeGroupAttributesIntoRoute($route)
    {}

    /**
     * Return the response returned by the given route.
     *
     * @param  string  $name
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public function respondWithRoute($name)
    {}

    /**
     * Dispatch the request to the application.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public function dispatch(Request $request)
    {}

    /**
     * Dispatch the request to a route and return the response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public function dispatchToRoute(Request $request)
    {}

    /**
     * Find the route matching a given request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Routing\Route
     */
    protected function findRoute($request)
    {}

    /**
     * Return the response for the given route.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Illuminate\Routing\Route  $route
     * @return \Symfony\Component\HttpFoundation\Response
     */
    protected function runRoute(Request $request, Route $route)
    {}

    /**
     * Run the given route within a Stack "onion" instance.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @param  \Illuminate\Http\Request  $request
     * @return mixed
     */
    protected function runRouteWithinStack(Route $route, Request $request)
    {}

    /**
     * Gather the middleware for the given route with resolved class names.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return array
     */
    public function gatherRouteMiddleware(Route $route)
    {}

    /**
     * Resolve a flat array of middleware classes from the provided array.
     *
     * @param  array  $middleware
     * @param  array  $excluded
     * @return array
     */
    public function resolveMiddleware(array $middleware, array $excluded = [])
    {}

    /**
     * Sort the given middleware by priority.
     *
     * @param  \Illuminate\Support\Collection  $middlewares
     * @return array
     */
    protected function sortMiddleware(Collection $middlewares)
    {}

    /**
     * Create a response instance from the given value.
     *
     * @param  \Symfony\Component\HttpFoundation\Request  $request
     * @param  mixed  $response
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public function prepareResponse($request, $response)
    {}

    /**
     * Static version of prepareResponse.
     *
     * @param  \Symfony\Component\HttpFoundation\Request  $request
     * @param  mixed  $response
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public static function toResponse($request, $response)
    {}

    /**
     * Substitute the route bindings onto the route.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return \Illuminate\Routing\Route
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<\Illuminate\Database\Eloquent\Model>
     * @throws \Illuminate\Routing\Exceptions\BackedEnumCaseNotFoundException
     */
    public function substituteBindings($route)
    {}

    /**
     * Substitute the implicit route bindings for the given route.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return void
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<\Illuminate\Database\Eloquent\Model>
     * @throws \Illuminate\Routing\Exceptions\BackedEnumCaseNotFoundException
     */
    public function substituteImplicitBindings($route)
    {}

    /**
     * Register a callback to run after implicit bindings are substituted.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function substituteImplicitBindingsUsing($callback)
    {}

    /**
     * Call the binding callback for the given key.
     *
     * @param  string  $key
     * @param  string  $value
     * @param  \Illuminate\Routing\Route  $route
     * @return mixed
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<\Illuminate\Database\Eloquent\Model>
     */
    protected function performBinding($key, $value, $route)
    {}

    /**
     * Register a route matched event listener.
     *
     * @param  string|callable  $callback
     * @return void
     */
    public function matched($callback)
    {}

    /**
     * Get all of the defined middleware short-hand names.
     *
     * @return array
     */
    public function getMiddleware()
    {}

    /**
     * Register a short-hand name for a middleware.
     *
     * @param  string  $name
     * @param  string  $class
     * @return $this
     */
    public function aliasMiddleware($name, $class)
    {}

    /**
     * Check if a middlewareGroup with the given name exists.
     *
     * @param  string  $name
     * @return bool
     */
    public function hasMiddlewareGroup($name)
    {}

    /**
     * Get all of the defined middleware groups.
     *
     * @return array
     */
    public function getMiddlewareGroups()
    {}

    /**
     * Register a group of middleware.
     *
     * @param  string  $name
     * @param  array  $middleware
     * @return $this
     */
    public function middlewareGroup($name, array $middleware)
    {}

    /**
     * Add a middleware to the beginning of a middleware group.
     *
     * If the middleware is already in the group, it will not be added again.
     *
     * @param  string  $group
     * @param  string  $middleware
     * @return $this
     */
    public function prependMiddlewareToGroup($group, $middleware)
    {}

    /**
     * Add a middleware to the end of a middleware group.
     *
     * If the middleware is already in the group, it will not be added again.
     *
     * @param  string  $group
     * @param  string  $middleware
     * @return $this
     */
    public function pushMiddlewareToGroup($group, $middleware)
    {}

    /**
     * Remove the given middleware from the specified group.
     *
     * @param  string  $group
     * @param  string  $middleware
     * @return $this
     */
    public function removeMiddlewareFromGroup($group, $middleware)
    {}

    /**
     * Flush the router's middleware groups.
     *
     * @return $this
     */
    public function flushMiddlewareGroups()
    {}

    /**
     * Add a new route parameter binder.
     *
     * @param  string  $key
     * @param  string|callable  $binder
     * @return void
     */
    public function bind($key, $binder)
    {}

    /**
     * Register a model binder for a wildcard.
     *
     * @param  string  $key
     * @param  string  $class
     * @param  \Closure|null  $callback
     * @return void
     */
    public function model($key, $class, ?Closure $callback = null)
    {}

    /**
     * Get the binding callback for a given binding.
     *
     * @param  string  $key
     * @return \Closure|null
     */
    public function getBindingCallback($key)
    {}

    /**
     * Get the global "where" patterns.
     *
     * @return array
     */
    public function getPatterns()
    {}

    /**
     * Set a global where pattern on all routes.
     *
     * @param  string  $key
     * @param  string  $pattern
     * @return void
     */
    public function pattern($key, $pattern)
    {}

    /**
     * Set a group of global where patterns on all routes.
     *
     * @param  array  $patterns
     * @return void
     */
    public function patterns($patterns)
    {}

    /**
     * Determine if the router currently has a group stack.
     *
     * @return bool
     */
    public function hasGroupStack()
    {}

    /**
     * Get the current group stack for the router.
     *
     * @return array
     */
    public function getGroupStack()
    {}

    /**
     * Get a route parameter for the current route.
     *
     * @param  string  $key
     * @param  string|null  $default
     * @return mixed
     */
    public function input($key, $default = null)
    {}

    /**
     * Get the request currently being dispatched.
     *
     * @return \Illuminate\Http\Request
     */
    public function getCurrentRequest()
    {}

    /**
     * Get the currently dispatched route instance.
     *
     * @return \Illuminate\Routing\Route|null
     */
    public function getCurrentRoute()
    {}

    /**
     * Get the currently dispatched route instance.
     *
     * @return \Illuminate\Routing\Route|null
     */
    public function current()
    {}

    /**
     * Check if a route with the given name exists.
     *
     * @param  string|array  $name
     * @return bool
     */
    public function has($name)
    {}

    /**
     * Get the current route name.
     *
     * @return string|null
     */
    public function currentRouteName()
    {}

    /**
     * Alias for the "currentRouteNamed" method.
     *
     * @param  mixed  ...$patterns
     * @return bool
     */
    public function is(...$patterns)
    {}

    /**
     * Determine if the current route matches a pattern.
     *
     * @param  mixed  ...$patterns
     * @return bool
     */
    public function currentRouteNamed(...$patterns)
    {}

    /**
     * Get the current route action.
     *
     * @return string|null
     */
    public function currentRouteAction()
    {}

    /**
     * Alias for the "currentRouteUses" method.
     *
     * @param  array|string  ...$patterns
     * @return bool
     */
    public function uses(...$patterns)
    {}

    /**
     * Determine if the current route action matches a given action.
     *
     * @param  string  $action
     * @return bool
     */
    public function currentRouteUses($action)
    {}

    /**
     * Set the unmapped global resource parameters to singular.
     *
     * @param  bool  $singular
     * @return void
     */
    public function singularResourceParameters($singular = true)
    {}

    /**
     * Set the global resource parameter mapping.
     *
     * @param  array  $parameters
     * @return void
     */
    public function resourceParameters(array $parameters = [])
    {}

    /**
     * Get or set the verbs used in the resource URIs.
     *
     * @param  array  $verbs
     * @return array|null
     */
    public function resourceVerbs(array $verbs = [])
    {}

    /**
     * Get the underlying route collection.
     *
     * @return \Illuminate\Routing\RouteCollectionInterface
     */
    public function getRoutes()
    {}

    /**
     * Set the route collection instance.
     *
     * @param  \Illuminate\Routing\RouteCollection  $routes
     * @return void
     */
    public function setRoutes(RouteCollection $routes)
    {}

    /**
     * Set the compiled route collection instance.
     *
     * @param  array  $routes
     * @return void
     */
    public function setCompiledRoutes(array $routes)
    {}

    /**
     * Remove any duplicate middleware from the given array.
     *
     * @param  array  $middleware
     * @return array
     */
    public static function uniqueMiddleware(array $middleware)
    {}

    /**
     * Set the container instance used by the router.
     *
     * @param  \Illuminate\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}

    /**
     * Dynamically handle calls into the router instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
