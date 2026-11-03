<?php

namespace Illuminate\Broadcasting;

use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Broadcasting\Factory as BroadcastingFactory;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Queue\Attributes\Backoff;
use Illuminate\Queue\Attributes\DeleteWhenMissingModels;
use Illuminate\Queue\Attributes\MaxExceptions;
use Illuminate\Queue\Attributes\ReadsQueueAttributes;
use Illuminate\Queue\Attributes\Timeout;
use Illuminate\Queue\Attributes\Tries;
use Illuminate\Support\Arr;
use ReflectionClass;
use ReflectionProperty;
use Throwable;

use function Illuminate\Support\enum_value;

class BroadcastEvent implements ShouldQueue
{
    use Queueable, ReadsQueueAttributes;

    /**
     * The event instance.
     *
     * @var mixed
     */
    public $event;

    /**
     * The number of times the job may be attempted.
     *
     * @var int
     */
    public $tries;

    /**
     * The number of seconds the job can run before timing out.
     *
     * @var int
     */
    public $timeout;

    /**
     * The number of seconds to wait before retrying the job when encountering an uncaught exception.
     *
     * @var int
     */
    public $backoff;

    /**
     * The maximum number of unhandled exceptions to allow before failing.
     *
     * @var int
     */
    public $maxExceptions;

    /**
     * Delete the job if its models no longer exist.
     *
     * @var bool
     */
    public $deleteWhenMissingModels = true;

    /**
     * Create a new job handler instance.
     *
     * @param  mixed  $event
     */
    public function __construct($event)
    {}

    /**
     * Handle the queued job.
     *
     * @param  \Illuminate\Contracts\Broadcasting\Factory  $manager
     * @return void
     */
    public function handle(BroadcastingFactory $manager)
    {}

    /**
     * Get the payload for the given event.
     *
     * @param  mixed  $event
     * @return array
     */
    protected function getPayloadFromEvent($event)
    {}

    /**
     * Format the given value for a property.
     *
     * @param  mixed  $value
     * @return mixed
     */
    protected function formatProperty($value)
    {}

    /**
     * Get the channels for the given connection.
     *
     * @param  array  $channels
     * @param  string|null  $connection
     * @return array
     */
    protected function getConnectionChannels($channels, $connection)
    {}

    /**
     * Get the payload for the given connection.
     *
     * @param  array  $payload
     * @param  string|null  $connection
     * @return array
     */
    protected function getConnectionPayload($payload, $connection)
    {}

    /**
     * Get the middleware for the underlying event.
     *
     * @return array<int, object>
     */
    public function middleware(): array
    {}

    /**
     * Handle a job failure.
     *
     * @param  \Throwable|null  $e
     * @return void
     */
    public function failed(?Throwable $e = null): void
    {}

    /**
     * Get the display name for the queued job.
     *
     * @return string
     */
    public function displayName()
    {}

    /**
     * Prepare the instance for cloning.
     *
     * @return void
     */
    public function __clone()
    {}
}
