<?php

namespace Illuminate\Queue\Console\Concerns;

trait ParsesQueue
{
    /**
     * Parse the queue argument into connection and queue name.
     *
     * @param  string  $queue
     * @return array{string, string}
     */
    protected function parseQueue($queue)
    {}
}
