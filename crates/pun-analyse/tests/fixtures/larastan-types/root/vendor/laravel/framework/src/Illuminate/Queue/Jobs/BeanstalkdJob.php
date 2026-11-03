<?php

namespace Illuminate\Queue\Jobs;

use Illuminate\Container\Container;
use Illuminate\Contracts\Queue\Job as JobContract;
use Pheanstalk\Contract\JobIdInterface;
use Pheanstalk\Pheanstalk;

class BeanstalkdJob extends Job implements JobContract
{
    /**
     * The Pheanstalk instance.
     *
     * @var \Pheanstalk\Contract\PheanstalkManagerInterface&\Pheanstalk\Contract\PheanstalkPublisherInterface&\Pheanstalk\Contract\PheanstalkSubscriberInterface
     */
    protected $pheanstalk;

    /**
     * The Pheanstalk job instance.
     *
     * @var \Pheanstalk\Job
     */
    protected $job;

    /**
     * Create a new job instance.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  \Pheanstalk\Contract\PheanstalkManagerInterface&\Pheanstalk\Contract\PheanstalkPublisherInterface&\Pheanstalk\Contract\PheanstalkSubscriberInterface  $pheanstalk
     * @param  \Pheanstalk\Contract\JobIdInterface  $job
     * @param  string  $connectionName
     * @param  string  $queue
     */
    public function __construct(Container $container, $pheanstalk, JobIdInterface $job, $connectionName, $queue)
    {}

    /**
     * Release the job back into the queue after (n) seconds.
     *
     * @param  int  $delay
     * @return void
     */
    public function release($delay = 0)
    {}

    /**
     * Bury the job in the queue.
     *
     * @return void
     */
    public function bury()
    {}

    /**
     * Delete the job from the queue.
     *
     * @return void
     */
    public function delete()
    {}

    /**
     * Get the number of times the job has been attempted.
     *
     * @return int
     */
    public function attempts()
    {}

    /**
     * Get the job identifier.
     *
     * @return int
     */
    public function getJobId()
    {}

    /**
     * Get the raw body string for the job.
     *
     * @return string
     */
    public function getRawBody()
    {}

    /**
     * Get the underlying Pheanstalk instance.
     *
     * @return \Pheanstalk\Contract\PheanstalkManagerInterface&\Pheanstalk\Contract\PheanstalkPublisherInterface&\Pheanstalk\Contract\PheanstalkSubscriberInterface
     */
    public function getPheanstalk()
    {}

    /**
     * Get the underlying Pheanstalk job.
     *
     * @return \Pheanstalk\Contract\JobIdInterface
     */
    public function getPheanstalkJob()
    {}
}
