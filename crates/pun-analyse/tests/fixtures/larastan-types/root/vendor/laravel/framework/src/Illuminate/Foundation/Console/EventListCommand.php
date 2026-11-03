<?php

namespace Illuminate\Foundation\Console;

use Closure;
use Illuminate\Console\Command;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Support\Collection;
use ReflectionFunction;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'event:list')]
class EventListCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'event:list
                            {--event= : Filter the events by name}
                            {--json : Output the events and listeners as JSON}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = "List the application's events and listeners";

    /**
     * The events dispatcher resolver callback.
     *
     * @var \Closure|null
     */
    protected static $eventsResolver;

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Display events and their listeners in JSON.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @return void
     */
    protected function displayJson(Collection $events)
    {}

    /**
     * Display the events and their listeners for the CLI.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @return void
     */
    protected function displayForCli(Collection $events)
    {}

    /**
     * Get all of the events and listeners configured for the application.
     *
     * @return \Illuminate\Support\Collection
     */
    protected function getEvents()
    {}

    /**
     * Get the event / listeners from the dispatcher object.
     *
     * @return array
     */
    protected function getListenersOnDispatcher()
    {}

    /**
     * Add the event implemented interfaces to the output.
     *
     * @param  string  $event
     * @return string
     */
    protected function appendEventInterfaces($event)
    {}

    /**
     * Add the listener implemented interfaces to the output.
     *
     * @param  string  $listener
     * @return string
     */
    protected function appendListenerInterfaces($listener)
    {}

    /**
     * Get a displayable string representation of a Closure listener.
     *
     * @param  \Closure  $rawListener
     * @return string
     */
    protected function stringifyClosure(Closure $rawListener)
    {}

    /**
     * Filter the given events using the provided event name filter.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @return \Illuminate\Support\Collection
     */
    protected function filterEvents($events)
    {}

    /**
     * Determine whether the user is filtering by an event name.
     *
     * @return bool
     */
    protected function filteringByEvent()
    {}

    /**
     * Gets the raw version of event listeners from the event dispatcher.
     *
     * @return array
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function getRawListeners()
    {}

    /**
     * Get the event dispatcher.
     *
     * @return \Illuminate\Events\Dispatcher
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function getEventsDispatcher()
    {}

    /**
     * Set a callback that should be used when resolving the events dispatcher.
     *
     * @param  \Closure|null  $resolver
     * @return void
     */
    public static function resolveEventsUsing($resolver)
    {}
}
