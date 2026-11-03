<?php

namespace Illuminate\Support\Testing\Fakes;

use Closure;
use Illuminate\Container\Container;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Events\ShouldDispatchAfterCommit;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\ReflectsClosures;
use PHPUnit\Framework\Assert as PHPUnit;
use ReflectionFunction;

class EventFake implements Dispatcher, Fake
{
    use ForwardsCalls, MatchesProperties, ReflectsClosures;

    /**
     * The original event dispatcher.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    public $dispatcher;

    /**
     * The event types that should be intercepted instead of dispatched.
     *
     * @var array
     */
    protected $eventsToFake = [];

    /**
     * The event types that should be dispatched instead of intercepted.
     *
     * @var array
     */
    protected $eventsToDispatch = [];

    /**
     * All of the events that have been intercepted keyed by type.
     *
     * @var array
     */
    protected $events = [];

    /**
     * Create a new event fake instance.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $dispatcher
     * @param  array|string  $eventsToFake
     */
    public function __construct(Dispatcher $dispatcher, $eventsToFake = [])
    {}

    /**
     * Specify the events that should be dispatched instead of faked.
     *
     * @param  array|string  $eventsToDispatch
     * @return $this
     */
    public function except($eventsToDispatch)
    {}

    /**
     * Assert if an event has a listener attached to it.
     *
     * @param  string  $expectedEvent
     * @param  string|array  $expectedListener
     * @return void
     */
    public function assertListening($expectedEvent, $expectedListener)
    {}

    /**
     * Assert if an event was dispatched based on a truth-test callback.
     *
     * @param  string|\Closure  $event
     * @param  callable|array<string, mixed>|int|null  $callback
     * @return void
     */
    public function assertDispatched($event, $callback = null)
    {}

    /**
     * Assert if an event was dispatched exactly once.
     *
     * @param  string  $event
     * @return void
     */
    public function assertDispatchedOnce($event)
    {}

    /**
     * Assert if an event was dispatched a number of times.
     *
     * @param  string  $event
     * @param  int  $times
     * @return void
     */
    public function assertDispatchedTimes($event, $times = 1)
    {}

    /**
     * Determine if an event was dispatched based on a truth-test callback.
     *
     * @param  string|\Closure  $event
     * @param  callable|array<string, mixed>|null  $callback
     * @return void
     */
    public function assertNotDispatched($event, $callback = null)
    {}

    /**
     * Assert that no events were dispatched.
     *
     * @return void
     */
    public function assertNothingDispatched()
    {}

    /**
     * Get all of the events matching a truth-test callback.
     *
     * @param  string  $event
     * @param  callable|array<string, mixed>|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function dispatched($event, $callback = null)
    {}

    /**
     * Determine if the given event has been dispatched.
     *
     * @param  string  $event
     * @return bool
     */
    public function hasDispatched($event)
    {}

    /**
     * Register an event listener with the dispatcher.
     *
     * @param  \Closure|string|array  $events
     * @param  mixed  $listener
     * @return void
     */
    public function listen($events, $listener = null)
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
     * Register an event and payload to be dispatched later.
     *
     * @param  string  $event
     * @param  array  $payload
     * @return void
     */
    public function push($event, $payload = [])
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
     * Flush a set of pushed events.
     *
     * @param  string  $event
     * @return void
     */
    public function flush($event)
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
     * Determine if an event should be faked or actually dispatched.
     *
     * @param  string  $eventName
     * @param  mixed  $payload
     * @return bool
     */
    protected function shouldFakeEvent($eventName, $payload)
    {}

    /**
     * Push the event onto the fake events array immediately or after the next database transaction.
     *
     * @param  string|object  $event
     * @param  string  $name
     * @param  array  $arguments
     * @return void
     */
    protected function fakeEvent($event, $name, $arguments)
    {}

    /**
     * Determine whether an event should be dispatched or not.
     *
     * @param  string  $eventName
     * @param  mixed  $payload
     * @return bool
     */
    protected function shouldDispatchEvent($eventName, $payload)
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
     * Forget all of the queued listeners.
     *
     * @return void
     */
    public function forgetPushed()
    {}

    /**
     * Dispatch an event and call the listeners.
     *
     * @param  string|object  $event
     * @param  mixed  $payload
     * @return mixed
     */
    public function until($event, $payload = [])
    {}

    /**
     * Get the events that have been dispatched.
     *
     * @return array
     */
    public function dispatchedEvents()
    {}

    /**
     * Handle dynamic method calls to the dispatcher.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
