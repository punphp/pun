<?php

namespace Illuminate\Broadcasting;

use Illuminate\Contracts\Events\Dispatcher;

use function Illuminate\Support\enum_value;

class PendingBroadcast
{
    /**
     * The event dispatcher implementation.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The event instance.
     *
     * @var mixed
     */
    protected $event;

    /**
     * Create a new pending broadcast instance.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @param  mixed  $event
     */
    public function __construct(Dispatcher $events, $event)
    {}

    /**
     * Broadcast the event using a specific broadcaster.
     *
     * @param  \UnitEnum|string|null  $connection
     * @return $this
     */
    public function via($connection = null)
    {}

    /**
     * Broadcast the event to everyone except the current user.
     *
     * @return $this
     */
    public function toOthers()
    {}

    /**
     * Handle the object's destruction.
     *
     * @return void
     */
    public function __destruct()
    {}
}
