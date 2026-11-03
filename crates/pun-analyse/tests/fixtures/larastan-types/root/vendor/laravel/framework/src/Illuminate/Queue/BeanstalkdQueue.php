<?php

namespace Illuminate\Queue;

use Illuminate\Contracts\Queue\Queue as QueueContract;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Jobs\BeanstalkdJob;
use Illuminate\Support\Collection;
use Pheanstalk\Contract\JobIdInterface;
use Pheanstalk\Pheanstalk;
use Pheanstalk\Values\Job;
use Pheanstalk\Values\JobId;
use Pheanstalk\Values\TubeName;

use function Illuminate\Support\enum_value;

class BeanstalkdQueue extends Queue implements QueueContract
{
    /**
     * The Pheanstalk instance.
     *
     * @var \Pheanstalk\Contract\PheanstalkManagerInterface&\Pheanstalk\Contract\PheanstalkPublisherInterface&\Pheanstalk\Contract\PheanstalkSubscriberInterface
     */
    protected $pheanstalk;

    /**
     * The name of the default tube.
     *
     * @var string
     */
    protected $default;

    /**
     * The "time to run" for all pushed jobs.
     *
     * @var int
     */
    protected $timeToRun;

    /**
     * The maximum number of seconds to block for a job.
     *
     * @var int
     */
    protected $blockFor;

    /**
     * Create a new Beanstalkd queue instance.
     *
     * @param  \Pheanstalk\Contract\PheanstalkManagerInterface&\Pheanstalk\Contract\PheanstalkPublisherInterface&\Pheanstalk\Contract\PheanstalkSubscriberInterface  $pheanstalk
     * @param  string  $default
     * @param  int  $timeToRun
     * @param  int  $blockFor
     * @param  bool  $dispatchAfterCommit
     */
    public function __construct(
        $pheanstalk,
        $default,
        $timeToRun,
        $blockFor = 0,
        $dispatchAfterCommit = false,
    ) {}

    /**
     * Get the size of the queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function size($queue = null)
    {}

    /**
     * Get the number of pending jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function pendingSize($queue = null)
    {}

    /**
     * Get the number of delayed jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function delayedSize($queue = null)
    {}

    /**
     * Get the number of reserved jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function reservedSize($queue = null)
    {}

    /**
     * Get the number of jobs across every queue.
     *
     * @return int
     */
    public function totalSize()
    {}

    /**
     * Get the number of pending jobs across every queue.
     *
     * @return int
     */
    public function totalPendingSize()
    {}

    /**
     * Get the number of delayed jobs across every queue.
     *
     * @return int
     */
    public function totalDelayedSize()
    {}

    /**
     * Get the number of reserved jobs across every queue.
     *
     * @return int
     */
    public function totalReservedSize()
    {}

    /**
     * Get the pending jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function pendingJobs($queue = null): Collection
    {}

    /**
     * Get the delayed jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function delayedJobs($queue = null): Collection
    {}

    /**
     * Get the reserved jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function reservedJobs($queue = null): Collection
    {}

    /**
     * Get all pending jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allPendingJobs(): Collection
    {}

    /**
     * Get all delayed jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allDelayedJobs(): Collection
    {}

    /**
     * Get all reserved jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allReservedJobs(): Collection
    {}

    /**
     * Get the creation timestamp of the oldest pending job, excluding delayed jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int|null
     */
    public function creationTimeOfOldestPendingJob($queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  string  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function push($job, $data = '', $queue = null)
    {}

    /**
     * Push a raw payload onto the queue.
     *
     * @param  string  $payload
     * @param  \UnitEnum|string|null  $queue
     * @param  array  $options
     * @return mixed
     */
    public function pushRaw($payload, $queue = null, array $options = [])
    {}

    /**
     * Push a new job onto the queue after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  string  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function later($delay, $job, $data = '', $queue = null)
    {}

    /**
     * Push an array of jobs onto the queue.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return void
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Pop the next job off of the queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Contracts\Queue\Job|null
     */
    public function pop($queue = null)
    {}

    /**
     * Delete a message from the Beanstalk queue.
     *
     * @param  \UnitEnum|string  $queue
     * @param  string|int  $id
     * @return void
     */
    public function deleteMessage($queue, $id)
    {}

    /**
     * Get the queue or return the default.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return string
     */
    public function getQueue($queue)
    {}

    /**
     * Get the underlying Pheanstalk instance.
     *
     * @return \Pheanstalk\Contract\PheanstalkManagerInterface&\Pheanstalk\Contract\PheanstalkPublisherInterface&\Pheanstalk\Contract\PheanstalkSubscriberInterface
     */
    public function getPheanstalk()
    {}
}
