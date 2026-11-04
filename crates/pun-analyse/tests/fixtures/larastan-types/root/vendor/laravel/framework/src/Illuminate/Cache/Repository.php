<?php

namespace Illuminate\Cache;

use ArrayAccess;
use BadMethodCallException;
use Closure;
use DateTimeInterface;
use Illuminate\Cache\Events\CacheFlushed;
use Illuminate\Cache\Events\CacheFlushFailed;
use Illuminate\Cache\Events\CacheFlushing;
use Illuminate\Cache\Events\CacheHit;
use Illuminate\Cache\Events\CacheLocksFlushed;
use Illuminate\Cache\Events\CacheLocksFlushFailed;
use Illuminate\Cache\Events\CacheLocksFlushing;
use Illuminate\Cache\Events\CacheMissed;
use Illuminate\Cache\Events\ForgettingKey;
use Illuminate\Cache\Events\KeyForgetFailed;
use Illuminate\Cache\Events\KeyForgotten;
use Illuminate\Cache\Events\KeyWriteFailed;
use Illuminate\Cache\Events\KeyWritten;
use Illuminate\Cache\Events\RetrievingKey;
use Illuminate\Cache\Events\RetrievingManyKeys;
use Illuminate\Cache\Events\WritingKey;
use Illuminate\Cache\Events\WritingManyKeys;
use Illuminate\Cache\Limiters\ConcurrencyLimiterBuilder;
use Illuminate\Contracts\Cache\CanFlushLocks;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Contracts\Cache\Repository as CacheContract;
use Illuminate\Contracts\Cache\Store;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;

use function Illuminate\Support\defer;
use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Contracts\Cache\Store
 */
class Repository implements ArrayAccess, CacheContract
{
    use InteractsWithTime, Macroable {
        __call as macroCall;
    }

    /**
     * The cache key prefix used to track when a flexible cache value was last refreshed.
     *
     * @var string
     */
    const FLEXIBLE_CREATED_KEY_PREFIX = 'illuminate:cache:flexible:created:';

    /**
     * The cache store implementation.
     *
     * @var \Illuminate\Contracts\Cache\Store
     */
    protected $store;

    /**
     * The event dispatcher implementation.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $events;

    /**
     * The default number of seconds to store items.
     *
     * @var int|null
     */
    protected $default = 3600;

    /**
     * The cache store configuration options.
     *
     * @var array
     */
    protected $config = [];

    /**
     * The callback to invoke when an unserializable class is encountered.
     *
     * @var callable|null
     */
    protected static $unserializableClassHandler;

    /**
     * Create a new cache repository instance.
     */
    public function __construct(Store $store, array $config = [])
    {}

    /**
     * Determine if an item exists in the cache.
     *
     * @param  \UnitEnum|array|string  $key
     */
    public function has($key): bool
    {}

    /**
     * Determine if an item doesn't exist in the cache.
     *
     * @param  \UnitEnum|string  $key
     * @return bool
     */
    public function missing($key)
    {}

    /**
     * Retrieve an item from the cache by key.
     *
     * @param  \UnitEnum|array|string  $key
     * @param  mixed  $default
     */
    public function get($key, $default = null): mixed
    {}

    /**
     * Retrieve multiple items from the cache by key.
     *
     * Items not found in the cache will have a null value.
     *
     * @return array
     */
    public function many(array $keys)
    {}

    /**
     * {@inheritdoc}
     */
    public function getMultiple($keys, $default = null): iterable
    {}

    /**
     * Handle a result for the "many" method.
     *
     * @param  array  $keys
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function handleManyResult($keys, $key, $value)
    {}

    /**
     * Retrieve an item from the cache and delete it.
     *
     * @param  \UnitEnum|array|string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function pull($key, $default = null)
    {}

    /**
     * Retrieve a string item from the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  (\Closure():(string|null))|string|null  $default
     *
     * @throws \InvalidArgumentException
     */
    public function string($key, $default = null): string
    {}

    /**
     * Retrieve an integer item from the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  (\Closure():(int|null))|int|null  $default
     *
     * @throws \InvalidArgumentException
     */
    public function integer($key, $default = null): int
    {}

    /**
     * Retrieve a float item from the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  (\Closure():(float|null))|float|null  $default
     *
     * @throws \InvalidArgumentException
     */
    public function float($key, $default = null): float
    {}

    /**
     * Retrieve a boolean item from the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  (\Closure():(bool|null))|bool|null  $default
     *
     * @throws \InvalidArgumentException
     */
    public function boolean($key, $default = null): bool
    {}

    /**
     * Retrieve an array item from the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  (\Closure():(array<array-key, mixed>|null))|array<array-key, mixed>|null  $default
     * @return array<array-key, mixed>
     *
     * @throws \InvalidArgumentException
     */
    public function array($key, $default = null): array
    {}

    /**
     * Store an item in the cache.
     *
     * @param  \UnitEnum|array|string  $key
     * @param  mixed  $value
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     * @return bool
     */
    public function put($key, $value, $ttl = null)
    {}

    /**
     * Store an item in the cache.
     *
     * @param  \UnitEnum|array|string  $key
     * @param  mixed  $value
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     */
    public function set($key, $value, $ttl = null): bool
    {}

    /**
     * Store multiple items in the cache for a given number of seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     * @return bool
     */
    public function putMany(array $values, $ttl = null)
    {}

    /**
     * Store multiple items in the cache indefinitely.
     *
     * @return bool
     */
    protected function putManyForever(array $values)
    {}

    /**
     * {@inheritdoc}
     */
    public function setMultiple($values, $ttl = null): bool
    {}

    /**
     * Store an item in the cache if the key does not exist.
     *
     * @param  \UnitEnum|array|string  $key
     * @param  mixed  $value
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     * @return bool
     */
    public function add($key, $value, $ttl = null)
    {}

    /**
     * Increment the value of an item in the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return int|bool
     */
    public function increment($key, $value = 1)
    {}

    /**
     * Decrement the value of an item in the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return int|bool
     */
    public function decrement($key, $value = 1)
    {}

    /**
     * Store an item in the cache indefinitely.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return bool
     */
    public function forever($key, $value)
    {}

    /**
     * Get an item from the cache, or execute the given Closure and store the result.
     *
     * @template TCacheValue
     *
     * @param  \UnitEnum|string  $key
     * @param  \Closure|\DateTimeInterface|\DateInterval|int|null  $ttl
     * @param  \Closure(): TCacheValue  $callback
     * @return TCacheValue
     */
    public function remember($key, $ttl, Closure $callback)
    {}

    /**
     * Get an item from the cache, or execute the given Closure and store the result.
     *
     * @template TCacheValue
     *
     * @param  \UnitEnum|string  $key
     * @param  \Closure|\DateTimeInterface|\DateInterval|int|null  $ttl
     * @param  \Closure(): TCacheValue  $callback
     * @return array{TCacheValue, bool} The cached value and whether it was warm.
     */
    public function rememberWithWarmth($key, $ttl, Closure $callback): array
    {}

    /**
     * Get an item from the cache, or execute the given Closure and store the result forever.
     *
     * @template TCacheValue
     *
     * @param  \UnitEnum|string  $key
     * @param  \Closure(): TCacheValue  $callback
     * @return TCacheValue
     */
    public function sear($key, Closure $callback)
    {}

    /**
     * Get an item from the cache, or execute the given Closure and store the result forever.
     *
     * @template TCacheValue
     *
     * @param  \UnitEnum|string  $key
     * @param  \Closure(): TCacheValue  $callback
     * @return TCacheValue
     */
    public function rememberForever($key, Closure $callback)
    {}

    /**
     * Retrieve an item from the cache by key, refreshing it in the background if it is stale.
     *
     * @template TCacheValue
     *
     * @param  \UnitEnum|string  $key
     * @param  array{ 0: \DateTimeInterface|\DateInterval|int, 1: \DateTimeInterface|\DateInterval|int }  $ttl
     * @param  (callable(): TCacheValue)  $callback
     * @param  array{ seconds?: int, owner?: string }|null  $lock
     * @param  bool  $alwaysDefer
     * @return TCacheValue
     */
    public function flexible($key, $ttl, $callback, $lock = null, $alwaysDefer = false)
    {}

    /**
     * Set the expiration of a cached item.
     *
     * @param  \UnitEnum|string  $key
     * @param  \DateTimeInterface|\DateInterval|int  $ttl
     * @return bool
     */
    public function touch($key, $ttl)
    {}

    /**
     * Execute a callback while holding an atomic lock on a cache mutex to prevent overlapping calls.
     *
     * @template TReturn
     *
     * @param  \UnitEnum|string  $key
     * @param  callable(): TReturn  $callback
     * @param  int  $lockFor
     * @param  int  $waitFor
     * @param  string|null  $owner
     * @return TReturn
     *
     * @throws \Illuminate\Contracts\Cache\LockTimeoutException
     */
    public function withoutOverlapping($key, callable $callback, $lockFor = 0, $waitFor = 10, $owner = null)
    {}

    /**
     * Funnel a callback for a maximum number of simultaneous executions.
     *
     * @param  \UnitEnum|string  $name
     * @return \Illuminate\Cache\Limiters\ConcurrencyLimiterBuilder
     */
    public function funnel($name)
    {}

    /**
     * Remove an item from the cache.
     *
     * @param  \UnitEnum|array|string  $key
     * @return bool
     */
    public function forget($key)
    {}

    /**
     * Remove an item from the cache.
     *
     * @param  \UnitEnum|array|string  $key
     */
    public function delete($key): bool
    {}

    /**
     * {@inheritdoc}
     */
    public function deleteMultiple($keys): bool
    {}

    /**
     * {@inheritdoc}
     */
    public function clear(): bool
    {}

    /**
     * Flush all locks from the cache store.
     *
     * @throws \BadMethodCallException
     */
    public function flushLocks(): bool
    {}

    /**
     * Begin executing a new tags operation if the store supports it.
     *
     * @param  mixed  $names
     * @return \Illuminate\Cache\TaggedCache
     *
     * @throws \BadMethodCallException
     */
    public function tags($names)
    { func_get_args(); }

    /**
     * Format the key for a cache item.
     *
     * @param  string  $key
     * @return string
     */
    protected function itemKey($key)
    {}

    /**
     * Handle a cache value that contains an incomplete class.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function handleIncompleteClass(string $key, mixed $value): mixed
    {}

    /**
     * Calculate the number of seconds for the given TTL.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $ttl
     * @return int
     */
    protected function getSeconds($ttl)
    {}

    /**
     * Get the name of the cache store.
     *
     * @return string|null
     */
    public function getName()
    {}

    /**
     * Determine if the current store supports tags.
     *
     * @return bool
     */
    public function supportsTags()
    {}

    /**
     * Determine if the current store supports flushing locks.
     */
    public function supportsFlushingLocks(): bool
    {}

    /**
     * Get the default cache time.
     *
     * @return int|null
     */
    public function getDefaultCacheTime()
    {}

    /**
     * Set the default cache time in seconds.
     *
     * @param  int|null  $seconds
     * @return $this
     */
    public function setDefaultCacheTime($seconds)
    {}

    /**
     * Get the cache store implementation.
     *
     * @return \Illuminate\Contracts\Cache\Store
     */
    public function getStore()
    {}

    /**
     * Set the cache store implementation.
     *
     * @param  \Illuminate\Contracts\Cache\Store  $store
     * @return $this
     */
    public function setStore($store)
    {}

    /**
     * Fire an event for this cache instance.
     *
     * @param  object|string  $event
     * @return void
     */
    protected function event($event)
    {}

    /**
     * Get the event dispatcher instance.
     *
     * @return \Illuminate\Contracts\Events\Dispatcher|null
     */
    public function getEventDispatcher()
    {}

    /**
     * Set the event dispatcher instance.
     *
     * @return void
     */
    public function setEventDispatcher(Dispatcher $events)
    {}

    /**
     * Register a callback to be invoked when an unserializable class is encountered.
     *
     * @param  callable|null  $callback
     * @return void
     */
    public static function handleUnserializableClassUsing(?callable $callback): void
    {}

    /**
     * Determine if a cached value exists.
     *
     * @param  \UnitEnum|string  $offset
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Retrieve an item from the cache by key.
     *
     * @param  \UnitEnum|string  $offset
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Store an item in the cache for the default time.
     *
     * @param  \UnitEnum|string  $offset
     * @param  mixed  $value
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Remove an item from the cache.
     *
     * @param  \UnitEnum|string  $offset
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Handle dynamic calls into macros or pass missing methods to the store.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}

    /**
     * Clone cache repository instance.
     *
     * @return void
     */
    public function __clone()
    {}
}
