<?php

namespace Illuminate\Broadcasting;

use Ably\AblyRest;
use Closure;
use GuzzleHttp\Client as GuzzleClient;
use Illuminate\Broadcasting\Broadcasters\AblyBroadcaster;
use Illuminate\Broadcasting\Broadcasters\LogBroadcaster;
use Illuminate\Broadcasting\Broadcasters\NullBroadcaster;
use Illuminate\Broadcasting\Broadcasters\PusherBroadcaster;
use Illuminate\Broadcasting\Broadcasters\RedisBroadcaster;
use Illuminate\Broadcasting\Mercure\CreatesMercureDrivers;
use Illuminate\Bus\UniqueLock;
use Illuminate\Contracts\Broadcasting\Factory as FactoryContract;
use Illuminate\Contracts\Broadcasting\ShouldBeUnique;
use Illuminate\Contracts\Broadcasting\ShouldBroadcastNow;
use Illuminate\Contracts\Broadcasting\ShouldRescue;
use Illuminate\Contracts\Bus\Dispatcher as BusDispatcherContract;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Contracts\Foundation\CachesRoutes;
use Illuminate\Queue\Attributes\Connection as ConnectionAttribute;
use Illuminate\Queue\Attributes\Queue as QueueAttribute;
use Illuminate\Queue\Attributes\ReadsQueueAttributes;
use Illuminate\Queue\Events\UniqueJobSkipped;
use Illuminate\Support\Queue\Concerns\ResolvesQueueRoutes;
use Illuminate\Support\RebindsCallbacksToSelf;
use InvalidArgumentException;
use Psr\Log\LoggerInterface;
use Pusher\Pusher;
use ReflectionException;
use RuntimeException;
use Throwable;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Contracts\Broadcasting\Broadcaster
 */
class BroadcastManager implements FactoryContract
{
    use CreatesMercureDrivers, ReadsQueueAttributes, RebindsCallbacksToSelf, ResolvesQueueRoutes;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $app;

    /**
     * The array of resolved broadcast drivers.
     *
     * @var array
     */
    protected $drivers = [];

    /**
     * The registered custom driver creators.
     *
     * @var array
     */
    protected $customCreators = [];

    /**
     * Create a new manager instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $app
     */
    public function __construct($app)
    {}

    /**
     * Register the routes for handling broadcast channel authentication and sockets.
     *
     * @param  array|null  $attributes
     * @return void
     */
    public function routes(?array $attributes = null)
    {}

    /**
     * Register the routes for handling broadcast user authentication.
     *
     * @param  array|null  $attributes
     * @return void
     */
    public function userRoutes(?array $attributes = null)
    {}

    /**
     * Register the routes for handling broadcast authentication and sockets.
     *
     * Alias of "routes" method.
     *
     * @param  array|null  $attributes
     * @return void
     */
    public function channelRoutes(?array $attributes = null)
    {}

    /**
     * Get the socket ID for the given request.
     *
     * @param  \Illuminate\Http\Request|null  $request
     * @return string|null
     */
    public function socket($request = null)
    {}

    /**
     * Begin sending an anonymous broadcast to the given channels.
     */
    public function on(Channel|string|array $channels): AnonymousEvent
    {}

    /**
     * Begin sending an anonymous broadcast to the given private channels.
     */
    public function private(string $channel): AnonymousEvent
    {}

    /**
     * Begin sending an anonymous broadcast to the given presence channels.
     */
    public function presence(string $channel): AnonymousEvent
    {}

    /**
     * Begin broadcasting an event.
     *
     * @param  mixed  $event
     * @return \Illuminate\Broadcasting\PendingBroadcast
     */
    public function event($event = null)
    {}

    /**
     * Queue the given event for broadcast.
     *
     * @param  mixed  $event
     * @return void
     */
    public function queue($event)
    {}

    /**
     * Determine if the broadcastable event must be unique and determine if we can acquire the necessary lock.
     *
     * @param  mixed  $event
     * @return bool
     */
    protected function mustBeUniqueAndCannotAcquireLock($event)
    {}

    /**
     * Get a broadcaster instance by name.
     *
     * @param  \UnitEnum|string|null  $name
     * @return mixed
     */
    public function connection($name = null)
    {}

    /**
     * Get a driver instance.
     *
     * @param  \UnitEnum|string|null  $name
     * @return mixed
     */
    public function driver($name = null)
    {}

    /**
     * Attempt to get the connection from the local cache.
     *
     * @param  string  $name
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function get($name)
    {}

    /**
     * Resolve the given broadcaster.
     *
     * @param  string  $name
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     *
     * @throws \InvalidArgumentException
     * @throws \RuntimeException
     */
    protected function resolve($name)
    {}

    /**
     * Call a custom driver creator.
     *
     * @param  array  $config
     * @return mixed
     */
    protected function callCustomCreator(array $config)
    {}

    /**
     * Create an instance of the driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function createReverbDriver(array $config)
    {}

    /**
     * Create an instance of the driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function createPusherDriver(array $config)
    {}

    /**
     * Get a Pusher instance for the given configuration.
     *
     * @param  array  $config
     * @return \Pusher\Pusher
     */
    public function pusher(array $config)
    {}

    /**
     * Create an instance of the driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function createAblyDriver(array $config)
    {}

    /**
     * Get an Ably instance for the given configuration.
     *
     * @param  array  $config
     * @return \Ably\AblyRest
     */
    public function ably(array $config)
    {}

    /**
     * Create an instance of the driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function createRedisDriver(array $config)
    {}

    /**
     * Create an instance of the driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function createLogDriver(array $config)
    {}

    /**
     * Create an instance of the driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function createNullDriver(array $config)
    {}

    /**
     * Get the connection configuration.
     *
     * @param  string  $name
     * @return array|null
     */
    protected function getConfig($name)
    {}

    /**
     * Get the default driver name.
     *
     * @return string
     */
    public function getDefaultDriver()
    {}

    /**
     * Set the default driver name.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultDriver($name)
    {}

    /**
     * Disconnect the given driver / connection and remove it from local cache.
     *
     * @param  \UnitEnum|string|null  $name
     * @return void
     */
    public function purge($name = null)
    {}

    /**
     * Register a custom driver creator Closure.
     *
     * @param  string  $driver
     * @param  \Closure  $callback
     *
     * @param-closure-this  $this  $callback
     *
     * @return $this
     */
    public function extend($driver, Closure $callback)
    {}

    /**
     * Execute the given callback using "rescue" if possible.
     *
     * @param  \Closure  $callback
     * @return mixed
     */
    protected function rescue(Closure $callback)
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
     * Forget all of the resolved driver instances.
     *
     * @return $this
     */
    public function forgetDrivers()
    {}

    /**
     * Dynamically call the default driver instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
