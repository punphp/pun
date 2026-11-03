<?php

namespace Illuminate\Foundation\Cloud;

use Illuminate\Contracts\Container\Container;
use Illuminate\Support\Traits\Macroable;
use RuntimeException;

class CloudManager
{
    use Macroable;

    /**
     * Create a new Laravel Cloud manager instance.
     */
    public function __construct(protected Container $container)
    {}

    /**
     * Determine if the application is currently hosted on Laravel Cloud.
     */
    public function hosted(): bool
    {}

    /**
     * Determine if the application is using Laravel Cloud managed queues.
     */
    public function usesManagedQueues(): bool
    {}

    /**
     * Get the Laravel Cloud managed queue connection.
     *
     * @throws \RuntimeException
     */
    public function queue(): Queue
    {}

    /**
     * Determine if the given queue is managed by Laravel Cloud.
     */
    public function isManagedQueue(string $queue): bool
    {}
}
