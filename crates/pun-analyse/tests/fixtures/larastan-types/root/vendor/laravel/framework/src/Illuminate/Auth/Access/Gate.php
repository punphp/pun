<?php

namespace Illuminate\Auth\Access;

use Closure;
use Exception;
use Illuminate\Auth\Access\Events\GateEvaluated;
use Illuminate\Contracts\Auth\Access\Gate as GateContract;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\Eloquent\Attributes\UsePolicy;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use InvalidArgumentException;
use ReflectionClass;
use ReflectionFunction;

use function Illuminate\Support\enum_value;

class Gate implements GateContract
{
    use HandlesAuthorization;

    /**
     * The container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The user resolver callable.
     *
     * @var callable
     */
    protected $userResolver;

    /**
     * All of the defined abilities.
     *
     * @var array
     */
    protected $abilities = [];

    /**
     * All of the defined policies.
     *
     * @var array
     */
    protected $policies = [];

    /**
     * All of the registered before callbacks.
     *
     * @var array
     */
    protected $beforeCallbacks = [];

    /**
     * All of the registered after callbacks.
     *
     * @var array
     */
    protected $afterCallbacks = [];

    /**
     * All of the defined abilities using class@method notation.
     *
     * @var array
     */
    protected $stringCallbacks = [];

    /**
     * The default denial response for gates and policies.
     *
     * @var \Illuminate\Auth\Access\Response|null
     */
    protected $defaultDenialResponse;

    /**
     * The callback to be used to guess policy names.
     *
     * @var callable|null
     */
    protected $guessPolicyNamesUsingCallback;

    /**
     * Create a new gate instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @param  callable  $userResolver
     * @param  array  $abilities
     * @param  array  $policies
     * @param  array  $beforeCallbacks
     * @param  array  $afterCallbacks
     * @param  callable|null  $guessPolicyNamesUsingCallback
     */
    public function __construct(
        Container $container,
        callable $userResolver,
        array $abilities = [],
        array $policies = [],
        array $beforeCallbacks = [],
        array $afterCallbacks = [],
        ?callable $guessPolicyNamesUsingCallback = null,
    ) {}

    /**
     * Determine if a given ability has been defined.
     *
     * @param  \UnitEnum|array|string  $ability
     * @return bool
     */
    public function has($ability)
    {}

    /**
     * Perform an on-demand authorization check. Throw an authorization exception if the condition or callback is false.
     *
     * @param  \Illuminate\Auth\Access\Response|\Closure|bool  $condition
     * @param  string|null  $message
     * @param  string|null  $code
     * @return \Illuminate\Auth\Access\Response
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    public function allowIf($condition, $message = null, $code = null)
    {}

    /**
     * Perform an on-demand authorization check. Throw an authorization exception if the condition or callback is true.
     *
     * @param  \Illuminate\Auth\Access\Response|\Closure|bool  $condition
     * @param  string|null  $message
     * @param  string|null  $code
     * @return \Illuminate\Auth\Access\Response
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    public function denyIf($condition, $message = null, $code = null)
    {}

    /**
     * Authorize a given condition or callback.
     *
     * @param  \Illuminate\Auth\Access\Response|\Closure|bool  $condition
     * @param  string|null  $message
     * @param  string|null  $code
     * @param  bool  $allowWhenResponseIs
     * @return \Illuminate\Auth\Access\Response
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    protected function authorizeOnDemand($condition, $message, $code, $allowWhenResponseIs)
    {}

    /**
     * Define a new ability.
     *
     * @param  \UnitEnum|string  $ability
     * @param  callable|array|string  $callback
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function define($ability, $callback)
    {}

    /**
     * Define abilities for a resource.
     *
     * @param  string  $name
     * @param  string  $class
     * @param  array|null  $abilities
     * @return $this
     */
    public function resource($name, $class, ?array $abilities = null)
    {}

    /**
     * Create the ability callback for a callback string.
     *
     * @param  string  $ability
     * @param  string  $callback
     * @return \Closure
     */
    protected function buildAbilityCallback($ability, $callback)
    {}

    /**
     * Define a policy class for a given class type.
     *
     * @param  string  $class
     * @param  string  $policy
     * @return $this
     */
    public function policy($class, $policy)
    {}

    /**
     * Register a callback to run before all Gate checks.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function before(callable $callback)
    {}

    /**
     * Register a callback to run after all Gate checks.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function after(callable $callback)
    {}

    /**
     * Determine if all of the given abilities should be granted for the current user.
     *
     * @param  iterable|\UnitEnum|string  $ability
     * @param  mixed  $arguments
     * @return bool
     */
    public function allows($ability, $arguments = [])
    {}

    /**
     * Determine if any of the given abilities should be denied for the current user.
     *
     * @param  iterable|\UnitEnum|string  $ability
     * @param  mixed  $arguments
     * @return bool
     */
    public function denies($ability, $arguments = [])
    {}

    /**
     * Determine if all of the given abilities should be granted for the current user.
     *
     * @param  iterable|\UnitEnum|string  $abilities
     * @param  mixed  $arguments
     * @return bool
     */
    public function check($abilities, $arguments = [])
    {}

    /**
     * Determine if any one of the given abilities should be granted for the current user.
     *
     * @param  iterable|\UnitEnum|string  $abilities
     * @param  mixed  $arguments
     * @return bool
     */
    public function any($abilities, $arguments = [])
    {}

    /**
     * Determine if all of the given abilities should be denied for the current user.
     *
     * @param  iterable|\UnitEnum|string  $abilities
     * @param  mixed  $arguments
     * @return bool
     */
    public function none($abilities, $arguments = [])
    {}

    /**
     * Determine if the given ability should be granted for the current user.
     *
     * @param  \UnitEnum|string  $ability
     * @param  mixed  $arguments
     * @return \Illuminate\Auth\Access\Response
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    public function authorize($ability, $arguments = [])
    {}

    /**
     * Inspect the user for the given ability.
     *
     * @param  \UnitEnum|string  $ability
     * @param  mixed  $arguments
     * @return \Illuminate\Auth\Access\Response
     */
    public function inspect($ability, $arguments = [])
    {}

    /**
     * Get the raw result from the authorization callback.
     *
     * @param  string  $ability
     * @param  mixed  $arguments
     * @return mixed
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    public function raw($ability, $arguments = [])
    {}

    /**
     * Determine whether the callback/method can be called with the given user.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable|null  $user
     * @param  \Closure|string|array  $class
     * @param  string|null  $method
     * @return bool
     */
    protected function canBeCalledWithUser($user, $class, $method = null)
    {}

    /**
     * Determine if the given class method allows guests.
     *
     * @param  string  $class
     * @param  string  $method
     * @return bool
     */
    protected function methodAllowsGuests($class, $method)
    {}

    /**
     * Determine if the callback allows guests.
     *
     * @param  callable  $callback
     * @return bool
     *
     * @throws \ReflectionException
     */
    protected function callbackAllowsGuests($callback)
    {}

    /**
     * Determine if the given parameter allows guests.
     *
     * @param  \ReflectionParameter  $parameter
     * @return bool
     */
    protected function parameterAllowsGuests($parameter)
    {}

    /**
     * Resolve and call the appropriate authorization callback.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable|null  $user
     * @param  string  $ability
     * @param  array  $arguments
     * @return bool
     */
    protected function callAuthCallback($user, $ability, array $arguments)
    {}

    /**
     * Call all of the before callbacks and return if a result is given.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable|null  $user
     * @param  string  $ability
     * @param  array  $arguments
     * @return bool|null
     */
    protected function callBeforeCallbacks($user, $ability, array $arguments)
    {}

    /**
     * Call all of the after callbacks with check result.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable  $user
     * @param  string  $ability
     * @param  array  $arguments
     * @param  bool|null  $result
     * @return bool|null
     */
    protected function callAfterCallbacks($user, $ability, array $arguments, $result)
    {}

    /**
     * Dispatch a gate evaluation event.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable|null  $user
     * @param  string  $ability
     * @param  array  $arguments
     * @param  bool|null  $result
     * @return void
     */
    protected function dispatchGateEvaluatedEvent($user, $ability, array $arguments, $result)
    {}

    /**
     * Resolve the callable for the given ability and arguments.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable|null  $user
     * @param  string  $ability
     * @param  array  $arguments
     * @return callable
     */
    protected function resolveAuthCallback($user, $ability, array $arguments)
    {}

    /**
     * Get a policy instance for a given class.
     *
     * @param  object|string  $class
     * @return mixed
     */
    public function getPolicyFor($class)
    {}

    /**
     * Get the policy class from the class attribute.
     *
     * @param  class-string<*>  $class
     * @param  bool  $includeParents
     * @return class-string<*>|null
     */
    protected function getPolicyFromAttribute(string $class, bool $includeParents = false): ?string
    {}

    /**
     * Guess the policy name for the given class.
     *
     * @param  string  $class
     * @return array
     */
    protected function guessPolicyName($class)
    {}

    /**
     * Specify a callback to be used to guess policy names.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function guessPolicyNamesUsing(callable $callback)
    {}

    /**
     * Build a policy class instance of the given type.
     *
     * @param  object|string  $class
     * @return mixed
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function resolvePolicy($class)
    {}

    /**
     * Resolve the callback for a policy check.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable  $user
     * @param  string  $ability
     * @param  array  $arguments
     * @param  mixed  $policy
     * @return bool|callable
     */
    protected function resolvePolicyCallback($user, $ability, array $arguments, $policy)
    {}

    /**
     * Call the "before" method on the given policy, if applicable.
     *
     * @param  mixed  $policy
     * @param  \Illuminate\Contracts\Auth\Authenticatable  $user
     * @param  string  $ability
     * @param  array  $arguments
     * @return mixed
     */
    protected function callPolicyBefore($policy, $user, $ability, $arguments)
    {}

    /**
     * Call the appropriate method on the given policy.
     *
     * @param  mixed  $policy
     * @param  string  $method
     * @param  \Illuminate\Contracts\Auth\Authenticatable|null  $user
     * @param  array  $arguments
     * @return mixed
     */
    protected function callPolicyMethod($policy, $method, $user, array $arguments)
    {}

    /**
     * Format the policy ability into a method name.
     *
     * @param  string  $ability
     * @return string
     */
    protected function formatAbilityToMethod($ability)
    {}

    /**
     * Get a gate instance for the given user.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable|mixed  $user
     * @return static
     */
    public function forUser($user)
    {}

    /**
     * Resolve the user from the user resolver.
     *
     * @return mixed
     */
    protected function resolveUser()
    {}

    /**
     * Get all of the defined abilities.
     *
     * @return array
     */
    public function abilities()
    {}

    /**
     * Get all of the defined policies.
     *
     * @return array
     */
    public function policies()
    {}

    /**
     * Set the default denial response for gates and policies.
     *
     * @param  \Illuminate\Auth\Access\Response  $response
     * @return $this
     */
    public function defaultDenialResponse(Response $response)
    {}

    /**
     * Set the container instance used by the gate.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}
}
