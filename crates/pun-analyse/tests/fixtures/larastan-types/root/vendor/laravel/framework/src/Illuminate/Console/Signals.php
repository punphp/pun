<?php

namespace Illuminate\Console;

/**
 * @internal
 */
class Signals
{
    /**
     * The signal registry instance.
     *
     * @var \Symfony\Component\Console\SignalRegistry\SignalRegistry
     */
    protected $registry;

    /**
     * The signal registry's previous list of handlers.
     *
     * @var array<int, array<int, callable(int): void>>|null
     */
    protected $previousHandlers;

    /**
     * The current availability resolver, if any.
     *
     * @var (callable(): bool)|null
     */
    protected static $availabilityResolver;

    /**
     * Create a new signal registrar instance.
     *
     * @param  \Symfony\Component\Console\SignalRegistry\SignalRegistry  $registry
     */
    public function __construct($registry)
    {}

    /**
     * Register a new signal handler.
     *
     * @param  int  $signal
     * @param  callable(int $signal): void  $callback
     * @return void
     */
    public function register($signal, $callback)
    {}

    /**
     * Gets the signal's existing handler in array format.
     *
     * @return array<int, callable(int $signal): void>|null
     */
    protected function initializeSignal($signal)
    {}

    /**
     * Unregister the current signal handlers.
     *
     * @return void
     */
    public function unregister()
    {}

    /**
     * Execute the given callback if "signals" should be used and are available.
     *
     * @param  callable  $callback
     * @return void
     */
    public static function whenAvailable($callback)
    {}

    /**
     * Get the registry's handlers.
     *
     * @return array<int, array<int, callable>>
     */
    protected function getHandlers()
    {}

    /**
     * Set the registry's handlers.
     *
     * @param  array<int, array<int, callable(int $signal):void>>  $handlers
     * @return void
     */
    protected function setHandlers($handlers)
    {}

    /**
     * Set the availability resolver.
     *
     * @param  (callable(): bool)  $resolver
     * @return void
     */
    public static function resolveAvailabilityUsing($resolver)
    {}
}
