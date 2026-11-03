<?php

namespace Orchestra\Testbench\Foundation\Console;

use Illuminate\Support\Collection;

/**
 * @internal
 */
final class TerminatingConsole
{
    /**
     * The terminating callbacks.
     *
     * @var array<int, (callable(?int):void)>
     */
    protected static array $beforeTerminatingCallbacks = [];

    /**
     * Register a callback to be run before terminating the command.
     *
     * @param  callable(?int):void  $callback
     * @return void
     */
    public static function before(callable $callback): void
    {}

    /**
     * Register a callback to be run before terminating the command.
     *
     * @param  bool  $condition
     * @param  callable(?int):void  $callback
     * @return void
     */
    public static function beforeWhen(bool $condition, callable $callback): void
    {}

    /**
     * Handle terminating console.
     *
     * @param  ?int  $signal
     * @return void
     */
    public static function handle(?int $signal = null): void
    {}

    /**
     * Purge terminating console callbacks.
     *
     * @return void
     */
    public static function flush(): void
    {}
}
