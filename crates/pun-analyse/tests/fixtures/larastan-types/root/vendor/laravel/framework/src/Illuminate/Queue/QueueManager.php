<?php

namespace Illuminate\Queue;

use Closure;
use Illuminate\Contracts\Queue\Factory as FactoryContract;
use Illuminate\Contracts\Queue\Monitor as MonitorContract;
use Illuminate\Support\Queue\Concerns\ResolvesQueueRoutes;
use InvalidArgumentException;
use UnitEnum;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Contracts\Queue\Queue
 */
class QueueManager implements FactoryContract, MonitorContract
{
    use ResolvesQueueRoutes;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The array of resolved queue connections.
     *
     * @var array
     */
    protected $connections = [];

    /**
     * The array of resolved queue connectors.
     *
     * @var array
     */
    protected $connectors = [];

    /**
     * Create a new queue manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Register an event listener for the before job event.
     *
     * @param  mixed  $callback
     * @return void
     */
    public function before($callback)
    {}

    /**
     * Register an event listener for the after job event.
     *
     * @param  mixed  $callback
     * @return void
     */
    public function after($callback)
    {}

    /**
     * Register an event listener for the exception occurred job event.
     *
     * @param  mixed  $callback
     * @return void
     */
    public function exceptionOccurred($callback)
    {}

    /**
     * Register an event listener for the daemon queue loop.
     *
     * @param  mixed  $callback
     * @return void
     */
    public function looping($callback)
    {}

    /**
     * Register an event listener for the failed job event.
     *
     * @param  mixed  $callback
     * @return void
     */
    public function failing($callback)
    {}

    /**
     * Register an event listener for the daemon queue starting.
     *
     * @param  mixed  $callback
     * @return void
     */
    public function starting($callback)
    {}

    /**
     * Register an event listener for the daemon queue stopping.
     *
     * @param  mixed  $callback
     * @return void
     */
    public function stopping($callback)
    {}

    /**
     * Set the queue route for the given class.
     *
     * @param  array|class-string  $class
     * @param  \UnitEnum|string|null  $queue
     * @param  \UnitEnum|string|null  $connection
     * @return void
     */
    public function route(array|string $class, $queue = null, $connection = null)
    {}

    /**
     * Forward the given queue to another queue and/or connection.
     *
     * @param  array<string, \UnitEnum|string>|\UnitEnum|string  $queue
     * @param  \UnitEnum|string|null  $to
     * @param  \UnitEnum|string|null  $connection
     * @return void
     */
    public function forward(array|string|UnitEnum $queue, $to = null, $connection = null)
    {}

    /**
     * Determine if the driver is connected.
     *
     * @param  \UnitEnum|string|null  $name
     * @return bool
     */
    public function connected($name = null)
    {}

    /**
     * Resolve a queue connection instance.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Contracts\Queue\Queue
     */
    public function connection($name = null)
    {}

    /**
     * Resolve a queue connection.
     *
     * @param  string  $name
     * @return \Illuminate\Contracts\Queue\Queue
     *
     * @throws \InvalidArgumentException
     */
    protected function resolve($name)
    {}

    /**
     * Get the connector for a given driver.
     *
     * @param  string  $driver
     * @return \Illuminate\Queue\Connectors\ConnectorInterface
     *
     * @throws \InvalidArgumentException
     */
    protected function getConnector($driver)
    {}

    /**
     * Pause a queue by its connection and name.
     *
     * @param  \UnitEnum|string  $connection
     * @param  \UnitEnum|string  $queue
     * @return void
     */
    public function pause($connection, $queue)
    {}

    /**
     * Pause a queue by its connection and name for a given amount of time.
     *
     * @param  \UnitEnum|string  $connection
     * @param  \UnitEnum|string  $queue
     * @param  \DateTimeInterface|\DateInterval|int  $ttl
     * @return void
     */
    public function pauseFor($connection, $queue, $ttl)
    {}

    /**
     * Pause job processing for all queues on all connections.
     *
     * @return void
     */
    public function pauseAll()
    {}

    /**
     * Resume a paused queue by its connection and name.
     *
     * @param  \UnitEnum|string  $connection
     * @param  \UnitEnum|string  $queue
     * @return void
     */
    public function resume($connection, $queue)
    {}

    /**
     * Resume job processing for all queues on all connections.
     *
     * Queues paused individually are not affected.
     *
     * @return void
     */
    public function resumeAll()
    {}

    /**
     * Determine if a queue is paused.
     *
     * @param  \UnitEnum|string  $connection
     * @param  \UnitEnum|string  $queue
     * @return bool
     */
    public function isPaused($connection, $queue)
    {}

    /**
     * Determine which of the given queues are currently paused.
     *
     * @param  string  $connection
     * @param  array  $queues
     * @return array
     */
    public function getPausedQueues($connection, $queues)
    {}

    /**
     * Indicate that queue workers should not poll for restart or pause signals.
     *
     * This prevents the workers from hitting the application cache to determine if they need to pause or restart.
     *
     * @return void
     */
    public function withoutInterruptionPolling()
    {}

    /**
     * Add a queue connection resolver.
     *
     * @param  string  $driver
     * @param  \Closure  $resolver
     * @return void
     */
    public function extend($driver, Closure $resolver)
    {}

    /**
     * Add a queue connection resolver.
     *
     * @param  string  $driver
     * @param  \Closure  $resolver
     * @return void
     */
    public function addConnector($driver, Closure $resolver)
    {}

    /**
     * Get the queue connection configuration.
     *
     * @param  string  $name
     * @return array|null
     */
    protected function getConfig($name)
    {}

    /**
     * Get the name of the default queue connection.
     *
     * @return string
     */
    public function getDefaultDriver()
    {}

    /**
     * Set the name of the default queue connection.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultDriver($name)
    {}

    /**
     * Get the full name for the given connection.
     *
     * @param  string|null  $connection
     * @return string
     */
    public function getName($connection = null)
    {}

    /**
     * Get the application instance used by the manager.
     *
     * @return \Illuminate\Contracts\Foundation\Application
     */
    public function getApplication()
    {}

    /**
     * Set the application instance used by the manager.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return $this
     */
    public function setApplication($app)
    {}

    /**
     * Register a callback to be executed when creating job payloads.
     *
     * @param  callable|null  $callback
     * @return void
     */
    public function createPayloadUsing($callback)
    {}

    /**
     * Dynamically pass calls to the default connection.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
