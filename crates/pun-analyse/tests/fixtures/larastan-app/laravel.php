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
}
