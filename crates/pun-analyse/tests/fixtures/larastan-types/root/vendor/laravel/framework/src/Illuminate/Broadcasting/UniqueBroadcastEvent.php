<?php

namespace Illuminate\Broadcasting;

use Illuminate\Container\Container;
use Illuminate\Contracts\Cache\Repository;
use Illuminate\Contracts\Queue\ShouldBeUnique;
use Illuminate\Queue\Attributes\UniqueFor;

class UniqueBroadcastEvent extends BroadcastEvent implements ShouldBeUnique
{
    /**
     * The unique lock identifier.
     *
     * @var mixed
     */
    public $uniqueId;

    /**
     * The number of seconds the unique lock should be maintained.
     *
     * @var int
     */
    public $uniqueFor;

    /**
     * Create a new event instance.
     *
     * @param  mixed  $event
     */
    public function __construct($event)
    {}

    /**
     * Resolve the cache implementation that should manage the event's uniqueness.
     *
     * @return \Illuminate\Contracts\Cache\Repository
     */
    public function uniqueVia()
    {}
}
