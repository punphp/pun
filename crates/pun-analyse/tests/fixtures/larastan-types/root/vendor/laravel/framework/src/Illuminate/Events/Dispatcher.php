<?php

namespace Illuminate\Events;

use Closure;
use Exception;
use Illuminate\Bus\DebounceLock;
use Illuminate\Bus\UniqueLock;
use Illuminate\Container\Container;
use Illuminate\Contracts\Broadcasting\Factory as BroadcastFactory;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Contracts\Container\Container as ContainerContract;
use Illuminate\Contracts\Events\Dispatcher as DispatcherContract;
use Illuminate\Contracts\Events\ShouldDispatchAfterCommit;
use Illuminate\Contracts\Events\ShouldHandleEventsAfterCommit;
use Illuminate\Contracts\Queue\ShouldBeEncrypted;
use Illuminate\Contracts\Queue\ShouldBeUnique;
use Illuminate\Contracts\Queue\ShouldBeUniqueUntilProcessing;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Contracts\Queue\ShouldQueueAfterCommit;
use Illuminate\Queue\Attributes\Backoff;
use Illuminate\Queue\Attributes\Connection;
use Illuminate\Queue\Attributes\CountCrashesAsExceptions;
use Illuminate\Queue\Attributes\DebounceFor;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Attributes\DeleteWhenMissingModels;
use Illuminate\Queue\Attributes\FailOnTimeout;
use Illuminate\Queue\Attributes\MaxExceptions;
use Illuminate\Queue\Attributes\Queue as QueueAttribute;
use Illuminate\Queue\Attributes\Timeout;
use Illuminate\Queue\Attributes\Tries;
use Illuminate\Queue\Attributes\UniqueFor;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Queue\Concerns\ResolvesQueueRoutes;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\ReadsClassAttributes;
use Illuminate\Support\Traits\ReflectsClosures;
use LogicException;
use ReflectionClass;
use Throwable;

use function Illuminate\Support\enum_value;

class Dispatcher implements DispatcherContract
{
    use Macroable, ReadsClassAttributes, ReflectsClosures, ResolvesQueueRoutes;

    /**
     * The IoC container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The registered event listeners.
     *
     * @var array<string, callable|array|class-string|null>
     */
    protected $listeners = [];

    /**
     * The wildcard listeners.
     *
     * @var array<string, \Closure|string>
     */
    protected $wildcards = [];

    /**
     * The cached wildcard listeners.
     *
     * @var array<string, \Closure|string>
     */
    protected $wildcardsCache = [];

    /**
     * The queue resolver instance.
     *
     * @var callable(): \Illuminate\Contracts\Queue\Queue
     */
    protected $queueResolver;

    /**
     * The database transaction manager resolver instance.
     *
     * @var callable
     */
    protected $transactionManagerResolver;

    /**
     * The currently deferred events.
     *
     * @var array
     */
    protected $deferredEvents = [];

    /**
     * Indicates if events should be deferred.
     *
     * @var bool
     */
    protected $deferringEvents = false;

    /**
     * The specific events to defer (null means defer all events).
     *
     * @var string[]|null
     */
    protected $eventsToDefer = null;

    /**
     * Create a new event dispatcher instance.
     */
    public function __construct(?ContainerContract $container = null)
    {}

    /**
     * Register an event listener with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string|string  $events
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string|null  $listener
     * @return void
     */
    public function listen($events, $listener = null)
    {}

    /**
     * Setup a wildcard listener callback.
     *
     * @param  string  $event
     * @param  \Closure|string  $listener
     * @return void
     */
    protected function setupWildcardListen($event, $listener)
    {}

    /**
     * Determine if a given event has listeners.
     *
     * @param  string  $eventName
     * @return bool
     */
    public function hasListeners($eventName)
    {}

    /**
     * Determine if the given event has any wildcard listeners.
     *
     * @param  string  $eventName
     * @return bool
     */
    public function hasWildcardListeners($eventName)
    {}

    /**
     * Register an event and payload to be fired later.
     *
     * @param  string  $event
     * @param  object|array  $payload
     * @return void
     */
    public function push($event, $payload = [])
    {}

    /**
     * Flush a set of pushed events.
     *
     * @param  string  $event
     * @return void
     */
    public function flush($event)
    {}

    /**
     * Register an event subscriber with the dispatcher.
     *
     * @param  object|string  $subscriber
     * @return void
     */
    public function subscribe($subscriber)
    {}

    /**
     * Resolve the subscriber instance.
     *
     * @param  object|class-string  $subscriber
     * @return ($subscriber is object ? object : mixed)
     */
    protected function resolveSubscriber($subscriber)
    {}

    /**
     * Fire an event until the first non-null response is returned.
     *
     * @param  string|object  $event
     * @param  mixed  $payload
     * @return array|null
     */
    public function until($event, $payload = [])
    {}

    /**
     * Fire an event and call the listeners.
     *
     * @param  string|object  $event
     * @param  mixed  $payload
     * @param  bool  $halt
     * @return array|null
     */
    public function dispatch($event, $payload = [], $halt = false)
    {}

    /**
     * Broadcast an event and call its listeners.
     *
     * @param  string|object  $event
     * @param  mixed  $payload
     * @param  bool  $halt
     * @return array|null
     */
    protected function invokeListeners($event, $payload, $halt = false)
    {}

    /**
     * Parse the given event and payload and prepare them for dispatching.
     *
     * @param  mixed  $event
     * @param  mixed  $payload
     * @return array{string, array}
     */
    protected function parseEventAndPayload($event, $payload)
    {}

    /**
     * Determine if the payload has a broadcastable event.
     *
     * @return bool
     */
    protected function shouldBroadcast(array $payload)
    {}

    /**
     * Check if the event should be broadcasted by the condition.
     *
     * @param  mixed  $event
     * @return bool
     */
    protected function broadcastWhen($event)
    {}

    /**
     * Broadcast the given event class.
     *
     * @param  \Illuminate\Contracts\Broadcasting\ShouldBroadcast  $event
     * @return void
     */
    protected function broadcastEvent($event)
    {}

    /**
     * Get all of the listeners for a given event name.
     *
     * @param  string  $eventName
     * @return array
     */
    public function getListeners($eventName)
    {}

    /**
     * Get the wildcard listeners for the event.
     *
     * @param  string  $eventName
     * @return array
     */
    protected function getWildcardListeners($eventName)
    {}

    /**
     * Add the listeners for the event's interfaces to the given array.
     *
     * @param  string  $eventName
     * @return array
     */
    protected function addInterfaceListeners($eventName, array $listeners = [])
    {}

    /**
     * Prepare the listeners for a given event.
     *
     * @return \Closure[]
     */
    protected function prepareListeners(string $eventName)
    {}

    /**
     * Register an event listener with the dispatcher.
     *
     * @param  \Closure|string|array{class-string, string}  $listener
     * @param  bool  $wildcard
     * @return \Closure
     */
    public function makeListener($listener, $wildcard = false)
    {}

    /**
     * Create a class based listener using the IoC container.
     *
     * @param  string  $listener
     * @param  bool  $wildcard
     * @return \Closure
     */
    public function createClassListener($listener, $wildcard = false)
    {}

    /**
     * Create the class based event callable.
     *
     * @param  array{class-string, string}|string  $listener
     * @return callable
     */
    protected function createClassCallable($listener)
    {}

    /**
     * Parse the class listener into class and method.
     *
     * @param  string  $listener
     * @return array{class-string, string}
     */
    protected function parseClassCallable($listener)
    {}

    /**
     * Determine if the event handler class should be queued.
     *
     * @param  class-string  $class
     * @return bool
     *
     * @phpstan-assert-if-true class-string<\Illuminate\Contracts\Queue\ShouldQueue> $class
     */
    protected function handlerShouldBeQueued($class)
    {}

    /**
     * Create a callable for putting an event handler on the queue.
     *
     * @param  class-string  $class
     * @param  string  $method
     * @return \Closure(): void
     */
    protected function createQueuedHandlerCallable($class, $method)
    {}

    /**
     * Determine if the given event handler should be dispatched after all database transactions have committed.
     *
     * @param  mixed  $listener
     * @return bool
     */
    protected function handlerShouldBeDispatchedAfterDatabaseTransactions($listener)
    {}

    /**
     * Create a callable for dispatching a listener after database transactions.
     *
     * @param  mixed  $listener
     * @param  string  $method
     * @return \Closure
     */
    protected function createCallbackForListenerRunningAfterCommits($listener, $method)
    {}

    /**
     * Determine if the event handler wants to be queued.
     *
     * @param  class-string  $class
     * @param  array  $arguments
     * @return bool
     */
    protected function handlerWantsToBeQueued($class, $arguments)
    {}

    /**
     * Queue the handler class.
     *
     * @param  string  $class
     * @param  string  $method
     * @param  array  $arguments
     * @return void
     *
     * @throws \LogicException
     */
    protected function queueHandler($class, $method, $arguments)
    {}

    /**
     * Create the listener and job for a queued listener.
     *
     * @template TListener
     *
     * @param  class-string<TListener>  $class
     * @param  string  $method
     * @param  array  $arguments
     * @return array{TListener, mixed}
     *
     * @throws \ReflectionException
     */
    protected function createListenerAndJob($class, $method, $arguments)
    {}

    /**
     * Propagate listener options to the job.
     *
     * @param  mixed  $listener
     * @param  \Illuminate\Events\CallQueuedListener  $job
     * @return \Illuminate\Events\CallQueuedListener
     */
    protected function propagateListenerOptions($listener, $job)
    {}

    /**
     * Remove a set of listeners from the dispatcher.
     *
     * @param  string  $event
     * @return void
     */
    public function forget($event)
    {}

    /**
     * Forget all of the pushed listeners.
     *
     * @return void
     */
    public function forgetPushed()
    {}

    /**
     * Get the queue implementation from the resolver.
     *
     * @return \Illuminate\Contracts\Queue\Queue
     */
    protected function resolveQueue()
    {}

    /**
     * Set the queue resolver implementation.
     *
     * @param  callable(): \Illuminate\Contracts\Queue\Queue  $resolver
     * @return $this
     */
    public function setQueueResolver(callable $resolver)
    {}

    /**
     * Get the database transaction manager implementation from the resolver.
     *
     * @return \Illuminate\Database\DatabaseTransactionsManager|null
     */
    protected function resolveTransactionManager()
    {}

    /**
     * Set the database transaction manager resolver implementation.
     *
     * @param  (callable(): (\Illuminate\Database\DatabaseTransactionsManager|null))  $resolver
     * @return $this
     */
    public function setTransactionManagerResolver(callable $resolver)
    {}

    /**
     * Execute the given callback while deferring events, then dispatch all deferred events.
     *
     * @template TResult
     *
     * @param  callable(): TResult  $callback
     * @param  string[]|null  $events
     * @return TResult
     */
    public function defer(callable $callback, ?array $events = null)
    {}

    /**
     * Determine if the given event should be deferred.
     *
     * @return bool
     */
    protected function shouldDeferEvent(string $event)
    {}

    /**
     * Gets the raw, unprepared listeners.
     *
     * @return array<string, callable|array|class-string|null>
     */
    public function getRawListeners()
    {}
}
