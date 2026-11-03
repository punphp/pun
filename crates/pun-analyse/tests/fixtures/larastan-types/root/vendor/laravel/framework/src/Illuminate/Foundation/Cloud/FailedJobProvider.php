<?php

namespace Illuminate\Foundation\Cloud;

use Carbon\CarbonImmutable;
use DateTimeInterface;
use Illuminate\Contracts\Encryption\StringEncrypter;
use Illuminate\Http\Client\RequestException;
use Illuminate\Http\Client\Response;
use Illuminate\Queue\Failed\CountableFailedJobProvider;
use Illuminate\Queue\Failed\FailedJobProviderInterface;
use Illuminate\Queue\Failed\PrunableFailedJobProvider;
use Illuminate\Support\Facades\Config;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\LazyCollection;
use Illuminate\Support\Str;
use Iterator;
use RuntimeException;
use Throwable;

class FailedJobProvider implements FailedJobProviderInterface, CountableFailedJobProvider, PrunableFailedJobProvider
{
    /**
     * The connected queue instance.
     *
     * @var ?\Illuminate\Foundation\Cloud\Queue
     */
    protected $queue = null;

    /**
     * The loaded failed jobs keyed by ID.
     *
     * @var array<string, object>
     */
    protected $loadedFailedJobs = [];

    /**
     * Create a new instance.
     */
    public function __construct(
        protected FailedJobProviderInterface $failer,
        protected Events $events,
        protected StringEncrypter $encrypter,
        protected ?ExceptionReporter $reporter = null,
    ) {}

    /**
     * Log a failed job into storage.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @param  string  $payload
     * @param  \Throwable  $exception
     * @return string|null
     */
    public function log($connection, $queue, $payload, $exception)
    {}

    /**
     * Get the IDs of all of the failed jobs.
     *
     * @param  string|null  $queue
     * @return array
     */
    public function ids($queue = null)
    {}

    /**
     * Get a list of all of the failed jobs.
     *
     * @return array
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
     * Build an iterator for the failed jobs from the given URL.
     */
    protected function failedJobsIterator(string $url): Iterator
    { yield; }

    /**
     * Resolve the failed jobs payload for the given URL.
     *
     * @return object{data: list<object>, links: object{self: string, next: string|null}}
     */
    protected function resolveFailedJobsPayload(string $url): object
    {}

    /**
     * Fetch the failed jobs from the given URL.
     */
    protected function fetchFailedJobs(string $url): Response
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
     * Count the failed jobs.
     *
     * @param  string|null  $connection
     * @param  string|null  $queue
     * @return int
     */
    public function count($connection = null, $queue = null)
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
     * Set the connected queue instance.
     *
     * @param  \Illuminate\Foundation\Cloud\Queue  $queue
     * @return $this
     */
    public function setQueue($queue)
    {}
}
