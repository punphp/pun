<?php

namespace Illuminate\Container;

use ArrayAccess;
use Closure;
use Exception;
use Illuminate\Container\Attributes\Bind;
use Illuminate\Container\Attributes\BindWhen;
use Illuminate\Container\Attributes\Scoped;
use Illuminate\Container\Attributes\Singleton;
use Illuminate\Contracts\Container\BindingResolutionException;
use Illuminate\Contracts\Container\CircularDependencyException;
use Illuminate\Contracts\Container\Container as ContainerContract;
use Illuminate\Contracts\Container\ContextualAttribute;
use Illuminate\Contracts\Container\SelfBuilding;
use Illuminate\Support\Traits\ReflectsClosures;
use LogicException;
use ReflectionAttribute;
use ReflectionClass;
use ReflectionException;
use ReflectionFunction;
use ReflectionParameter;
use TypeError;

class Container implements ArrayAccess, ContainerContract
{
    use ReflectsClosures;

    /**
     * The current globally available container (if any).
     *
     * @var static|null
     */
    protected static $instance;

    /**
     * An array of the types that have been resolved.
     *
     * @var bool[]
     */
    protected $resolved = [];

    /**
     * The container's bindings.
     *
     * @var array[]
     */
    protected $bindings = [];

    /**
     * The container's method bindings.
     *
     * @var \Closure[]
     */
    protected $methodBindings = [];

    /**
     * The container's shared instances.
     *
     * @var object[]
     */
    protected $instances = [];

    /**
     * The container's scoped instances.
     *
     * @var array
     */
    protected $scopedInstances = [];

    /**
     * The registered type aliases.
     *
     * @var string[]
     */
    protected $aliases = [];

    /**
     * The registered aliases keyed by the abstract name.
     *
     * @var array[]
     */
    protected $abstractAliases = [];

    /**
     * The extension closures for services.
     *
     * @var array[]
     */
    protected $extenders = [];

    /**
     * All of the registered tags.
     *
     * @var array[]
     */
    protected $tags = [];

    /**
     * The stack of concretions currently being built.
     *
     * @var array<int, string|int>
     */
    protected $buildStack = [];

    /**
     * The parameter override stack.
     *
     * @var array[]
     */
    protected $with = [];

    /**
     * The contextual binding map.
     *
     * @var array[]
     */
    public $contextual = [];

    /**
     * The contextual attribute handlers.
     *
     * @var array[]
     */
    public $contextualAttributes = [];

    /**
     * Whether an abstract class has already had its attributes checked for bindings.
     *
     * @var array<class-string, true>
     */
    protected $checkedForAttributeBindings = [];

    /**
     * Whether a class has already been checked for Singleton or Scoped attributes.
     *
     * @var array<class-string, "scoped"|"singleton"|null>
     */
    protected $checkedForSingletonOrScopedAttributes = [];

    /**
     * All of the registered rebound callbacks.
     *
     * @var array[]
     */
    protected $reboundCallbacks = [];

    /**
     * All of the global before resolving callbacks.
     *
     * @var \Closure[]
     */
    protected $globalBeforeResolvingCallbacks = [];

    /**
     * All of the global resolving callbacks.
     *
     * @var \Closure[]
     */
    protected $globalResolvingCallbacks = [];

    /**
     * All of the global after resolving callbacks.
     *
     * @var \Closure[]
     */
    protected $globalAfterResolvingCallbacks = [];

    /**
     * All of the before resolving callbacks by class type.
     *
     * @var array[]
     */
    protected $beforeResolvingCallbacks = [];

    /**
     * All of the resolving callbacks by class type.
     *
     * @var array[]
     */
    protected $resolvingCallbacks = [];

    /**
     * All of the after resolving callbacks by class type.
     *
     * @var array[]
     */
    protected $afterResolvingCallbacks = [];

    /**
     * All of the after resolving attribute callbacks by class type.
     *
     * @var array[]
     */
    protected $afterResolvingAttributeCallbacks = [];

    /**
     * The callback used to determine the container's environment.
     *
     * @var (callable(array<int, string>|string): (bool|string))|null
     */
    protected $environmentResolver = null;

    /**
     * Define a contextual binding.
     *
     * @param  array|string  $concrete
     * @return \Illuminate\Contracts\Container\ContextualBindingBuilder
     */
    public function when($concrete)
    {}

    /**
     * Define a contextual binding based on an attribute.
     *
     * @return void
     */
    public function whenHasAttribute(string $attribute, Closure $handler)
    {}

    /**
     * Determine if the given abstract type has been bound.
     *
     * @param  string  $abstract
     * @return bool
     */
    public function bound($abstract)
    {}

    /**
     * {@inheritdoc}
     */
    public function has(string $id): bool
    {}

    /**
     * Determine if the given abstract type has been resolved.
     *
     * @param  string  $abstract
     * @return bool
     */
    public function resolved($abstract)
    {}

    /**
     * Determine if a given type is shared.
     *
     * @param  string  $abstract
     * @return bool
     */
    public function isShared($abstract)
    {}

    /**
     * Determine if a ReflectionClass has scoping attributes applied.
     *
     * @param  ReflectionClass<object>|class-string  $reflection
     * @return "singleton"|"scoped"|null
     */
    protected function getScopedTyped(ReflectionClass|string $reflection): ?string
    {}

    /**
     * Determine if a given string is an alias.
     *
     * @param  string  $name
     * @return bool
     */
    public function isAlias($name)
    {}

    /**
     * Register a binding with the container.
     *
     * @param  \Closure|string  $abstract
     * @param  \Closure|string|null  $concrete
     * @param  bool  $shared
     * @return void
     *
     * @throws \TypeError
     * @throws ReflectionException
     */
    public function bind($abstract, $concrete = null, $shared = false)
    {}

    /**
     * Get the Closure to be used when building a type.
     *
     * @param  string  $abstract
     * @param  string  $concrete
     * @return \Closure
     */
    protected function getClosure($abstract, $concrete)
    {}

    /**
     * Determine if the container has a method binding.
     *
     * @param  string  $method
     * @return bool
     */
    public function hasMethodBinding($method)
    {}

    /**
     * Bind a callback to resolve with Container::call.
     *
     * @param  array|string  $method
     * @param  \Closure  $callback
     * @return void
     */
    public function bindMethod($method, $callback)
    {}

    /**
     * Get the method to be bound in class@method format.
     *
     * @param  array|string  $method
     * @return string
     */
    protected function parseBindMethod($method)
    {}

    /**
     * Get the method binding for the given method.
     *
     * @param  string  $method
     * @param  mixed  $instance
     * @return mixed
     */
    public function callMethodBinding($method, $instance)
    {}

    /**
     * Add a contextual binding to the container.
     *
     * @param  string  $concrete
     * @param  \Closure|string  $abstract
     * @param  \Closure|string  $implementation
     * @return void
     */
    public function addContextualBinding($concrete, $abstract, $implementation)
    {}

    /**
     * Register a binding if it hasn't already been registered.
     *
     * @param  \Closure|string  $abstract
     * @param  \Closure|string|null  $concrete
     * @param  bool  $shared
     * @return void
     */
    public function bindIf($abstract, $concrete = null, $shared = false)
    {}

    /**
     * Register a shared binding in the container.
     *
     * @param  \Closure|string  $abstract
     * @param  \Closure|string|null  $concrete
     * @return void
     */
    public function singleton($abstract, $concrete = null)
    {}

    /**
     * Register a shared binding if it hasn't already been registered.
     *
     * @param  \Closure|string  $abstract
     * @param  \Closure|string|null  $concrete
     * @return void
     */
    public function singletonIf($abstract, $concrete = null)
    {}

    /**
     * Register a scoped binding in the container.
     *
     * @param  \Closure|string  $abstract
     * @param  \Closure|string|null  $concrete
     * @return void
     */
    public function scoped($abstract, $concrete = null)
    {}

    /**
     * Register a scoped binding if it hasn't already been registered.
     *
     * @param  \Closure|string  $abstract
     * @param  \Closure|string|null  $concrete
     * @return void
     */
    public function scopedIf($abstract, $concrete = null)
    {}

    /**
     * Register a binding with the container based on the given Closure's return types.
     *
     * @param  \Closure|string  $abstract
     * @param  \Closure|string|null  $concrete
     * @param  bool  $shared
     * @return void
     */
    protected function bindBasedOnClosureReturnTypes($abstract, $concrete = null, $shared = false)
    {}

    /**
     * "Extend" an abstract type in the container.
     *
     * @param  string  $abstract
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    public function extend($abstract, Closure $closure)
    {}

    /**
     * Register an existing instance as shared in the container.
     *
     * @template TInstance of mixed
     *
     * @param  string  $abstract
     * @param  TInstance  $instance
     * @return TInstance
     */
    public function instance($abstract, $instance)
    {}

    /**
     * Remove an alias from the contextual binding alias cache.
     *
     * @param  string  $searched
     * @return void
     */
    protected function removeAbstractAlias($searched)
    {}

    /**
     * Assign a set of tags to a given binding.
     *
     * @param  array|string  $abstracts
     * @param  mixed  ...$tags
     * @return void
     */
    public function tag($abstracts, $tags)
    { func_get_args(); }

    /**
     * Resolve all of the bindings for a given tag.
     *
     * @param  string  $tag
     * @return iterable
     */
    public function tagged($tag)
    {}

    /**
     * Alias a type to a different name.
     *
     * @param  string  $abstract
     * @param  string  $alias
     * @return void
     *
     * @throws \LogicException
     */
    public function alias($abstract, $alias)
    {}

    /**
     * Bind a new callback to an abstract's rebind event.
     *
     * @param  string  $abstract
     * @return mixed
     */
    public function rebinding($abstract, Closure $callback)
    {}

    /**
     * Refresh an instance on the given target and method.
     *
     * @param  string  $abstract
     * @param  mixed  $target
     * @param  string  $method
     * @return mixed
     */
    public function refresh($abstract, $target, $method)
    {}

    /**
     * Fire the "rebound" callbacks for the given abstract type.
     *
     * @param  string  $abstract
     * @return void
     */
    protected function rebound($abstract)
    {}

    /**
     * Get the rebound callbacks for a given type.
     *
     * @param  string  $abstract
     * @return array
     */
    protected function getReboundCallbacks($abstract)
    {}

    /**
     * Wrap the given closure such that its dependencies will be injected when executed.
     *
     * @return \Closure
     */
    public function wrap(Closure $callback, array $parameters = [])
    {}

    /**
     * Call the given Closure / class@method and inject its dependencies.
     *
     * @param  callable|string  $callback
     * @param  array<string, mixed>  $parameters
     * @param  string|null  $defaultMethod
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    public function call($callback, array $parameters = [], $defaultMethod = null)
    {}

    /**
     * Get the class name for the given callback, if one can be determined.
     *
     * @param  callable|string  $callback
     * @return string|false
     */
    protected function getClassForCallable($callback)
    {}

    /**
     * Get a closure to resolve the given type from the container.
     *
     * @template TClass of object
     *
     * @param  string|class-string<TClass>  $abstract
     * @return ($abstract is class-string<TClass> ? \Closure(): TClass : \Closure(): mixed)
     */
    public function factory($abstract)
    {}

    /**
     * An alias function name for make().
     *
     * @template TClass of object
     *
     * @param  string|class-string<TClass>|callable  $abstract
     * @return ($abstract is class-string<TClass> ? TClass : mixed)
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function makeWith($abstract, array $parameters = [])
    {}

    /**
     * Resolve the given type from the container.
     *
     * @template TClass of object
     *
     * @param  string|class-string<TClass>  $abstract
     * @return ($abstract is class-string<TClass> ? TClass : mixed)
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function make($abstract, array $parameters = [])
    {}

    /**
     * {@inheritdoc}
     *
     * @template TClass of object
     *
     * @param  string|class-string<TClass>  $id
     * @return ($id is class-string<TClass> ? TClass : mixed)
     *
     * @throws \Illuminate\Contracts\Container\CircularDependencyException
     * @throws \Illuminate\Container\EntryNotFoundException
     */
    public function get(string $id)
    {}

    /**
     * Resolve the given type from the container.
     *
     * @template TClass of object
     *
     * @param  string|class-string<TClass>|callable  $abstract
     * @param  array  $parameters
     * @param  bool  $raiseEvents
     * @return ($abstract is class-string<TClass> ? TClass : mixed)
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     * @throws \Illuminate\Contracts\Container\CircularDependencyException
     */
    protected function resolve($abstract, $parameters = [], $raiseEvents = true)
    {}

    /**
     * Get the concrete type for a given abstract.
     *
     * @param  string|callable  $abstract
     * @return mixed
     */
    protected function getConcrete($abstract)
    {}

    /**
     * Get the concrete binding for an abstract from the BindWhen or Bind attributes.
     *
     * @param  string  $abstract
     * @return mixed
     */
    protected function getConcreteBindingFromAttributes($abstract)
    {}

    /**
     * Resolve the concrete from the Bind and BindWhen attributes in declaration order.
     *
     * @param  ReflectionClass<object>  $reflected
     * @return class-string|null
     */
    protected function resolveConcreteFromAttributes(ReflectionClass $reflected)
    {}

    /**
     * Get the contextual concrete binding for the given abstract.
     *
     * @param  string|callable  $abstract
     * @return \Closure|string|array|null
     */
    protected function getContextualConcrete($abstract)
    {}

    /**
     * Find the concrete binding for the given abstract in the contextual binding array.
     *
     * @param  string|callable  $abstract
     * @return \Closure|string|null
     */
    protected function findInContextualBindings($abstract)
    {}

    /**
     * Determine if the given concrete is buildable.
     *
     * @param  mixed  $concrete
     * @param  string  $abstract
     * @return bool
     */
    protected function isBuildable($concrete, $abstract)
    {}

    /**
     * Instantiate a concrete instance of the given type.
     *
     * @template TClass of object
     *
     * @param  \Closure(static, array): TClass|class-string<TClass>  $concrete
     * @return TClass
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     * @throws \Illuminate\Contracts\Container\CircularDependencyException
     */
    public function build($concrete)
    {}

    /**
     * Instantiate a concrete instance of the given self building type.
     *
     * @template TClass of object
     *
     * @param  object{'newInstance': \Closure(static, array): TClass|class-string<TClass>}  $concrete
     * @param  \ReflectionClass  $reflector
     * @return TClass
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function buildSelfBuildingInstance($concrete, $reflector)
    {}

    /**
     * Resolve all of the dependencies from the ReflectionParameters.
     *
     * @param  \ReflectionParameter[]  $dependencies
     * @return array
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function resolveDependencies(array $dependencies)
    {}

    /**
     * Determine if the given dependency has a parameter override.
     *
     * @param  \ReflectionParameter  $dependency
     * @return bool
     */
    protected function hasParameterOverride($dependency)
    {}

    /**
     * Get a parameter override for a dependency.
     *
     * @param  \ReflectionParameter  $dependency
     * @return mixed
     */
    protected function getParameterOverride($dependency)
    {}

    /**
     * Get the last parameter override.
     *
     * @return array
     */
    protected function getLastParameterOverride()
    {}

    /**
     * Resolve a non-class hinted primitive dependency.
     *
     * @return mixed
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function resolvePrimitive(ReflectionParameter $parameter)
    {}

    /**
     * Resolve a class based dependency from the container.
     *
     * @return mixed
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function resolveClass(ReflectionParameter $parameter, ?string $className = null)
    {}

    /**
     * Resolve a class based variadic dependency from the container.
     *
     * @return mixed
     */
    protected function resolveVariadicClass(ReflectionParameter $parameter)
    {}

    /**
     * Resolve a dependency based on an attribute.
     *
     * @return mixed
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function resolveFromAttribute(ReflectionAttribute $attribute, ReflectionParameter $parameter)
    {}

    /**
     * Throw an exception that the concrete is not instantiable.
     *
     * @param  string  $concrete
     * @return void
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function notInstantiable($concrete)
    {}

    /**
     * Throw an exception for an unresolvable primitive.
     *
     * @return void
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function unresolvablePrimitive(ReflectionParameter $parameter)
    {}

    /**
     * Register a new before resolving callback for all types.
     *
     * @param  \Closure|string  $abstract
     * @return void
     */
    public function beforeResolving($abstract, ?Closure $callback = null)
    {}

    /**
     * Register a new resolving callback.
     *
     * @param  \Closure|string  $abstract
     * @return void
     */
    public function resolving($abstract, ?Closure $callback = null)
    {}

    /**
     * Register a new after resolving callback for all types.
     *
     * @param  \Closure|string  $abstract
     * @return void
     */
    public function afterResolving($abstract, ?Closure $callback = null)
    {}

    /**
     * Register a new after resolving attribute callback for all types.
     *
     * @return void
     */
    public function afterResolvingAttribute(string $attribute, \Closure $callback)
    {}

    /**
     * Fire all of the before resolving callbacks.
     *
     * @param  string  $abstract
     * @param  array  $parameters
     * @return void
     */
    protected function fireBeforeResolvingCallbacks($abstract, $parameters = [])
    {}

    /**
     * Fire an array of callbacks with an object.
     *
     * @param  string  $abstract
     * @param  array  $parameters
     * @return void
     */
    protected function fireBeforeCallbackArray($abstract, $parameters, array $callbacks)
    {}

    /**
     * Fire all of the resolving callbacks.
     *
     * @param  string  $abstract
     * @param  mixed  $object
     * @return void
     */
    protected function fireResolvingCallbacks($abstract, $object)
    {}

    /**
     * Fire all of the after resolving callbacks.
     *
     * @param  string  $abstract
     * @param  mixed  $object
     * @return void
     */
    protected function fireAfterResolvingCallbacks($abstract, $object)
    {}

    /**
     * Fire all of the after resolving attribute callbacks.
     *
     * @param  \ReflectionAttribute[]  $attributes
     * @param  mixed  $object
     * @return void
     */
    public function fireAfterResolvingAttributeCallbacks(array $attributes, $object)
    {}

    /**
     * Get all callbacks for a given type.
     *
     * @param  string  $abstract
     * @param  object  $object
     * @return array
     */
    protected function getCallbacksForType($abstract, $object, array $callbacksPerType)
    {}

    /**
     * Fire an array of callbacks with an object.
     *
     * @param  mixed  $object
     * @return void
     */
    protected function fireCallbackArray($object, array $callbacks)
    {}

    /**
     * Get the name of the binding the container is currently resolving.
     *
     * @return class-string|string|int|null
     */
    public function currentlyResolving()
    {}

    /**
     * Get the container's bindings.
     *
     * @return array
     */
    public function getBindings()
    {}

    /**
     * Get the alias for an abstract if available.
     *
     * @param  string  $abstract
     * @return string
     *
     * @throws \LogicException
     */
    public function getAlias($abstract)
    {}

    /**
     * Get the extender callbacks for a given type.
     *
     * @param  string  $abstract
     * @return array
     */
    protected function getExtenders($abstract)
    {}

    /**
     * Remove all of the extender callbacks for a given type.
     *
     * @param  string  $abstract
     * @return void
     */
    public function forgetExtenders($abstract)
    {}

    /**
     * Drop all of the stale instances and aliases.
     *
     * @param  string  $abstract
     * @return void
     */
    protected function dropStaleInstances($abstract)
    {}

    /**
     * Remove a resolved instance from the instance cache.
     *
     * @param  string  $abstract
     * @return void
     */
    public function forgetInstance($abstract)
    {}

    /**
     * Clear all of the instances from the container.
     *
     * @return void
     */
    public function forgetInstances()
    {}

    /**
     * Clear all of the scoped instances from the container.
     *
     * @return void
     */
    public function forgetScopedInstances()
    {}

    /**
     * Set the callback which determines the current container environment.
     *
     * @param  (callable(array<int, string>|string): (bool|string))|null  $callback
     * @return void
     */
    public function resolveEnvironmentUsing(?callable $callback)
    {}

    /**
     * Determine the environment for the container.
     *
     * @param  array<int, string>|string  $environments
     * @return bool
     */
    public function currentEnvironmentIs($environments)
    {}

    /**
     * Flush the container of all bindings and resolved instances.
     *
     * @return void
     */
    public function flush()
    {}

    /**
     * Get the globally available instance of the container.
     *
     * @return static
     */
    public static function getInstance()
    {}

    /**
     * Set the shared instance of the container.
     *
     * @return \Illuminate\Contracts\Container\Container|static|null
     */
    public static function setInstance(?ContainerContract $container = null)
    {}

    /**
     * Determine if a given offset exists.
     *
     * @param  string  $offset
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value at a given offset.
     *
     * @param  string  $offset
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value at a given offset.
     *
     * @param  string  $offset
     * @param  mixed  $value
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the value at a given offset.
     *
     * @param  string  $offset
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Dynamically access container services.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}

    /**
     * Dynamically set container services.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return void
     */
    public function __set($key, $value)
    {}
}
