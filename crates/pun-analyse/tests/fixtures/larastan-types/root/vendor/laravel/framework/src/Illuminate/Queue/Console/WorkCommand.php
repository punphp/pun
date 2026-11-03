<?php

namespace Illuminate\Queue\Console;

use Illuminate\Console\Command;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Contracts\Queue\Job;
use Illuminate\Queue\Events\JobFailed;
use Illuminate\Queue\Events\JobProcessed;
use Illuminate\Queue\Events\JobProcessing;
use Illuminate\Queue\Events\JobReleasedAfterException;
use Illuminate\Queue\Events\WorkerQueuePaused;
use Illuminate\Queue\Events\WorkerQueueResumed;
use Illuminate\Queue\Events\WorkerStopping;
use Illuminate\Queue\Worker;
use Illuminate\Queue\WorkerOptions;
use Illuminate\Support\Carbon;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Stringable;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Terminal;
use Throwable;

use function Termwind\terminal;

#[AsCommand(name: 'queue:work')]
class WorkCommand extends Command
{
    use InteractsWithTime;

    /**
     * The console command name.
     *
     * @var string
     */
    protected $signature = 'queue:work
                            {connection? : The name of the queue connection to work}
                            {--name=default : The name of the worker}
                            {--queue= : The names of the queues to work}
                            {--daemon : Run the worker in daemon mode (Deprecated)}
                            {--once : Only process the next job on the queue}
                            {--stop-when-empty : Stop when the queue is empty}
                            {--stop-when-empty-for=0 : Stop when no jobs have been processed for the given number of seconds}
                            {--delay=0 : The number of seconds to delay failed jobs (Deprecated)}
                            {--backoff=0 : The number of seconds to wait before retrying a job that encountered an uncaught exception}
                            {--max-jobs=0 : The number of jobs to process before stopping}
                            {--max-time=0 : The maximum number of seconds the worker should run}
                            {--force : Force the worker to run even in maintenance mode}
                            {--memory=128 : The memory limit in megabytes, or as a percentage of the PHP memory limit}
                            {--sleep=3 : The number of seconds to sleep when no job is available}
                            {--rest=0 : The number of seconds to rest between jobs}
                            {--timeout=60 : The number of seconds a child process can run}
                            {--tries=1 : The number of times to attempt a job before logging it failed}
                            {--json : Output the queue worker information as JSON}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Start processing jobs on the queue as a daemon';

    /**
     * The queue worker instance.
     *
     * @var \Illuminate\Queue\Worker
     */
    protected $worker;

    /**
     * The cache store implementation.
     *
     * @var \Illuminate\Contracts\Cache\Repository
     */
    protected $cache;

    /**
     * Holds the start time of the last processed job, if any.
     *
     * @var float|null
     */
    protected $latestStartedAt;

    /**
     * Indicates if the worker's event listeners have been registered.
     *
     * @var bool
     */
    private static $hasRegisteredListeners = false;

    /**
     * Create a new queue work command.
     *
     * @param  \Illuminate\Queue\Worker  $worker
     * @param  \Illuminate\Contracts\Cache\Repository  $cache
     */
    public function __construct(Worker $worker, Cache $cache)
    {}

    /**
     * Execute the console command.
     *
     * @return int|null
     */
    public function handle()
    {}

    /**
     * Run the worker instance.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @return int|null
     */
    protected function runWorker($connection, $queue)
    {}

    /**
     * Gather all of the queue worker options as a single object.
     *
     * @return \Illuminate\Queue\WorkerOptions
     */
    protected function gatherWorkerOptions()
    {}

    /**
     * Listen for the queue events in order to update the console output.
     *
     * @return void
     */
    protected function listenForEvents()
    {}

    /**
     * Write the status output for the queue worker for JSON or TTY.
     *
     * @param  Job  $job
     * @param  string  $status
     * @param  \Throwable|null  $exception
     * @return void
     */
    protected function writeOutput(Job $job, $status, ?Throwable $exception = null)
    {}

    /**
     * Write the status output for a paused or resumed queue.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @param  string  $status
     * @return void
     */
    protected function writeQueueStatus($connection, $queue, $status)
    {}

    /**
     * Write the status output for a queue worker that is stopping.
     *
     * @param  \Illuminate\Queue\Events\WorkerStopping  $event
     * @return void
     */
    protected function writeStopReason(WorkerStopping $event)
    {}

    /**
     * Write the status output for the queue worker.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  string  $status
     * @return void
     */
    protected function writeOutputForCli(Job $job, $status)
    {}

    /**
     * Write the status output for the queue worker in JSON format.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  string  $status
     * @param  \Throwable|null  $exception
     * @return void
     */
    protected function writeOutputAsJson(Job $job, $status, ?Throwable $exception = null)
    {}

    /**
     * Get the current date / time.
     *
     * @return \Illuminate\Support\Carbon
     */
    protected function now()
    {}

    /**
     * Store a failed job event.
     *
     * @param  \Illuminate\Queue\Events\JobFailed  $event
     * @return void
     */
    protected function logFailedJob(JobFailed $event)
    {}

    /**
     * Get the queue name for the worker.
     *
     * @param  string  $connection
     * @return string
     */
    protected function getQueue($connection)
    {}

    /**
     * Determine if the worker should run in maintenance mode.
     *
     * @return bool
     */
    protected function downForMaintenance()
    {}

    /**
     * Determine if the worker should output using JSON.
     *
     * @return bool
     */
    protected function outputUsingJson()
    {}

    /**
     * Reset static variables.
     *
     * @return void
     */
    public static function flushState()
    {}
}
