<?php

namespace Illuminate\Queue\Failed;

use Closure;
use DateTimeInterface;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Date;

class FileFailedJobProvider implements CountableFailedJobProvider, FailedJobProviderInterface, PrunableFailedJobProvider
{
    /**
     * The file path where the failed job file should be stored.
     *
     * @var string
     */
    protected $path;

    /**
     * The maximum number of failed jobs to retain.
     *
     * @var int
     */
    protected $limit;

    /**
     * The lock provider resolver.
     *
     * @var \Closure|null
     */
    protected $lockProviderResolver;

    /**
     * Create a new file failed job provider.
     *
     * @param  string  $path
     * @param  int  $limit
     * @param  \Closure|null  $lockProviderResolver
     */
    public function __construct($path, $limit = 100, ?Closure $lockProviderResolver = null)
    {}

    /**
     * Log a failed job into storage.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @param  string  $payload
     * @param  \Throwable  $exception
     * @return string|int|null
     */
    public function log($connection, $queue, $payload, $exception)
    {}

    /**
     * Get the IDs of all of the failed jobs.
     *
     * @param  string|null  $queue
     * @return list<string|int>
     */
    public function ids($queue = null)
    {}

    /**
     * Get a list of all of the failed jobs.
     *
     * @return list<object>
     */
    public function all()
    {}

    /**
     * Get a single failed job.
     *
     * @param  mixed  $id
     * @return object|null
     */
    public function find($id)
    {}

    /**
     * Delete a single failed job from storage.
     *
     * @param  mixed  $id
     * @return bool
     */
    public function forget($id)
    {}

    /**
     * Flush the failed jobs from storage.
     *
     * @param  int|null  $hours
     * @param  string|null  $queue
     * @return void
     */
    public function flush($hours = null, $queue = null)
    {}

    /**
     * Prune all of the entries older than the given date.
     *
     * @param  \DateTimeInterface  $before
     * @return int
     */
    public function prune(DateTimeInterface $before)
    {}

    /**
     * Execute the given callback while holding a lock.
     *
     * @template TReturn
     *
     * @param  (\Closure(): TReturn)  $callback
     * @return TReturn
     */
    protected function lock(Closure $callback)
    {}

    /**
     * Read the failed jobs file.
     *
     * @return array
     */
    protected function read()
    {}

    /**
     * Write the given array of jobs to the failed jobs file.
     *
     * @param  array  $jobs
     * @return void
     */
    protected function write(array $jobs)
    {}

    /**
     * Count the failed jobs.
     *
     * @param  string|null  $connection
     * @param  string|null  $queue
     * @return int
     */
    public function count($connection = null, $queue = null)
    {}
}
