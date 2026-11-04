<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Events\NullDispatcher;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use InvalidArgumentException;
use ReflectionClass;

trait HasEvents
{
    /**
     * The event map for the model.
     *
     * Allows for object-based events for native Eloquent events.
     *
     * @var array<string, class-string>
     */
    protected $dispatchesEvents = [];

    /**
     * User exposed observable events.
     *
     * These are extra user-defined events observers may subscribe to.
     *
     * @var string[]
     */
    protected $observables = [];

    /**
     * Boot the has event trait for a model.
     *
     * @return void
     */
    public static function bootHasEvents()
    {}

    /**
     * Resolve the observe class names from the attributes.
     *
     * @return array
     */
    public static function resolveObserveAttributes()
    {}

    /**
     * Register observers with the model.
     *
     * @param  object|string[]|string  $classes
     * @return void
     *
     * @throws \RuntimeException
     */
    public static function observe($classes)
    {}

    /**
     * Register a single observer with the model.
     *
     * @param  object|string  $class
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function registerObserver($class)
    {}

    /**
     * Resolve the observer's class name from an object or string.
     *
     * @param  object|string  $class
     * @return class-string
     *
     * @throws \InvalidArgumentException
     */
    private function resolveObserverClassName($class)
    {}

    /**
     * Get the observable event names.
     *
     * @return string[]
     */
    public function getObservableEvents()
    {}

    /**
     * Set the observable event names.
     *
     * @param  string[]  $observables
     * @return $this
     */
    public function setObservableEvents(array $observables)
    {}

    /**
     * Add an observable event name.
     *
     * @param  string|string[]  $observables
     * @return void
     */
    public function addObservableEvents($observables)
    { func_get_args(); }

    /**
     * Remove an observable event name.
     *
     * @param  string|string[]  $observables
     * @return void
     */
    public function removeObservableEvents($observables)
    { func_get_args(); }

    /**
     * Register a model event with the dispatcher.
     *
     * @param  string  $event
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    protected static function registerModelEvent($event, $callback)
    {}

    /**
     * Fire the given event for the model.
     *
     * @param  string  $event
     * @param  bool  $halt
     * @return mixed
     */
    protected function fireModelEvent($event, $halt = true)
    {}

    /**
     * Fire a custom model event for the given event.
     *
     * @param  string  $event
     * @param  'until'|'dispatch'  $method
     * @return array|null|void
     */
    protected function fireCustomModelEvent($event, $method)
    {}

    /**
     * Filter the model event results.
     *
     * @param  mixed  $result
     * @return mixed
     */
    protected function filterModelEventResults($result)
    {}

    /**
     * Register a retrieved model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function retrieved($callback)
    {}

    /**
     * Register a saving model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function saving($callback)
    {}

    /**
     * Register a saved model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function saved($callback)
    {}

    /**
     * Register an updating model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function updating($callback)
    {}

    /**
     * Register an updated model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function updated($callback)
    {}

    /**
     * Register a creating model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function creating($callback)
    {}

    /**
     * Register a created model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function created($callback)
    {}

    /**
     * Register a replicating model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function replicating($callback)
    {}

    /**
     * Register a deleting model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function deleting($callback)
    {}

    /**
     * Register a deleted model event with the dispatcher.
     *
     * @param  \Illuminate\Events\QueuedClosure|callable|array|class-string  $callback
     * @return void
     */
    public static function deleted($callback)
    {}

    /**
     * Remove all the event listeners for the model.
     *
     * @return void
     */
    public static function flushEventListeners()
    {}

    /**
     * Get the event map for the model.
     *
     * @return array<string, class-string>
     */
    public function dispatchesEvents()
    {}

    /**
     * Get the event dispatcher instance.
     *
     * @return \Illuminate\Contracts\Events\Dispatcher|null
     */
    public static function getEventDispatcher()
    {}

    /**
     * Set the event dispatcher instance.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $dispatcher
     * @return void
     */
    public static function setEventDispatcher(Dispatcher $dispatcher)
    {}

    /**
     * Unset the event dispatcher for models.
     *
     * @return void
     */
    public static function unsetEventDispatcher()
    {}

    /**
     * Execute a callback without firing any model events for any model type.
     *
     * @template TReturn
     *
     * @param  callable(): TReturn  $callback
     * @return TReturn
     */
    public static function withoutEvents(callable $callback)
    {}
}
