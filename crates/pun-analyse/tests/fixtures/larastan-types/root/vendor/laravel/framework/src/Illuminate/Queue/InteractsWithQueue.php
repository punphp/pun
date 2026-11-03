<?php

namespace Illuminate\Queue;

use DateTimeInterface;
use Illuminate\Contracts\Queue\Job as JobContract;
use Illuminate\Queue\Jobs\FakeJob;
use Illuminate\Support\InteractsWithTime;
use InvalidArgumentException;
use PHPUnit\Framework\Assert as PHPUnit;
use RuntimeException;
use Throwable;

trait InteractsWithQueue
{
    use InteractsWithTime;

    /**
     * The underlying queue job instance.
     *
     * @var \Illuminate\Contracts\Queue\Job|null
     */
    public $job;

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
     * Fail the job from the queue.
     *
     * @param  \Throwable|string|null  $exception
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    public function fail($exception = null)
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
     * Indicate that queue interactions like fail, delete, and release should be faked.
     *
     * @return $this
     */
    public function withFakeQueueInteractions()
    {}

    /**
     * Assert that the job was deleted from the queue.
     *
     * @return $this
     */
    public function assertDeleted()
    {}

    /**
     * Assert that the job was not deleted from the queue.
     *
     * @return $this
     */
    public function assertNotDeleted()
    {}

    /**
     * Assert that the job was manually failed.
     *
     * @return $this
     */
    public function assertFailed()
    {}

    /**
     * Assert that the job was manually failed with a specific exception.
     *
     * @param  \Throwable|string  $exception
     * @return $this
     */
    public function assertFailedWith($exception)
    {}

    /**
     * Assert that the job was not manually failed.
     *
     * @return $this
     */
    public function assertNotFailed()
    {}

    /**
     * Assert that the job was released back onto the queue.
     *
     * @param  \DateTimeInterface|\DateInterval|int|null  $delay
     * @return $this
     */
    public function assertReleased($delay = null)
    {}

    /**
     * Assert that the job was not released back onto the queue.
     *
     * @return $this
     */
    public function assertNotReleased()
    {}

    /**
     * Ensure that queue interactions have been faked.
     *
     * @return void
     *
     * @throws \RuntimeException
     */
    private function ensureQueueInteractionsHaveBeenFaked()
    {}

    /**
     * Set the base queue job instance.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @return $this
     */
    public function setJob(JobContract $job)
    {}
}
