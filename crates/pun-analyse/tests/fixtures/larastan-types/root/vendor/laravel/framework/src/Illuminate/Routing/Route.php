<?php

namespace Illuminate\Routing;

use BackedEnum;
use Closure;
use Illuminate\Container\Container;
use Illuminate\Http\Exceptions\HttpResponseException;
use Illuminate\Http\Request;
use Illuminate\Routing\Attributes\Controllers\Middleware as MiddlewareAttribute;
use Illuminate\Routing\Attributes\Controllers\WithoutMiddleware;
use Illuminate\Routing\Contracts\CallableDispatcher;
use Illuminate\Routing\Contracts\ControllerDispatcher as ControllerDispatcherContract;
use Illuminate\Routing\Controllers\HasMiddleware;
use Illuminate\Routing\Controllers\Middleware;
use Illuminate\Routing\Matching\HostValidator;
use Illuminate\Routing\Matching\MethodValidator;
use Illuminate\Routing\Matching\SchemeValidator;
use Illuminate\Routing\Matching\UriValidator;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use Laravel\SerializableClosure\SerializableClosure;
use LogicException;
use ReflectionAttribute;
use ReflectionClass;
use ReflectionException;
use Symfony\Component\Routing\Route as SymfonyRoute;

use function Illuminate\Support\enum_value;

class Route
{
    use Conditionable, CreatesRegularExpressionRouteConstraints, FiltersControllerMiddleware, Macroable, ResolvesRouteDependencies;

    /**
     * The URI pattern the route responds to.
     *
     * @var string
     */
    public $uri;

    /**
     * The HTTP methods the route responds to.
     *
     * @var array
     */
    public $methods;

    /**
     * The route action array.
     *
     * @var array
     */
    public $action;

    /**
     * Indicates whether the route is a fallback route.
     *
     * @var bool
     */
    public $isFallback = false;

    /**
     * The controller instance.
     *
     * @var mixed
     */
    public $controller;

    /**
     * The default values for the route.
     *
     * @var array
     */
    public $defaults = [];

    /**
     * The regular expression requirements.
     *
     * @var array
     */
    public $wheres = [];

    /**
     * The array of matched parameters.
     *
     * @var array|null
     */
    public $parameters;

    /**
     * The parameter names for the route.
     *
     * @var array|null
     */
    public $parameterNames;

    /**
     * The array of the matched parameters' original values.
     *
     * @var array|null
     */
    protected $originalParameters;

    /**
     * Indicates "trashed" models can be retrieved when resolving implicit model bindings for this route.
     *
     * @var bool
     */
    protected $withTrashedBindings = false;

    /**
     * Indicates the maximum number of seconds the route should acquire a session lock for.
     *
     * @var int|null
     */
    protected $lockSeconds;

    /**
     * Indicates the maximum number of seconds the route should wait while attempting to acquire a session lock.
     *
     * @var int|null
     */
    protected $waitSeconds;

    /**
     * The computed gathered middleware.
     *
     * @var array|null
     */
    public $computedMiddleware;

    /**
     * The compiled version of the route.
     *
     * @var \Symfony\Component\Routing\CompiledRoute
     */
    public $compiled;

    /**
     * The router instance used by the route.
     *
     * @var \Illuminate\Routing\Router
     */
    protected $router;

    /**
     * The container instance used by the route.
     *
     * @var \Illuminate\Container\Container
     */
    protected $container;

    /**
     * The fields that implicit binding should use for a given parameter.
     *
     * @var array
     */
    protected $bindingFields = [];

    /**
     * The validators used by the routes.
     *
     * @var array|null
     */
    public static $validators;

    /**
     * Create a new Route instance.
     *
     * @param  array|string  $methods
     * @param  string  $uri
     * @param  \Closure|array  $action
     */
    public function __construct($methods, $uri, $action)
    {}

    /**
     * Parse the route action into a standard array.
     *
     * @param  callable|array|null  $action
     * @return array
     *
     * @throws \UnexpectedValueException
     */
    protected function parseAction($action)
    {}

    /**
     * Run the route action and return the response.
     *
     * @return mixed
     */
    public function run()
    {}

    /**
     * Checks whether the route's action is a controller.
     *
     * @return bool
     */
    protected function isControllerAction()
    {}

    /**
     * Run the route action and return the response.
     *
     * @return mixed
     */
    protected function runCallable()
    {}

    /**
     * Determine if the route action is a serialized Closure.
     *
     * @return bool
     */
    protected function isSerializedClosure()
    {}

    /**
     * Run the route action and return the response.
     *
     * @return mixed
     *
     * @throws \Symfony\Component\HttpKernel\Exception\NotFoundHttpException
     */
    protected function runController()
    {}

    /**
     * Get the controller instance for the route.
     *
     * @return mixed
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function getController()
    {}

    /**
     * Get the controller class used for the route.
     *
     * @return string|null
     */
    public function getControllerClass()
    {}

    /**
     * Get the controller method used for the route.
     *
     * @return string
     */
    protected function getControllerMethod()
    {}

    /**
     * Parse the controller.
     *
     * @return array
     */
    protected function parseControllerCallback()
    {}

    /**
     * Flush the cached container instance on the route.
     *
     * @return void
     */
    public function flushController()
    {}

    /**
     * Determine if the route matches a given request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  bool  $includingMethod
     * @return bool
     */
    public function matches(Request $request, $includingMethod = true)
    {}

    /**
     * Compile the route into a Symfony CompiledRoute instance.
     *
     * @return \Symfony\Component\Routing\CompiledRoute
     */
    protected function compileRoute()
    {}

    /**
     * Bind the route to a given request for execution.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return $this
     */
    public function bind(Request $request)
    {}

    /**
     * Determine if the route has parameters.
     *
     * @return bool
     */
    public function hasParameters()
    {}

    /**
     * Determine a given parameter exists from the route.
     *
     * @param  string  $name
     * @return bool
     */
    public function hasParameter($name)
    {}

    /**
     * Get a given parameter from the route.
     *
     * @param  string  $name
     * @param  string|object|null  $default
     * @return string|object|null
     */
    public function parameter($name, $default = null)
    {}

    /**
     * Get original value of a given parameter from the route.
     *
     * @param  string  $name
     * @param  string|null  $default
     * @return string|null
     */
    public function originalParameter($name, $default = null)
    {}

    /**
     * Set a parameter to the given value.
     *
     * @param  string  $name
     * @param  string|object|null  $value
     * @return void
     */
    public function setParameter($name, $value)
    {}

    /**
     * Unset a parameter on the route if it is set.
     *
     * @param  string  $name
     * @return void
     */
    public function forgetParameter($name)
    {}

    /**
     * Get the key / value list of parameters for the route.
     *
     * @return array
     *
     * @throws \LogicException
     */
    public function parameters()
    {}

    /**
     * Get the key / value list of original parameters for the route.
     *
     * @return array
     *
     * @throws \LogicException
     */
    public function originalParameters()
    {}

    /**
     * Get the key / value list of parameters without null values.
     *
     * @return array
     */
    public function parametersWithoutNulls()
    {}

    /**
     * Get all of the parameter names for the route.
     *
     * @return array
     */
    public function parameterNames()
    {}

    /**
     * Get the parameter names for the route.
     *
     * @return array
     */
    protected function compileParameterNames()
    {}

    /**
     * Get the parameters that are listed in the route / controller signature.
     *
     * @param  array  $conditions
     * @return array
     */
    public function signatureParameters($conditions = [])
    {}

    /**
     * Get the binding field for the given parameter.
     *
     * @param  string|int  $parameter
     * @return string|null
     */
    public function bindingFieldFor($parameter)
    {}

    /**
     * Get the binding fields for the route.
     *
     * @return array
     */
    public function bindingFields()
    {}

    /**
     * Set the binding fields for the route.
     *
     * @param  array  $bindingFields
     * @return $this
     */
    public function setBindingFields(array $bindingFields)
    {}

    /**
     * Get the parent parameter of the given parameter.
     *
     * @param  string  $parameter
     * @return string|null
     */
    public function parentOfParameter($parameter)
    {}

    /**
     * Allow "trashed" models to be retrieved when resolving implicit model bindings for this route.
     *
     * @param  bool  $withTrashed
     * @return $this
     */
    public function withTrashed($withTrashed = true)
    {}

    /**
     * Determines if the route allows "trashed" models to be retrieved when resolving implicit model bindings.
     *
     * @return bool
     */
    public function allowsTrashedBindings()
    {}

    /**
     * Set a default value for the route.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return $this
     */
    public function defaults($key, $value)
    {}

    /**
     * Set the default values for the route.
     *
     * @param  array  $defaults
     * @return $this
     */
    public function setDefaults(array $defaults)
    {}

    /**
     * Set a regular expression requirement on the route.
     *
     * @param  array|string  $name
     * @param  string|null  $expression
     * @return $this
     */
    public function where($name, $expression = null)
    {}

    /**
     * Parse arguments to the where method into an array.
     *
     * @param  array|string  $name
     * @param  string  $expression
     * @return array
     */
    protected function parseWhere($name, $expression)
    {}

    /**
     * Set a list of regular expression requirements on the route.
     *
     * @param  array  $wheres
     * @return $this
     */
    public function setWheres(array $wheres)
    {}

    /**
     * Mark this route as a fallback route.
     *
     * @return $this
     */
    public function fallback()
    {}

    /**
     * Set the fallback value.
     *
     * @param  bool  $isFallback
     * @return $this
     */
    public function setFallback($isFallback)
    {}

    /**
     * Get the HTTP verbs the route responds to.
     *
     * @return array
     */
    public function methods()
    {}

    /**
     * Determine if the route only responds to HTTP requests.
     *
     * @return bool
     */
    public function httpOnly()
    {}

    /**
     * Determine if the route only responds to HTTPS requests.
     *
     * @return bool
     */
    public function httpsOnly()
    {}

    /**
     * Determine if the route only responds to HTTPS requests.
     *
     * @return bool
     */
    public function secure()
    {}

    /**
     * Get or set the domain for the route.
     *
     * @param  \BackedEnum|string|null  $domain
     * @return ($domain is null ? string|null : $this)
     *
     * @throws \InvalidArgumentException
     */
    public function domain($domain = null)
    {}

    /**
     * Get the domain defined for the route.
     *
     * @return string|null
     */
    public function getDomain()
    {}

    /**
     * Get the prefix of the route instance.
     *
     * @return string|null
     */
    public function getPrefix()
    {}

    /**
     * Add a prefix to the route URI.
     *
     * @param  string|null  $prefix
     * @return $this
     */
    public function prefix($prefix)
    {}

    /**
     * Update the "prefix" attribute on the action array.
     *
     * @param  string  $prefix
     * @return void
     */
    protected function updatePrefixOnAction($prefix)
    {}

    /**
     * Get the URI associated with the route.
     *
     * @return string
     */
    public function uri()
    {}

    /**
     * Set the URI that the route responds to.
     *
     * @param  string  $uri
     * @return $this
     */
    public function setUri($uri)
    {}

    /**
     * Parse the route URI and normalize / store any implicit binding fields.
     *
     * @param  string  $uri
     * @return string
     */
    protected function parseUri($uri)
    {}

    /**
     * Get the name of the route instance.
     *
     * @return string|null
     */
    public function getName()
    {}

    /**
     * Add or change the route name.
     *
     * @param  \BackedEnum|string  $name
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function name($name)
    {}

    /**
     * Determine whether the route's name matches the given patterns.
     *
     * @param  mixed  ...$patterns
     * @return bool
     */
    public function named(...$patterns)
    {}

    /**
     * Set the handler for the route.
     *
     * @param  \Closure|array|string  $action
     * @return $this
     */
    public function uses($action)
    {}

    /**
     * Parse a string based action for the "uses" fluent method.
     *
     * @param  string  $action
     * @return string
     */
    protected function addGroupNamespaceToStringUses($action)
    {}

    /**
     * Get the action name for the route.
     *
     * @return string
     */
    public function getActionName()
    {}

    /**
     * Get the method name of the route action.
     *
     * @return string
     */
    public function getActionMethod()
    {}

    /**
     * Get the action array or one of its properties for the route.
     *
     * @param  string|null  $key
     * @return mixed
     */
    public function getAction($key = null)
    {}

    /**
     * Set the action array for the route.
     *
     * @param  array  $action
     * @return $this
     */
    public function setAction(array $action)
    {}

    /**
     * Get the value of the action that should be taken on a missing model exception.
     *
     * @return \Closure|null
     */
    public function getMissing()
    {}

    /**
     * Define the callable that should be invoked on a missing model exception.
     *
     * @param  \Closure  $missing
     * @return $this
     */
    public function missing($missing)
    {}

    /**
     * Get all middleware, including the ones from the controller.
     *
     * @return array
     */
    public function gatherMiddleware()
    {}

    /**
     * Get or set the middlewares attached to the route.
     *
     * @param  array|string|null  $middleware
     * @return ($middleware is null ? array : $this)
     */
    public function middleware($middleware = null)
    {}

    /**
     * Specify that the "Authorize" / "can" middleware should be applied to the route with the given options.
     *
     * @param  \UnitEnum|string  $ability
     * @param  array|string  $models
     * @return $this
     */
    public function can($ability, $models = [])
    {}

    /**
     * Get the middleware for the route's controller.
     *
     * @return array
     */
    public function controllerMiddleware()
    {}

    /**
     * Get the statically provided controller middleware for the given class and method.
     *
     * @param  string  $class
     * @param  string  $method
     * @return array
     */
    protected function staticallyProvidedControllerMiddleware(string $class, string $method)
    {}

    /**
     * Get the attribute provided controller middleware for the given class and method.
     *
     * @return array
     */
    protected function attributeProvidedControllerMiddleware(string $class, string $method)
    {}

    /**
     * Get the excluded middleware for the route's controller.
     *
     * @return array
     */
    public function excludedControllerMiddleware()
    {}

    /**
     * Get the attribute provided excluded controller middleware for the given class and method.
     *
     * @param  string  $class
     * @param  string  $method
     * @return array
     */
    protected function attributeProvidedControllerMiddlewareExclusions(string $class, string $method): array
    {}

    /**
     * Specify middleware that should be removed from the given route.
     *
     * @param  array|string  $middleware
     * @return $this
     */
    public function withoutMiddleware($middleware)
    {}

    /**
     * Get the middleware that should be removed from the route.
     *
     * @return array
     */
    public function excludedMiddleware()
    {}

    /**
     * Indicate that the route should enforce scoping of multiple implicit Eloquent bindings.
     *
     * @return $this
     */
    public function scopeBindings()
    {}

    /**
     * Indicate that the route should not enforce scoping of multiple implicit Eloquent bindings.
     *
     * @return $this
     */
    public function withoutScopedBindings()
    {}

    /**
     * Determine if the route should enforce scoping of multiple implicit Eloquent bindings.
     *
     * @return bool
     */
    public function enforcesScopedBindings()
    {}

    /**
     * Determine if the route should prevent scoping of multiple implicit Eloquent bindings.
     *
     * @return bool
     */
    public function preventsScopedBindings()
    {}

    /**
     * Specify that the route should not allow concurrent requests from the same session.
     *
     * @param  int|null  $lockSeconds
     * @param  int|null  $waitSeconds
     * @return $this
     */
    public function block($lockSeconds = 10, $waitSeconds = 10)
    {}

    /**
     * Specify that the route should allow concurrent requests from the same session.
     *
     * @return $this
     */
    public function withoutBlocking()
    {}

    /**
     * Get the maximum number of seconds the route's session lock should be held for.
     *
     * @return int|null
     */
    public function locksFor()
    {}

    /**
     * Get the maximum number of seconds to wait while attempting to acquire a session lock.
     *
     * @return int|null
     */
    public function waitsFor()
    {}

    /**
     * Add metadata to the route.
     *
     * @param  array  $metadata
     * @return $this
     */
    public function metadata(array $metadata)
    {}

    /**
     * Get metadata for the route.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? array<array-key, mixed> : mixed)
     */
    public function getMetadata($key = null, $default = null)
    {}

    /**
     * Set the metadata for the route, replacing any existing metadata.
     *
     * @param  array  $metadata
     * @return $this
     */
    public function setMetadata(array $metadata)
    {}

    /**
     * Get the dispatcher for the route's controller.
     *
     * @return \Illuminate\Routing\Contracts\ControllerDispatcher
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function controllerDispatcher()
    {}

    /**
     * Get the route validators for the instance.
     *
     * @return array
     */
    public static function getValidators()
    {}

    /**
     * Convert the route to a Symfony route.
     *
     * @return \Symfony\Component\Routing\Route
     */
    public function toSymfonyRoute()
    {}

    /**
     * Get the optional parameter names for the route.
     *
     * @return array<string, null>
     */
    public function getOptionalParameterNames()
    {}

    /**
     * Get the compiled version of the route.
     *
     * @return \Symfony\Component\Routing\CompiledRoute
     */
    public function getCompiled()
    {}

    /**
     * Set the router instance on the route.
     *
     * @param  \Illuminate\Routing\Router  $router
     * @return $this
     */
    public function setRouter(Router $router)
    {}

    /**
     * Set the container instance on the route.
     *
     * @param  \Illuminate\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}

    /**
     * Prepare the route instance for serialization.
     *
     * @return void
     *
     * @throws \LogicException
     */
    public function prepareForSerialization()
    {}

    /**
     * Dynamically access route parameters.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}
}
