<?php

namespace Illuminate\Console\Scheduling;

use Closure;
use Cron\CronExpression;
use GuzzleHttp\Client as HttpClient;
use GuzzleHttp\ClientInterface as HttpClientInterface;
use GuzzleHttp\Exception\TransferException;
use Illuminate\Console\Application;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Contracts\Mail\Mailer;
use Illuminate\Log\Context\Repository;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\ReflectsClosures;
use Illuminate\Support\Traits\Tappable;
use Psr\Http\Client\ClientExceptionInterface;
use Symfony\Component\Process\Process;
use Throwable;

class Event
{
    use Macroable, ManagesAttributes, ManagesFrequencies, ReflectsClosures, Tappable;

    /**
     * The command string.
     *
     * @var string|null
     */
    public $command;

    /**
     * The location that output should be sent to.
     *
     * @var string
     */
    public $output = '/dev/null';

    /**
     * Indicates whether output should be appended.
     *
     * @var bool
     */
    public $shouldAppendOutput = false;

    /**
     * The array of callbacks to be run before the event is started.
     *
     * @var array
     */
    protected $beforeCallbacks = [];

    /**
     * The array of callbacks to be run after the event is finished.
     *
     * @var array
     */
    protected $afterCallbacks = [];

    /**
     * The event mutex implementation.
     *
     * @var \Illuminate\Console\Scheduling\EventMutex
     */
    public $mutex;

    /**
     * The mutex name resolver callback.
     *
     * @var \Closure|null
     */
    public $mutexNameResolver;

    /**
     * The last time the event was checked for eligibility to run.
     *
     * Utilized by sub-minute repeated events.
     *
     * @var \Illuminate\Support\Carbon|null
     */
    protected $lastChecked;

    /**
     * The exit status code of the command.
     *
     * @var int|null
     */
    public $exitCode;

    /**
     * Indicates whether the execution was skipped due to the mutex already being reserved.
     *
     * @var bool
     */
    public $skippedBecauseOverlapping = false;

    /**
     * Create a new event instance.
     *
     * @param  \Illuminate\Console\Scheduling\EventMutex  $mutex
     * @param  string  $command
     * @param  \DateTimeZone|string|null  $timezone
     */
    public function __construct(EventMutex $mutex, $command, $timezone = null)
    {}

    /**
     * Get the default output depending on the OS.
     *
     * @return string
     */
    public function getDefaultOutput()
    {}

    /**
     * Run the given event.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return void
     *
     * @throws \Throwable
     */
    public function run(Container $container)
    {}

    /**
     * Determine if the event should skip because another process is overlapping.
     *
     * @return bool
     */
    public function shouldSkipDueToOverlapping()
    {}

    /**
     * Determine if the event has been configured to repeat multiple times per minute.
     *
     * @return bool
     */
    public function isRepeatable()
    {}

    /**
     * Determine if the event is ready to repeat.
     *
     * @return bool
     */
    public function shouldRepeatNow()
    {}

    /**
     * Run the command process.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return int
     *
     * @throws \Throwable
     */
    protected function start($container)
    {}

    /**
     * Run the command process.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return int
     */
    protected function execute($container)
    {}

    /**
     * Mark the command process as finished and run callbacks/cleanup.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @param  int  $exitCode
     * @return void
     */
    public function finish(Container $container, $exitCode)
    {}

    /**
     * Call all of the "before" callbacks for the event.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return void
     */
    public function callBeforeCallbacks(Container $container)
    {}

    /**
     * Call all of the "after" callbacks for the event.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return void
     */
    public function callAfterCallbacks(Container $container)
    {}

    /**
     * Build the command string.
     *
     * @return string
     */
    public function buildCommand()
    {}

    /**
     * Determine if the given event should run based on the Cron expression.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return bool
     */
    public function isDue($app)
    {}

    /**
     * Determine if the event runs in maintenance mode.
     *
     * @return bool
     */
    public function runsInMaintenanceMode()
    {}

    /**
     * Determine if the event runs when the scheduler is paused.
     *
     * @return bool
     */
    public function runsWhenPaused()
    {}

    /**
     * Determine if the Cron expression passes.
     *
     * @return bool
     */
    protected function expressionPasses()
    {}

    /**
     * Determine if the event runs in the given environment.
     *
     * @param  string  $environment
     * @return bool
     */
    public function runsInEnvironment($environment)
    {}

    /**
     * Determine if the filters pass for the event.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return bool
     */
    public function filtersPass($app)
    {}

    /**
     * Ensure that the output is stored on disk in a log file.
     *
     * @return $this
     */
    public function storeOutput()
    {}

    /**
     * Send the output of the command to a given location.
     *
     * @param  string  $location
     * @param  bool  $append
     * @return $this
     */
    public function sendOutputTo($location, $append = false)
    {}

    /**
     * Append the output of the command to a given location.
     *
     * @param  string  $location
     * @return $this
     */
    public function appendOutputTo($location)
    {}

    /**
     * E-mail the results of the scheduled operation.
     *
     * @param  mixed  $addresses
     * @param  bool  $onlyIfOutputExists
     * @return $this
     *
     * @throws \LogicException
     */
    public function emailOutputTo($addresses, $onlyIfOutputExists = true)
    {}

    /**
     * E-mail the results of the scheduled operation if it produces output.
     *
     * @param  mixed  $addresses
     * @return $this
     *
     * @throws \LogicException
     */
    public function emailWrittenOutputTo($addresses)
    {}

    /**
     * E-mail the results of the scheduled operation if it fails.
     *
     * @param  mixed  $addresses
     * @return $this
     */
    public function emailOutputOnFailure($addresses)
    {}

    /**
     * Ensure that the command output is being captured.
     *
     * @return void
     */
    protected function ensureOutputIsBeingCaptured()
    {}

    /**
     * E-mail the output of the event to the recipients.
     *
     * @param  \Illuminate\Contracts\Mail\Mailer  $mailer
     * @param  array  $addresses
     * @param  bool  $onlyIfOutputExists
     * @return void
     */
    protected function emailOutput(Mailer $mailer, $addresses, $onlyIfOutputExists = true)
    {}

    /**
     * Get the e-mail subject line for output results.
     *
     * @return string
     */
    protected function getEmailSubject()
    {}

    /**
     * Register a callback to ping a given URL before the job runs.
     *
     * @param  string  $url
     * @return $this
     */
    public function pingBefore($url)
    {}

    /**
     * Register a callback to ping a given URL before the job runs if the given condition is true.
     *
     * @param  bool  $value
     * @param  string  $url
     * @return $this
     */
    public function pingBeforeIf($value, $url)
    {}

    /**
     * Register a callback to ping a given URL after the job runs.
     *
     * @param  string  $url
     * @return $this
     */
    public function thenPing($url)
    {}

    /**
     * Register a callback to ping a given URL after the job runs if the given condition is true.
     *
     * @param  bool  $value
     * @param  string  $url
     * @return $this
     */
    public function thenPingIf($value, $url)
    {}

    /**
     * Register a callback to ping a given URL if the operation succeeds.
     *
     * @param  string  $url
     * @return $this
     */
    public function pingOnSuccess($url)
    {}

    /**
     * Register a callback to ping a given URL if the operation succeeds and if the given condition is true.
     *
     * @param  bool  $value
     * @param  string  $url
     * @return $this
     */
    public function pingOnSuccessIf($value, $url)
    {}

    /**
     * Register a callback to ping a given URL if the operation fails.
     *
     * @param  string  $url
     * @return $this
     */
    public function pingOnFailure($url)
    {}

    /**
     * Register a callback to ping a given URL if the operation fails and if the given condition is true.
     *
     * @param  bool  $value
     * @param  string  $url
     * @return $this
     */
    public function pingOnFailureIf($value, $url)
    {}

    /**
     * Get the callback that pings the given URL.
     *
     * @param  string  $url
     * @return \Closure
     */
    protected function pingCallback($url)
    {}

    /**
     * Get the Guzzle HTTP client to use to send pings.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return \GuzzleHttp\ClientInterface
     */
    protected function getHttpClient(Container $container)
    {}

    /**
     * Register a callback to be called before the operation.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function before(Closure $callback)
    {}

    /**
     * Register a callback to be called after the operation.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function after(Closure $callback)
    {}

    /**
     * Register a callback to be called after the operation.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function then(Closure $callback)
    {}

    /**
     * Register a callback that uses the output after the job runs.
     *
     * @param  \Closure  $callback
     * @param  bool  $onlyIfOutputExists
     * @return $this
     */
    public function thenWithOutput(Closure $callback, $onlyIfOutputExists = false)
    {}

    /**
     * Register a callback to be called if the operation succeeds.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function onSuccess(Closure $callback)
    {}

    /**
     * Register a callback that uses the output if the operation succeeds.
     *
     * @param  \Closure  $callback
     * @param  bool  $onlyIfOutputExists
     * @return $this
     */
    public function onSuccessWithOutput(Closure $callback, $onlyIfOutputExists = false)
    {}

    /**
     * Register a callback to be called if the operation fails.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function onFailure(Closure $callback)
    {}

    /**
     * Register a callback that uses the output if the operation fails.
     *
     * @param  \Closure  $callback
     * @param  bool  $onlyIfOutputExists
     * @return $this
     */
    public function onFailureWithOutput(Closure $callback, $onlyIfOutputExists = false)
    {}

    /**
     * Get a callback that provides output.
     *
     * @param  \Closure  $callback
     * @param  bool  $onlyIfOutputExists
     * @return \Closure
     */
    protected function withOutputCallback(Closure $callback, $onlyIfOutputExists = false)
    {}

    /**
     * Call the given event callback.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @param  callable  $callback
     * @param  array<string, mixed>  $parameters
     * @return mixed
     */
    protected function callEventCallback(Container $container, callable $callback, array $parameters = [])
    {}

    /**
     * Get the event parameters for the given callback.
     *
     * @param  \Closure  $callback
     * @return array
     */
    protected function eventParametersForCallback(Closure $callback)
    {}

    /**
     * Get the summary of the event for display.
     *
     * @return string
     */
    public function getSummaryForDisplay()
    {}

    /**
     * Determine the next due date for an event.
     *
     * @param  \DateTimeInterface|string  $currentTime
     * @param  int  $nth
     * @param  bool  $allowCurrentDate
     * @return \Illuminate\Support\Carbon
     */
    public function nextRunDate($currentTime = 'now', $nth = 0, $allowCurrentDate = false)
    {}

    /**
     * Get the Cron expression for the event.
     *
     * @return string
     */
    public function getExpression()
    {}

    /**
     * Set the event mutex implementation to be used.
     *
     * @param  \Illuminate\Console\Scheduling\EventMutex  $mutex
     * @return $this
     */
    public function preventOverlapsUsing(EventMutex $mutex)
    {}

    /**
     * Get the mutex name for the scheduled command.
     *
     * @return string
     */
    public function mutexName()
    {}

    /**
     * Set the mutex name or name resolver callback.
     *
     * @param  \Closure|string  $mutexName
     * @return $this
     */
    public function createMutexNameUsing(Closure|string $mutexName)
    {}

    /**
     * Ensure the mutex is released if the process receives a termination signal.
     *
     * @return void
     */
    protected function ensureMutexIsReleasedOnSignal()
    {}

    /**
     * Delete the mutex for the event.
     *
     * @return void
     */
    protected function removeMutex()
    {}

    /**
     * Format the given command string with a normalized PHP binary path.
     *
     * @param  string  $command
     * @return string
     */
    public static function normalizeCommand($command)
    {}
}
