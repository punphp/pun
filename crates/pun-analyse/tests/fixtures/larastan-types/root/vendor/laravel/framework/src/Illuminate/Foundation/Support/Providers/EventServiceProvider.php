<?php

namespace Illuminate\Foundation\Support\Providers;

use Illuminate\Auth\Events\Registered;
use Illuminate\Auth\Listeners\SendEmailVerificationNotification;
use Illuminate\Foundation\Events\DiscoverEvents;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Event;
use Illuminate\Support\LazyCollection;
use Illuminate\Support\ServiceProvider;

class EventServiceProvider extends ServiceProvider
{
    /**
     * The event handler mappings for the application.
     *
     * @var array<string, array<int, string>>
     */
    protected $listen = [];

    /**
     * The subscribers to register.
     *
     * @var array
     */
    protected $subscribe = [];

    /**
     * The model observers to register.
     *
     * @var array<string, string|object|array<int, string|object>>
     */
    protected $observers = [];

    /**
     * Indicates if events should be discovered.
     *
     * @var bool
     */
    protected static $shouldDiscoverEvents = true;

    /**
     * The configured event discovery paths.
     *
     * @var iterable<int, string>|null
     */
    protected static $eventDiscoveryPaths;

    /**
     * Register the application's event listeners.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Boot any application services.
     *
     * @return void
     */
    public function boot()
    {}

    /**
     * Get the events and handlers.
     *
     * @return array<string, array<int, string>>
     */
    public function listens()
    {}

    /**
     * Get the discovered events and listeners for the application.
     *
     * @return array
     */
    public function getEvents()
    {}

    /**
     * Get the discovered events for the application.
     *
     * @return array
     */
    protected function discoveredEvents()
    {}

    /**
     * Determine if events and listeners should be automatically discovered.
     *
     * @return bool
     */
    public function shouldDiscoverEvents()
    {}

    /**
     * Discover the events and listeners for the application.
     *
     * @return array
     */
    public function discoverEvents()
    {}

    /**
     * Get the listener directories that should be used to discover events.
     *
     * @return iterable<int, string>
     */
    protected function discoverEventsWithin()
    {}

    /**
     * Add the given event discovery paths to the application's event discovery paths.
     *
     * @param  string|iterable<int, string>  $paths
     * @return void
     */
    public static function addEventDiscoveryPaths(iterable|string $paths)
    {}

    /**
     * Set the globally configured event discovery paths.
     *
     * @param  iterable<int, string>  $paths
     * @return void
     */
    public static function setEventDiscoveryPaths(iterable $paths)
    {}

    /**
     * Get the base path to be used during event discovery.
     *
     * @return string
     */
    protected function eventDiscoveryBasePath()
    {}

    /**
     * Disable event discovery for the application.
     *
     * @return void
     */
    public static function disableEventDiscovery()
    {}

    /**
     * Configure the proper event listeners for email verification.
     *
     * @return void
     */
    protected function configureEmailVerification()
    {}
}
