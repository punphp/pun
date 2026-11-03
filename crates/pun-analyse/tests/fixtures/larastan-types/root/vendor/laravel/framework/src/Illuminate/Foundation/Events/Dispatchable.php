<?php

namespace Illuminate\Foundation\Events;

trait Dispatchable
{
    /**
     * Dispatch the event with the given arguments.
     *
     * @param  mixed  ...$arguments
     * @return mixed
     */
    public static function dispatch(...$arguments)
    {}

    /**
     * Dispatch the event with the given arguments if the given truth test passes.
     *
     * @param  bool  $boolean
     * @param  mixed  ...$arguments
     * @return mixed
     */
    public static function dispatchIf($boolean, ...$arguments)
    {}

    /**
     * Dispatch the event with the given arguments unless the given truth test passes.
     *
     * @param  bool  $boolean
     * @param  mixed  ...$arguments
     * @return mixed
     */
    public static function dispatchUnless($boolean, ...$arguments)
    {}

    /**
     * Broadcast the event with the given arguments.
     *
     * @param  mixed  ...$arguments
     * @return \Illuminate\Broadcasting\PendingBroadcast
     */
    public static function broadcast(...$arguments)
    {}
}
