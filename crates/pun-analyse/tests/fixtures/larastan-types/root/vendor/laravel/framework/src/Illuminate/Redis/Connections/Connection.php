<?php

namespace Illuminate\Redis\Connections;

use Closure;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Redis\Events\CommandExecuted;
use Illuminate\Redis\Events\CommandFailed;
use Illuminate\Redis\Limiters\ConcurrencyLimiterBuilder;
use Illuminate\Redis\Limiters\DurationLimiterBuilder;
use Illuminate\Support\Traits\Macroable;
use Throwable;

/**
 * @mixin \Redis
 */
abstract class Connection
{
    use Macroable {
        __call as macroCall;
    }

    /**
     * The Redis client.
     *
     * @var \Redis
     */
    protected $client;

    /**
     * The Redis connection name.
     *
     * @var string|null
     */
    protected $name;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $events;

    /**
     * Subscribe to a set of given channels for messages.
     *
     * @param  array|string  $channels
     * @param  \Closure  $callback
     * @param  string  $method
     * @return void
     */
    abstract public function createSubscription($channels, Closure $callback, $method = 'subscribe');

    /**
     * Funnel a callback for a maximum number of simultaneous executions.
     *
     * @param  string  $name
     * @return \Illuminate\Redis\Limiters\ConcurrencyLimiterBuilder
     */
    public function funnel($name)
    {}

    /**
     * Throttle a callback for a maximum number of executions over a given duration.
     *
     * @param  string  $name
     * @return \Illuminate\Redis\Limiters\DurationLimiterBuilder
     */
    public function throttle($name)
    {}

    /**
     * Get the underlying Redis client.
     *
     * @return mixed
     */
    public function client()
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
     * Run a command against the Redis database.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \Throwable
     */
    public function command($method, array $parameters = [])
    {}

    /**
     * Parse the command's parameters for event dispatching.
     *
     * @param  array  $parameters
     * @return array
     */
    protected function parseParametersForEvent(array $parameters)
    {}

    /**
     * Fire the given event if possible.
     *
     * @param  mixed  $event
     * @return void
     *
     * @deprecated since Laravel 11.x
     */
    protected function event($event)
    {}

    /**
     * Register a Redis command listener with the connection.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function listen(Closure $callback)
    {}

    /**
     * Register a Redis command failure listener with the connection.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function listenForFailures(Closure $callback)
    {}

    /**
     * Determine if the connection is a cluster connection.
     *
     * @return bool
     */
    public function isCluster()
    {}

    /**
     * Get the connection name.
     *
     * @return string|null
     */
    public function getName()
    {}

    /**
     * Set the connection's name.
     *
     * @param  string  $name
     * @return $this
     */
    public function setName($name)
    {}

    /**
     * Get the event dispatcher used by the connection.
     *
     * @return \Illuminate\Contracts\Events\Dispatcher|null
     */
    public function getEventDispatcher()
    {}

    /**
     * Set the event dispatcher instance on the connection.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @return void
     */
    public function setEventDispatcher(Dispatcher $events)
    {}

    /**
     * Unset the event dispatcher instance on the connection.
     *
     * @return void
     */
    public function unsetEventDispatcher()
    {}

    /**
     * Determine if the given key contains a Redis Cluster hash tag.
     *
     * @param  string  $key
     * @return bool
     */
    public static function hasHashTag(string $key): bool
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
