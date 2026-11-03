<?php

namespace Illuminate\Queue;

use Illuminate\Contracts\Cache\Repository as CacheContract;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Queue\Factory as QueueManager;
use Illuminate\Contracts\Queue\Interruptible;
use Illuminate\Database\DetectsLostConnections;
use Illuminate\Queue\Events\JobAttempted;
use Illuminate\Queue\Events\JobExceptionOccurred;
use Illuminate\Queue\Events\JobInterrupted;
use Illuminate\Queue\Events\JobPopped;
use Illuminate\Queue\Events\JobPopping;
use Illuminate\Queue\Events\JobProcessed;
use Illuminate\Queue\Events\JobProcessing;
use Illuminate\Queue\Events\JobReleased;
use Illuminate\Queue\Events\JobReleasedAfterException;
use Illuminate\Queue\Events\JobTimedOut;
use Illuminate\Queue\Events\Looping;
use Illuminate\Queue\Events\WorkerIdle;
use Illuminate\Queue\Events\WorkerInterrupted;
use Illuminate\Queue\Events\WorkerPausing;
use Illuminate\Queue\Events\WorkerQueuePaused;
use Illuminate\Queue\Events\WorkerQueueResumed;
use Illuminate\Queue\Events\WorkerResuming;
use Illuminate\Queue\Events\WorkerStarting;
use Illuminate\Queue\Events\WorkerStopping;
use Illuminate\Support\Carbon;
use Throwable;

class Worker
{
    use DetectsLostConnections;

    const EXIT_SUCCESS = 0;
    const EXIT_ERROR = 1;
    const EXIT_MEMORY_LIMIT = 12;

    /**
     * The name of the worker.
     *
     * @var string|null
     */
    protected $name;

    /**
     * The queue manager instance.
     *
     * @var \Illuminate\Contracts\Queue\Factory
     */
    protected $manager;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The cache repository implementation.
     *
     * @var \Illuminate\Contracts\Cache\Repository
     */
    protected $cache;

    /**
     * The exception handler instance.
     *
     * @var \Illuminate\Contracts\Debug\ExceptionHandler
     */
    protected $exceptions;

    /**
     * The callback used to determine if the application is in maintenance mode.
     *
     * @var callable
     */
    protected $isDownForMaintenance;

    /**
     * The callback used to reset the application's scope.
     *
     * @var callable
     */
    protected $resetScope;

    /**
     * The number of jobs processed by the worker.
     *
     * @var int|null
     */
    protected $jobsProcessed;

    /**
     * The timestamp of the last job processed by the worker.
     *
     * @var int|float|null
     */
    protected $lastJobProcessedAt;

    /**
     * The job currently being processed.
     *
     * @var \Illuminate\Contracts\Queue\Job|null
     */
    public $currentJob = null;

    /**
     * Indicates if the worker should exit.
     *
     * @var bool
     */
    public $shouldQuit = false;

    /**
     * Indicates if the worker lost its connection.
     *
     * @var bool
     */
    public $lostConnection = false;

    /**
     * Indicates if the worker is paused.
     *
     * @var bool
     */
    public $paused = false;

    /**
     * The queues the worker last observed to be paused.
     *
     * @var array<int, string>
     */
    protected $pausedQueues = [];

    /**
     * The callbacks used to pop jobs from queues.
     *
     * @var callable[]
     */
    protected static $popCallbacks = [];

    /**
     * The custom exit code to be used when memory is exceeded.
     *
     * @var int|null
     */
    public static $memoryExceededExitCode;

    /**
     * The custom exit code to be used when a job times out.
     *
     * @var int|null
     */
    public static $timedOutExitCode;

    /**
     * Indicates if the worker should be killed when a job exceeds its timeout.
     *
     * @var bool
     */
    public static $killOnTimeout = true;

    /**
     * The callback used to kill the worker process.
     *
     * @var (callable(int): mixed)|null
     */
    protected static $killCallback;

    /**
     * Indicates if the worker should report job exceptions.
     *
     * @var bool
     */
    public static $reportJobExceptions = true;

    /**
     * Indicates if the worker should stop when a lost connection is detected.
     *
     * @var bool
     */
    public static $stopOnLostConnection = true;

    /**
     * Indicates if the worker should check for the restart signal in the cache.
     *
     * @var bool
     */
    public static $restartable = true;

    /**
     * Indicates if the worker should check for the paused signal in the cache.
     *
     * @var bool
     */
    public static $pausable = true;

    /**
     * Indicates if the worker should flush sticky database connections between jobs.
     *
     * @var bool
     */
    public static $flushStickyConnections = false;

    /**
     * Create a new queue worker.
     *
     * @param  \Illuminate\Contracts\Queue\Factory  $manager
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @param  \Illuminate\Contracts\Debug\ExceptionHandler  $exceptions
     * @param  callable  $isDownForMaintenance
     * @param  callable|null  $resetScope
     */
    public function __construct(
        QueueManager $manager,
        Dispatcher $events,
        ExceptionHandler $exceptions,
        callable $isDownForMaintenance,
        ?callable $resetScope = null,
    ) {}

    /**
     * Listen to the given queue in a loop.
     *
     * @param  string  $connectionName
     * @param  string  $queue
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return int
     */
    public function daemon($connectionName, $queue, WorkerOptions $options)
    {}

    /**
     * Register the worker timeout handler.
     *
     * @param  string|null  $connectionName
     * @param  string|null  $queue
     * @param  \Illuminate\Contracts\Queue\Job|null  $job
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return void
     */
    protected function registerTimeoutHandler($connectionName, $queue, $job, WorkerOptions $options)
    {}

    /**
     * Reset the worker timeout handler.
     *
     * @return void
     */
    protected function resetTimeoutHandler()
    {}

    /**
     * Get the appropriate timeout for the given job.
     *
     * @param  \Illuminate\Contracts\Queue\Job|null  $job
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return int
     */
    protected function timeoutForJob($job, WorkerOptions $options)
    {}

    /**
     * Determine if the daemon should process on this iteration.
     *
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @param  string  $connectionName
     * @param  string  $queue
     * @return bool
     */
    protected function daemonShouldRun(WorkerOptions $options, $connectionName, $queue)
    {}

    /**
     * Pause the worker for the current loop.
     *
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @param  int|null  $lastRestart
     * @param  int|float  $startTime
     * @return array|null
     */
    protected function pauseWorker(WorkerOptions $options, $lastRestart, $startTime = 0)
    {}

    /**
     * Determine the exit code to stop the process if necessary.
     *
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @param  int|null  $lastRestart
     * @param  int|float  $startTime
     * @param  mixed  $job
     * @return array|null
     */
    protected function stopIfNecessary(WorkerOptions $options, $lastRestart, $startTime = 0, $job = null)
    {}

    /**
     * Process the next job on the queue.
     *
     * @param  string  $connectionName
     * @param  string  $queue
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return void
     */
    public function runNextJob($connectionName, $queue, WorkerOptions $options)
    {}

    /**
     * Get the next job from the queue connection.
     *
     * @param  \Illuminate\Contracts\Queue\Queue  $connection
     * @param  string  $queue
     * @return \Illuminate\Contracts\Queue\Job|null
     */
    protected function getNextJob($connection, $queue)
    {}

    /**
     * Determine which of the given queues are currently paused.
     *
     * @param  string  $connectionName
     * @param  array  $queues
     * @return array
     */
    protected function getPausedQueues($connectionName, $queues)
    {}

    /**
     * Raise events for any queues that have been paused or resumed since the last check.
     *
     * @param  string  $connectionName
     * @param  array  $paused
     * @return void
     */
    protected function raisePausedQueueEvents($connectionName, array $paused)
    {}

    /**
     * Process the given job.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  string  $connectionName
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return void
     */
    protected function runJob($job, $connectionName, WorkerOptions $options)
    {}

    /**
     * Stop the worker if we have lost connection to a database.
     *
     * @param  \Throwable  $e
     * @return void
     */
    protected function stopWorkerIfLostConnection($e)
    {}

    /**
     * Process the given job from the queue.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return void
     *
     * @throws \Throwable
     */
    public function process($connectionName, $job, WorkerOptions $options)
    {}

    /**
     * Handle an exception that occurred while the job was running.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @param  \Throwable  $e
     * @return void
     *
     * @throws \Throwable
     */
    protected function handleJobException($connectionName, $job, WorkerOptions $options, Throwable $e)
    {}

    /**
     * Mark the given job as failed if it has exceeded the maximum allowed attempts.
     *
     * This will likely be because the job previously exceeded a timeout.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  int  $maxTries
     * @return void
     *
     * @throws \Throwable
     */
    protected function markJobAsFailedIfAlreadyExceedsMaxAttempts($connectionName, $job, $maxTries)
    {}

    /**
     * Mark the given job as failed if it has exceeded the maximum allowed exceptions.
     *
     * This will likely be because the worker previously died while processing the job.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @return void
     *
     * @throws \Throwable
     */
    protected function markJobAsFailedIfAlreadyExceedsMaxExceptions($connectionName, $job)
    {}

    /**
     * Mark the given job as failed if it has exceeded the maximum allowed attempts.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  int  $maxTries
     * @param  \Throwable  $e
     * @return void
     */
    protected function markJobAsFailedIfWillExceedMaxAttempts($connectionName, $job, $maxTries, Throwable $e)
    {}

    /**
     * Mark the given job as failed if it has exceeded the maximum allowed attempts.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable  $e
     * @return void
     */
    protected function markJobAsFailedIfWillExceedMaxExceptions($connectionName, $job, Throwable $e)
    {}

    /**
     * Mark the given job as failed if the exception handler determines it should not be retried.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable  $e
     * @return void
     */
    protected function markJobAsFailedIfItShouldntBeRetried($connectionName, $job, Throwable $e)
    {}

    /**
     * Mark the given job as failed if it should fail on timeouts.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable  $e
     * @return void
     */
    protected function markJobAsFailedIfItShouldFailOnTimeout($connectionName, $job, Throwable $e)
    {}

    /**
     * Mark the given job as failed and raise the relevant event.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable  $e
     * @return void
     */
    protected function failJob($job, Throwable $e)
    {}

    /**
     * Calculate the backoff for the given job.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return int
     */
    protected function calculateBackoff($job, WorkerOptions $options)
    {}

    /**
     * Raise an event indicating the worker is starting.
     *
     * @param  string  $connectionName
     * @param  string  $queue
     * @param  \Illuminate\Queue\WorkerOptions  $options
     * @return void
     */
    protected function raiseWorkerStartingEvent($connectionName, $queue, $options)
    {}

    /**
     * Raise an event indicating a job is being popped from the queue.
     *
     * @param  string  $connectionName
     * @param  string|null  $queue
     * @return void
     */
    protected function raiseBeforeJobPopEvent($connectionName, $queue = null)
    {}

    /**
     * Raise an event indicating a job has been popped from the queue.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job|null  $job
     * @return void
     */
    protected function raiseAfterJobPopEvent($connectionName, $job)
    {}

    /**
     * Raise an event indicating a job is being processed.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @return void
     */
    protected function raiseBeforeJobEvent($connectionName, $job)
    {}

    /**
     * Raise an event indicating a job has been processed.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  float|null  $duration
     * @return void
     */
    protected function raiseAfterJobEvent($connectionName, $job, $duration = null)
    {}

    /**
     * Raise the exception occurred queue job event.
     *
     * @param  string  $connectionName
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable  $e
     * @return void
     */
    protected function raiseExceptionOccurredJobEvent($connectionName, $job, Throwable $e)
    {}

    /**
     * Determine if the queue worker should restart.
     *
     * @param  int|null  $lastRestart
     * @return bool
     */
    protected function queueShouldRestart($lastRestart)
    {}

    /**
     * Get the last queue restart timestamp, or null.
     *
     * @return int|null
     */
    protected function getTimestampOfLastQueueRestart()
    {}

    /**
     * Enable async signals for the process.
     *
     * @param  string|null  $connectionName
     * @param  string|null  $queue
     * @param  \Illuminate\Queue\WorkerOptions|null  $options
     * @return void
     */
    protected function listenForSignals($connectionName = null, $queue = null, ?WorkerOptions $options = null)
    {}

    /**
     * Passes the signal to the running job.
     *
     * @param  int  $signal
     * @return void
     */
    protected function notifyJobOfSignal(int $signal): void
    {}

    /**
     * Determine if "async" signals are supported.
     *
     * @return bool
     */
    protected function supportsAsyncSignals()
    {}

    /**
     * Determine if the memory limit has been exceeded.
     *
     * @param  int|string  $memoryLimit
     * @return bool
     */
    public function memoryExceeded($memoryLimit)
    {}

    /**
     * Stop listening and bail out of the script.
     *
     * @param  int  $status
     * @param  WorkerOptions|null  $options
     * @param  WorkerStopReason|null  $reason
     * @param  string|null  $connectionName
     * @param  string|null  $queue
     * @return int
     */
    public function stop($status = 0, $options = null, $reason = null, $connectionName = null, $queue = null)
    {}

    /**
     * Kill the process.
     *
     * @param  int  $status
     * @param  \Illuminate\Queue\WorkerOptions|null  $options
     * @param  \Illuminate\Queue\WorkerStopReason|null  $reason
     * @param  string|null  $connectionName
     * @param  string|null  $queue
     * @return never
     */
    public function kill($status = 0, $options = null, $reason = null, $connectionName = null, $queue = null)
    {}

    /**
     * Create an instance of MaxAttemptsExceededException.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @return \Illuminate\Queue\MaxAttemptsExceededException
     */
    protected function maxAttemptsExceededException($job)
    {}

    /**
     * Create an instance of TimeoutExceededException.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @return \Illuminate\Queue\TimeoutExceededException
     */
    protected function timeoutExceededException($job)
    {}

    /**
     * Sleep the script for a given number of seconds.
     *
     * @param  int|float  $seconds
     * @return void
     */
    public function sleep($seconds)
    {}

    /**
     * Set the cache repository implementation.
     *
     * @param  \Illuminate\Contracts\Cache\Repository  $cache
     * @return $this
     */
    public function setCache(CacheContract $cache)
    {}

    /**
     * Set the name of the worker.
     *
     * @param  string  $name
     * @return $this
     */
    public function setName($name)
    {}

    /**
     * Register a callback to be executed to pick jobs.
     *
     * @param  string  $workerName
     * @param  callable  $callback
     * @return void
     */
    public static function popUsing($workerName, $callback)
    {}

    /**
     * Register a callback to be used to kill the worker process.
     *
     * @param  (callable(int): mixed)|null  $callback
     * @return void
     */
    public static function killUsing($callback)
    {}

    /**
     * Get the queue manager instance.
     *
     * @return \Illuminate\Contracts\Queue\Factory
     */
    public function getManager()
    {}

    /**
     * Set the queue manager instance.
     *
     * @param  \Illuminate\Contracts\Queue\Factory  $manager
     * @return void
     */
    public function setManager(QueueManager $manager)
    {}

    /**
     * Get the current high-resolution timestamp.
     *
     * @return float
     */
    protected function currentTime()
    {}

    /**
     * Get the current memory usage in MB.
     *
     * @return int|float
     */
    protected function currentMemoryUsage()
    {}
}
