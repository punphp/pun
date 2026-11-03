<?php

namespace Illuminate\Support\Queue\Concerns;

use Illuminate\Container\Container;
use Illuminate\Queue\QueueRoutes;

trait ResolvesQueueRoutes
{
    /**
     * Resolve the default connection name for a given queueable instance.
     *
     * @param  object  $queueable
     * @return string|null
     */
    public function resolveConnectionFromQueueRoute($queueable)
    {}

    /**
     * Resolve the default queue name for a given queueable instance.
     *
     * @param  object  $queueable
     * @return string|null
     */
    public function resolveQueueFromQueueRoute($queueable)
    {}

    /**
     * Get the queue routes manager instance.
     *
     * @return \Illuminate\Queue\QueueRoutes
     */
    protected function queueRoutes()
    {}
}
