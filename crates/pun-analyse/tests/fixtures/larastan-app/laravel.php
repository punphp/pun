<?php

// Laravel 13.23's declarations the probe reaches, with their signatures and PHPDoc, without their bodies.

namespace Carbon {
    interface CarbonInterface {}

    class Carbon implements CarbonInterface {}

    class CarbonImmutable implements CarbonInterface {}
}

namespace Psr\Container {
    interface ContainerInterface {}
}

namespace Illuminate\Contracts\Container {
    interface Container extends \Psr\Container\ContainerInterface {}
}

namespace Illuminate\Contracts\Foundation {
    interface Application extends \Illuminate\Contracts\Container\Container
    {
        /**
         * Get or check the current application environment.
         *
         * @param  string|array  ...$environments
         * @return string|bool
         */
        public function environment(...$environments);
    }
}

namespace Illuminate\Contracts\Auth {
    interface Authenticatable {}

    interface Factory
    {
        /**
         * Get a guard instance by name.
         *
         * @param  \UnitEnum|string|null  $name
         * @return \Illuminate\Contracts\Auth\Guard|\Illuminate\Contracts\Auth\StatefulGuard
         */
        public function guard($name = null);
    }

    interface Guard
    {
        /**
         * Get the currently authenticated user.
         *
         * @return \Illuminate\Contracts\Auth\Authenticatable|null
         */
        public function user();
    }

    interface StatefulGuard extends Guard {}
}

namespace Illuminate\Auth {
    /**
     * @mixin \Illuminate\Contracts\Auth\Guard
     * @mixin \Illuminate\Contracts\Auth\StatefulGuard
     */
    class AuthManager implements \Illuminate\Contracts\Auth\Factory
    {
        /**
         * Attempt to get the guard from the local cache.
         *
         * @param  \UnitEnum|string|null  $name
         * @return \Illuminate\Contracts\Auth\Guard|\Illuminate\Contracts\Auth\StatefulGuard
         */
        public function guard($name = null) {}
    }

    class SessionGuard implements \Illuminate\Contracts\Auth\StatefulGuard
    {
        /**
         * Get the currently authenticated user.
         *
         * @return \Illuminate\Contracts\Auth\Authenticatable|null
         */
        public function user() {}
    }
}

namespace Illuminate\Foundation\Auth {
    class User implements \Illuminate\Contracts\Auth\Authenticatable {}
}

namespace Illuminate\Contracts\Filesystem {
    interface Filesystem {}
}

namespace Illuminate\Filesystem {
    class FilesystemAdapter implements \Illuminate\Contracts\Filesystem\Filesystem {}
}

namespace Illuminate\Contracts\Support {
    /**
     * @template TKey of array-key
     * @template TValue
     */
    interface Arrayable
    {
        /**
         * Get the instance as an array.
         *
         * @return array<TKey, TValue>
         */
        public function toArray();
    }
}

namespace Illuminate\Support\Traits {
    trait Conditionable
    {
        /**
         * Apply the callback if the given "value" is (or resolves to) truthy.
         *
         * @template TWhenParameter
         * @template TWhenReturnType
         *
         * @param  (\Closure($this): TWhenParameter)|TWhenParameter|null  $value
         * @param  (callable($this, TWhenParameter): TWhenReturnType)|null  $callback
         * @param  (callable($this, TWhenParameter): TWhenReturnType)|null  $default
         * @return $this|TWhenReturnType
         */
        public function when($value = null, ?callable $callback = null, ?callable $default = null) {}
    }

    /**
     * @template TKey of array-key
     *
     * @template-covariant TValue
     */
    trait EnumeratesValues
    {
        use Conditionable;

        /**
         * Create a collection of all elements that do not pass a given truth test.
         *
         * @param  (callable(TValue, TKey): bool)|bool|TValue  $callback
         * @return static
         */
        public function reject($callback = true) {}

        /**
         * Filter items by the given key value pair.
         *
         * @param  callable|string  $key
         * @param  mixed  $operator
         * @param  mixed  $value
         * @return static
         */
        public function where($key, $operator = null, $value = null) {}
    }
}

namespace Illuminate\Support {
    class Carbon extends \Carbon\Carbon {}

    class Stringable {}

    /**
     * @template TTarget
     */
    class HigherOrderTapProxy
    {
        /**
         * The target being tapped.
         *
         * @var TTarget
         */
        public $target;

        /**
         * Create a new tap proxy instance.
         *
         * @param  TTarget  $target
         */
        public function __construct($target) {}

        /**
         * Dynamically pass method calls to the target.
         *
         * @param  string  $method
         * @param  array  $parameters
         * @return TTarget
         */
        public function __call($method, $parameters) {}
    }

    /**
     * @template TKey of array-key
     *
     * @template-covariant TValue
     *
     * @extends \Illuminate\Contracts\Support\Arrayable<TKey, TValue>
     * @extends \IteratorAggregate<TKey, TValue>
     */
    interface Enumerable extends \Illuminate\Contracts\Support\Arrayable, \Countable, \IteratorAggregate, \JsonSerializable
    {
        /**
         * Run a filter over each of the items.
         *
         * @param  (callable(TValue): bool)|null  $callback
         * @return static
         */
        public function filter(?callable $callback = null);

        /**
         * Get the values of a given key.
         *
         * @param  string|array<array-key, string>  $value
         * @param  string|null  $key
         * @return static<array-key, mixed>
         */
        public function pluck($value, $key = null);
    }

    /**
     * @template TKey of array-key
     *
     * @template-covariant TValue
     *
     * @implements \ArrayAccess<TKey, TValue>
     * @implements \Illuminate\Support\Enumerable<TKey, TValue>
     */
    class Collection implements \ArrayAccess, Enumerable
    {
        /**
         * @use \Illuminate\Support\Traits\EnumeratesValues<TKey, TValue>
         */
        use Traits\EnumeratesValues;

        /**
         * Create a new collection.
         *
         * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>|null  $items
         */
        public function __construct($items = []) {}

        /**
         * Run a filter over each of the items.
         *
         * @param  (callable(TValue, TKey): bool)|null  $callback
         * @return static
         */
        public function filter(?callable $callback = null) {}

        /**
         * Get the values of a given key.
         *
         * @param  \Closure|string|int|array<array-key, string>|null  $value
         * @param  \Closure|string|null  $key
         * @return static<array-key, mixed>
         */
        public function pluck($value, $key = null) {}

        public function toArray() {}

        public function count(): int {}

        public function getIterator(): \Traversable {}

        public function jsonSerialize(): mixed {}

        public function offsetExists($key): bool {}

        public function offsetGet($key): mixed {}

        public function offsetSet($key, $value): void {}

        public function offsetUnset($key): void {}
    }
}

namespace Illuminate\Container {
    class Container implements \Illuminate\Contracts\Container\Container
    {
        /**
         * Resolve the given type from the container.
         *
         * @template TClass of object
         *
         * @param  string|class-string<TClass>  $abstract
         * @return ($abstract is class-string<TClass> ? TClass : mixed)
         */
        public function make($abstract, array $parameters = []) {}
    }
}

namespace Illuminate\Contracts\Validation {
    interface Factory {}

    interface Validator {}
}

namespace Illuminate\Validation {
    class Validator implements \Illuminate\Contracts\Validation\Validator {}
}

namespace Illuminate\Routing {
    class Route {}
}

namespace Illuminate\Http\Concerns {
    trait InteractsWithInput
    {
        /**
         * Retrieve a query string item from the request.
         *
         * @param  string|null  $key
         * @param  string|array|null  $default
         * @return string|array|null
         */
        public function query($key = null, $default = null) {}

        /**
         * Retrieve a request payload item from the request.
         *
         * @param  string|null  $key
         * @param  string|array|null  $default
         * @return string|array|null
         */
        public function post($key = null, $default = null) {}

        /**
         * Retrieve a server variable from the request.
         *
         * @param  string|null  $key
         * @param  string|array|null  $default
         * @return string|array|null
         */
        public function server($key = null, $default = null) {}

        /**
         * Retrieve a file from the request.
         *
         * @param  string|null  $key
         * @param  mixed  $default
         * @return ($key is null ? array<string, \Illuminate\Http\UploadedFile|\Illuminate\Http\UploadedFile[]> : \Illuminate\Http\UploadedFile|\Illuminate\Http\UploadedFile[]|null)
         */
        public function file($key = null, $default = null) {}
    }
}

namespace Illuminate\Http {
    class UploadedFile {}

    class Request
    {
        use Concerns\InteractsWithInput;

        /**
         * Get the route handling the request.
         *
         * @param  string|null  $param
         * @param  mixed  $default
         * @return ($param is null ? \Illuminate\Routing\Route|null : object|string|null)
         */
        public function route($param = null, $default = null) {}
    }
}

namespace Symfony\Component\Console\Command {
    class Command
    {
        protected function configure() {}
    }
}

namespace Illuminate\Console {
    class Command extends \Symfony\Component\Console\Command\Command
    {
        /**
         * The name and signature of the console command.
         *
         * @var string
         */
        protected $signature;

        /**
         * The console command name.
         *
         * @var string
         */
        protected $name;

        /**
         * Determine if the given argument is present.
         *
         * @param  string|int  $name
         * @return bool
         */
        public function hasArgument($name) {}

        /**
         * Get the value of a command argument.
         *
         * @param  string|null  $key
         * @return ($key is null ? array<array|string|float|int|bool|null> : array|string|float|int|bool|null)
         */
        public function argument($key = null) {}

        /**
         * Get all of the arguments passed to the command.
         *
         * @return array<array|string|float|int|bool|null>
         */
        public function arguments() {}

        /**
         * Determine whether the option is defined in the command signature.
         *
         * @param  string  $name
         * @return bool
         */
        public function hasOption($name) {}

        /**
         * Get the value of a command option.
         *
         * @param  string|null  $key
         * @return ($key is null ? array<array|string|float|int|bool|null> : array|string|float|int|bool|null)
         */
        public function option($key = null) {}

        /**
         * Get all of the options passed to the command.
         *
         * @return array<array|string|float|int|bool|null>
         */
        public function options() {}
    }
}

namespace Illuminate\Support\Facades {
    abstract class Facade
    {
        public static function __callStatic($method, $args) {}
    }

    /**
     * @method static \Illuminate\Contracts\Auth\Authenticatable|null user()
     */
    class Auth extends Facade {}

    /**
     * @method static mixed use(mixed $handler)
     */
    class Date extends Facade {}

    /**
     * @method static \Illuminate\Contracts\Filesystem\Filesystem disk(\UnitEnum|string|null $name = null)
     */
    class Storage extends Facade {}
}

namespace {
    use Illuminate\Contracts\Auth\Factory as AuthFactory;
    use Illuminate\Contracts\Auth\Guard;
    use Carbon\CarbonInterface;

    /**
     * Get the available auth instance.
     *
     * @param  string|null  $guard
     * @return ($guard is null ? \Illuminate\Contracts\Auth\Factory : \Illuminate\Contracts\Auth\Guard)
     */
    function auth($guard = null): AuthFactory|Guard {}

    /**
     * Create a new Carbon instance for the current time.
     *
     * @param  \DateTimeZone|\UnitEnum|string|null  $tz
     */
    function now($tz = null): CarbonInterface {}

    /**
     * Create a new Carbon instance for the current date.
     *
     * @param  \DateTimeZone|\UnitEnum|string|null  $tz
     * @return \Illuminate\Support\Carbon
     */
    function today($tz = null): CarbonInterface {}

    /**
     * Translate the given message.
     *
     * @param  string|null  $key
     * @param  array  $replace
     * @param  string|null  $locale
     * @return ($key is null ? null : array|string)
     */
    function __($key = null, $replace = [], $locale = null): string|array|null {}

    /**
     * Create a collection from the given value.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>|null  $value
     * @return \Illuminate\Support\Collection<TKey, TValue>
     */
    function collect($value = []) {}

    /**
     * Get the available container instance.
     *
     * @template TClass of object
     *
     * @param  string|class-string<TClass>|null  $abstract
     * @return ($abstract is class-string<TClass> ? TClass : ($abstract is null ? \Illuminate\Foundation\Application : mixed))
     */
    function app($abstract = null, array $parameters = []) {}

    /**
     * Resolve a service from the container.
     *
     * @template TClass of object
     *
     * @param  string|class-string<TClass>  $name
     * @return ($name is class-string<TClass> ? TClass : mixed)
     */
    function resolve($name, array $parameters = []) {}

    /**
     * Return the default value of the given value.
     *
     * @template TValue
     * @template TArgs
     *
     * @param  TValue|\Closure(TArgs): TValue  $value
     * @param  TArgs  ...$args
     * @return TValue
     */
    function value($value, ...$args) {}

    /**
     * Create a new Validator instance.
     *
     * @return ($data is null ? \Illuminate\Contracts\Validation\Factory : \Illuminate\Contracts\Validation\Validator)
     */
    function validator(?array $data = null, array $rules = [], array $messages = [], array $attributes = []): \Illuminate\Contracts\Validation\Validator|\Illuminate\Contracts\Validation\Factory {}

    /**
     * Get a new stringable object from the given string.
     *
     * @param  string|null  $string
     * @return ($string is null ? object : \Illuminate\Support\Stringable)
     */
    function str($string = null) {}

    /**
     * Call the given Closure with the given value then return the value.
     *
     * @template TValue
     *
     * @param  TValue  $value
     * @param  (callable(TValue): mixed)|null  $callback
     * @return ($callback is null ? \Illuminate\Support\HigherOrderTapProxy<TValue> : TValue)
     */
    function tap($value, $callback = null) {}
}
