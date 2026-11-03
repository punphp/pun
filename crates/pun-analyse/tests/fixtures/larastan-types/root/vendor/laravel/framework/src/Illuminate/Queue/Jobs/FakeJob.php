<?php

namespace Illuminate\Queue\Jobs;

use Illuminate\Contracts\Queue\Job as JobContract;
use Illuminate\Support\Str;

class FakeJob extends Job implements JobContract
{
    /**
     * The number of seconds the released job was delayed.
     *
     * @var int
     */
    public $releaseDelay;

    /**
     * The number of attempts made to process the job.
     *
     * @var int
     */
    public $attempts = 1;

    /**
     * The exception the job failed with.
     *
     * @var \Throwable|null
     */
    public $failedWith;

    /**
     * Get the job identifier.
     *
     * @return string
     */
    public function getJobId()
    {}

    /**
     * Get the raw body of the job.
     *
     * @return string
     */
    public function getRawBody()
    {}

    /**
     * Release the job back into the queue after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @return void
     */
    public function release($delay = 0)
    {}

    /**
     * Get the number of times the job has been attempted.
     *
     * @return int
     */
    public function attempts()
    {}

    /**
     * Delete the job from the queue.
     *
     * @return void
     */
    public function delete()
    {}

    /**
     * Delete the job, call the "failed" method, and raise the failed job event.
     *
     * @param  \Throwable|null  $e
     * @return void
     */
    public function fail($e = null)
    {}
}
