<?php

namespace Illuminate\Queue;

use Illuminate\Queue\Attributes\Queue as QueueAttribute;
use Illuminate\Support\Traits\ReadsClassAttributes;
use UnitEnum;

use function Illuminate\Support\enum_value;

class QueueRoutes
{
    use ReadsClassAttributes;

    /**
     * The mapping of class names to their default routes.
     *
     * @var array<class-string, array|string>
     */
    protected $routes = [];

    /**
     * The queues that have been forwarded to another queue and/or connection.
     *
     * @var array<string, array>
     */
    protected $forwards = [];

    /**
     * Get the queue connection that a given queueable instance should be routed to.
     *
     * @param  object  $queueable
     * @return string|null
     */
    public function getConnection($queueable)
    {}

    /**
     * Get the connection the given queue has been forwarded to.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return string|null
     */
    protected function forwardedConnection($queue)
    {}

    /**
     * Get the queue that a given queueable instance should be routed to.
     *
     * @param  object  $queueable
     * @return string|null
     */
    public function getQueue($queueable)
    {}

    /**
     * Get the queue the given queue has been forwarded to.
     *
     * @param  string  $queue
     * @param  string|null  $connection
     * @return string
     */
    public function forwardedQueue($queue, $connection = null)
    {}

    /**
     * Get the route for a given queueable instance.
     *
     * @param  object  $queueable
     * @return array|string|null
     */
    public function getRoute($queueable)
    {}

    /**
     * Register the queue route for the given class.
     *
     * @param  array|class-string  $class
     * @param  \UnitEnum|string|null  $queue
     * @param  \UnitEnum|string|null  $connection
     * @return void
     */
    public function set(array|string $class, $queue = null, $connection = null)
    {}

    /**
     * Register a forward for the given queue.
     *
     * @param  \UnitEnum|array<string, \UnitEnum|string>|string  $queue
     * @param  \UnitEnum|string|null  $to
     * @param  \UnitEnum|string|null  $connection
     * @return void
     */
    public function forward(UnitEnum|array|string $queue, $to = null, $connection = null)
    {}

    /**
     * Get all registered queue routes.
     *
     * @return array<class-string, array|string>
     */
    public function all()
    {}
}
