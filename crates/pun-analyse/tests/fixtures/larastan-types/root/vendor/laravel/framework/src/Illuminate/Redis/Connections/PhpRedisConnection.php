<?php

namespace Illuminate\Redis\Connections;

use Closure;
use ErrorException;
use Illuminate\Contracts\Redis\Connection as ConnectionContract;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use RedisClusterException;
use RedisException;
use Throwable;

/**
 * @mixin \Redis
 */
class PhpRedisConnection extends Connection implements ConnectionContract
{
    use PacksPhpRedisValues;

    /**
     * The commands that may be safely retried after reconnecting.
     *
     * @var list<string>
     */
    protected const RETRYABLE_COMMANDS = [
        'bitcount',
        'bitpos',
        'dbsize',
        'dump',
        'exists',
        'geodist',
        'geohash',
        'geopos',
        'geosearch',
        'get',
        'getbit',
        'getrange',
        'hexists',
        'hget',
        'hgetall',
        'hkeys',
        'hlen',
        'hmget',
        'hmset',
        'hstrlen',
        'hvals',
        'keys',
        'lindex',
        'llen',
        'lpos',
        'lrange',
        'mget',
        'mset',
        'ping',
        'pttl',
        'randomkey',
        'scard',
        'sdiff',
        'sinter',
        'sismember',
        'smembers',
        'smismember',
        'srandmember',
        'strlen',
        'sunion',
        'time',
        'ttl',
        'type',
        'xinfo',
        'xlen',
        'xpending',
        'xrange',
        'xrevrange',
        'zcard',
        'zcount',
        'zlexcount',
        'zmscore',
        'zrange',
        'zrank',
        'zrevrank',
        'zscore',
    ];

    /**
     * The connection creation callback.
     *
     * @var callable
     */
    protected $connector;

    /**
     * The connection configuration array.
     *
     * @var array
     */
    protected $config;

    /**
     * Create a new PhpRedis connection.
     *
     * @param  \Redis  $client
     * @param  callable|null  $connector
     * @param  array  $config
     */
    public function __construct($client, ?callable $connector = null, array $config = [])
    {}

    /**
     * Returns the value of the given key.
     *
     * @param  string  $key
     * @return string|null
     */
    public function get($key)
    {}

    /**
     * Get the values of all the given keys.
     *
     * @param  array  $keys
     * @return array
     */
    public function mget(array $keys)
    {}

    /**
     * Set the string value in the argument as the value of the key.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  string|null  $expireResolution
     * @param  int|null  $expireTTL
     * @param  string|null  $flag
     * @return bool
     */
    public function set($key, $value, $expireResolution = null, $expireTTL = null, $flag = null)
    {}

    /**
     * Set the given key if it doesn't exist.
     *
     * @param  string  $key
     * @param  string  $value
     * @return int
     */
    public function setnx($key, $value)
    {}

    /**
     * Get the value of the given hash fields.
     *
     * @param  string  $key
     * @param  mixed  ...$dictionary
     * @return array
     */
    public function hmget($key, ...$dictionary)
    {}

    /**
     * Set the given hash fields to their respective values.
     *
     * @param  string  $key
     * @param  mixed  ...$dictionary
     * @return int
     */
    public function hmset($key, ...$dictionary)
    {}

    /**
     * Set the given hash field if it doesn't exist.
     *
     * @param  string  $hash
     * @param  string  $key
     * @param  string  $value
     * @return int
     */
    public function hsetnx($hash, $key, $value)
    {}

    /**
     * Removes the first count occurrences of the value element from the list.
     *
     * @param  string  $key
     * @param  int  $count
     * @param  mixed  $value
     * @return int|false
     */
    public function lrem($key, $count, $value)
    {}

    /**
     * Removes and returns the first element of the list stored at key.
     *
     * @param  mixed  ...$arguments
     * @return array|null
     */
    public function blpop(...$arguments)
    {}

    /**
     * Removes and returns the last element of the list stored at key.
     *
     * @param  mixed  ...$arguments
     * @return array|null
     */
    public function brpop(...$arguments)
    {}

    /**
     * Removes and returns a random element from the set value at key.
     *
     * @param  string  $key
     * @param  int|null  $count
     * @return mixed|false
     */
    public function spop($key, $count = 1)
    {}

    /**
     * Add one or more members to a sorted set or update its score if it already exists.
     *
     * @param  string  $key
     * @param  mixed  ...$dictionary
     * @return int
     */
    public function zadd($key, ...$dictionary)
    {}

    /**
     * Return elements with score between $min and $max.
     *
     * @param  string  $key
     * @param  mixed  $min
     * @param  mixed  $max
     * @param  array  $options
     * @return array
     */
    public function zrangebyscore($key, $min, $max, $options = [])
    {}

    /**
     * Return elements with score between $min and $max.
     *
     * @param  string  $key
     * @param  mixed  $min
     * @param  mixed  $max
     * @param  array  $options
     * @return array
     */
    public function zrevrangebyscore($key, $min, $max, $options = [])
    {}

    /**
     * Find the intersection between sets and store in a new set.
     *
     * @param  string  $output
     * @param  array  $keys
     * @param  array  $options
     * @return int
     */
    public function zinterstore($output, $keys, $options = [])
    {}

    /**
     * Find the union between sets and store in a new set.
     *
     * @param  string  $output
     * @param  array  $keys
     * @param  array  $options
     * @return int
     */
    public function zunionstore($output, $keys, $options = [])
    {}

    /**
     * Scans all keys based on options.
     *
     * @param  mixed  $cursor
     * @param  array  $options
     * @return mixed
     */
    public function scan($cursor, $options = [])
    {}

    /**
     * Scans the given set for all values based on options.
     *
     * @param  string  $key
     * @param  mixed  $cursor
     * @param  array  $options
     * @return mixed
     */
    public function zscan($key, $cursor, $options = [])
    {}

    /**
     * Scans the given hash for all values based on options.
     *
     * @param  string  $key
     * @param  mixed  $cursor
     * @param  array  $options
     * @return mixed
     */
    public function hscan($key, $cursor, $options = [])
    {}

    /**
     * Scans the given set for all values based on options.
     *
     * @param  string  $key
     * @param  mixed  $cursor
     * @param  array  $options
     * @return mixed
     */
    public function sscan($key, $cursor, $options = [])
    {}

    /**
     * Execute commands in a pipeline.
     *
     * @param  callable|null  $callback
     * @return \Redis|array
     */
    public function pipeline(?callable $callback = null)
    {}

    /**
     * Execute commands in a transaction.
     *
     * @param  callable|null  $callback
     * @return \Redis|array
     */
    public function transaction(?callable $callback = null)
    {}

    /**
     * Evaluate a LUA script serverside, from the SHA1 hash of the script instead of the script itself.
     *
     * @param  string  $script
     * @param  int  $numkeys
     * @param  mixed  ...$arguments
     * @return mixed
     */
    public function evalsha($script, $numkeys, ...$arguments)
    {}

    /**
     * Evaluate a script and return its result.
     *
     * @param  string  $script
     * @param  int  $numberOfKeys
     * @param  mixed  ...$arguments
     * @return mixed
     */
    public function eval($script, $numberOfKeys, ...$arguments)
    {}

    /**
     * Subscribe to a set of given channels for messages.
     *
     * @param  array|string  $channels
     * @param  \Closure  $callback
     * @return void
     */
    public function subscribe($channels, Closure $callback)
    {}

    /**
     * Subscribe to a set of given channels with wildcards.
     *
     * @param  array|string  $channels
     * @param  \Closure  $callback
     * @return void
     */
    public function psubscribe($channels, Closure $callback)
    {}

    /**
     * Subscribe to a set of given channels for messages.
     *
     * @param  array|string  $channels
     * @param  \Closure  $callback
     * @param  string  $method
     * @return void
     */
    public function createSubscription($channels, Closure $callback, $method = 'subscribe')
    {}

    /**
     * Flush the selected Redis database.
     *
     * @return mixed
     */
    public function flushdb()
    {}

    /**
     * Execute a raw command.
     *
     * @param  array  $parameters
     * @return mixed
     */
    public function executeRaw(array $parameters)
    {}

    /**
     * Run a command against the Redis database.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \RedisClusterException
     * @throws \RedisException
     */
    public function command($method, array $parameters = [])
    {}

    /**
     * Determine whether the command may be safely retried.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return bool
     */
    protected function isRetryable($method, array $parameters)
    {}

    /**
     * Run the given callback, retrying it once on a rebuilt client if the connection was lost.
     *
     * For operations that have not yet sent a command that could have taken effect, such as opening a pipeline or a transaction.
     *
     * @param  \Closure  $callback
     * @return mixed
     */
    protected function retryOnceOnLostConnection(Closure $callback)
    {}

    /**
     * Rebuild the client if the given exception was caused by a lost connection.
     *
     * @param  \Throwable  $e
     * @return void
     */
    protected function rebuildClientOnLostConnection(Throwable $e)
    {}

    /**
     * Determine if the given exception was caused by a lost connection to the Redis server.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    protected function causedByLostConnection(Throwable $e)
    {}

    /**
     * Replace the underlying client with a freshly connected one.
     *
     * @return void
     */
    protected function rebuildClient()
    {}

    /**
     * Disconnects from the Redis instance.
     *
     * @return void
     */
    public function disconnect()
    {}

    /**
     * Pass other method calls down to the underlying client.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
