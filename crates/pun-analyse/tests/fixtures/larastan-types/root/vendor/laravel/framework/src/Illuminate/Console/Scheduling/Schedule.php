<?php

namespace Illuminate\Console\Scheduling;

use BadMethodCallException;
use Closure;
use DateTimeInterface;
use Illuminate\Bus\UniqueLock;
use Illuminate\Console\Application;
use Illuminate\Container\Container;
use Illuminate\Contracts\Bus\Dispatcher;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Contracts\Container\BindingResolutionException;
use Illuminate\Contracts\Queue\ShouldBeUnique;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Queue\CallQueuedClosure;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\ProcessUtils;
use Illuminate\Support\Traits\Macroable;
use RuntimeException;
use Symfony\Component\Console\Command\Command as SymfonyCommand;
use Throwable;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Console\Scheduling\PendingEventAttributes
 */
class Schedule
{
    use Macroable {
        __call as macroCall;
    }

    const SUNDAY = 0;

    const MONDAY = 1;

    const TUESDAY = 2;

    const WEDNESDAY = 3;

    const THURSDAY = 4;

    const FRIDAY = 5;

    const SATURDAY = 6;

    /**
     * All of the events on the schedule.
     *
     * @var \Illuminate\Console\Scheduling\Event[]
     */
    protected $events = [];

    /**
     * The event mutex implementation.
     *
     * @var \Illuminate\Console\Scheduling\EventMutex
     */
    protected $eventMutex;

    /**
     * The scheduling mutex implementation.
     *
     * @var \Illuminate\Console\Scheduling\SchedulingMutex
     */
    protected $schedulingMutex;

    /**
     * The timezone the date should be evaluated on.
     *
     * @var \DateTimeZone|string|null
     */
    protected $timezone;

    /**
     * The job dispatcher implementation.
     *
     * @var \Illuminate\Contracts\Bus\Dispatcher
     */
    protected $dispatcher;

    /**
     * The cache of mutex results.
     *
     * @var array<string, bool>
     */
    protected $mutexCache = [];

    /**
     * The attributes to pass to the event.
     *
     * @var \Illuminate\Console\Scheduling\PendingEventAttributes|null
     */
    protected $attributes;

    /**
     * The schedule group attributes stack.
     *
     * @var array<int, PendingEventAttributes>
     */
    protected array $groupStack = [];

    /**
     * Indicates if the schedule should check for the paused signal in the cache.
     *
     * @var bool
     */
    public static $pausable = true;

    /**
     * Indicates if the schedule should check for the interrupt signal in the cache.
     *
     * @var bool
     */
    public static $interruptible = true;

    /**
     * Indicates if every event should only run on one server for each cron expression.
     *
     * @var bool
     */
    public static $alwaysOnOneServer = false;

    /**
     * Create a new schedule instance.
     *
     * @param  \DateTimeZone|string|null  $timezone
     *
     * @throws \RuntimeException
     */
    public function __construct($timezone = null)
    {}

    /**
     * Add a new callback event to the schedule.
     *
     * @param  string|callable  $callback
     * @param  array  $parameters
     * @return \Illuminate\Console\Scheduling\CallbackEvent
     */
    public function call($callback, array $parameters = [])
    {}

    /**
     * Add a new Artisan command event to the schedule.
     *
     * @param  \Symfony\Component\Console\Command\Command|string  $command
     * @param  array  $parameters
     * @return \Illuminate\Console\Scheduling\Event
     */
    public function command($command, array $parameters = [])
    {}

    /**
     * Add a new job callback event to the schedule.
     *
     * @param  object|string  $job
     * @param  \UnitEnum|string|null  $queue
     * @param  \UnitEnum|string|null  $connection
     * @return \Illuminate\Console\Scheduling\CallbackEvent
     */
    public function job($job, $queue = null, $connection = null)
    {}

    /**
     * Dispatch the given job to the queue.
     *
     * @param  object  $job
     * @param  string|null  $queue
     * @param  string|null  $connection
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function dispatchToQueue($job, $queue, $connection)
    {}

    /**
     * Dispatch the given unique job to the queue.
     *
     * @param  object  $job
     * @param  string|null  $queue
     * @param  string|null  $connection
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function dispatchUniqueJobToQueue($job, $queue, $connection)
    {}

    /**
     * Dispatch the given job right now.
     *
     * @param  object  $job
     * @return void
     */
    protected function dispatchNow($job)
    {}

    /**
     * Add a new command event to the schedule.
     *
     * @param  string  $command
     * @param  array  $parameters
     * @return \Illuminate\Console\Scheduling\Event
     */
    public function exec($command, array $parameters = [])
    {}

    /**
     * Create new schedule group.
     *
     * @param  \Closure  $events
     * @return void
     *
     * @throws \RuntimeException
     */
    public function group(Closure $events)
    {}

    /**
     * Merge the current group attributes with the given event.
     *
     * @param  \Illuminate\Console\Scheduling\Event  $event
     * @return void
     */
    protected function mergePendingAttributes(Event $event)
    {}

    /**
     * Compile parameters for a command.
     *
     * @param  array  $parameters
     * @return string
     */
    protected function compileParameters(array $parameters)
    {}

    /**
     * Compile array input for a command.
     *
     * @param  string|int  $key
     * @param  array  $value
     * @return string
     */
    public function compileArrayInput($key, $value)
    {}

    /**
     * Determine if the server is allowed to run this event.
     *
     * @param  \Illuminate\Console\Scheduling\Event  $event
     * @param  \DateTimeInterface  $time
     * @return bool
     */
    public function serverShouldRun(Event $event, DateTimeInterface $time)
    {}

    /**
     * Get all of the events on the schedule that are due.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return \Illuminate\Support\Collection
     */
    public function dueEvents($app)
    {}

    /**
     * Get all of the events on the schedule.
     *
     * @return \Illuminate\Console\Scheduling\Event[]
     */
    public function events()
    {}

    /**
     * Get all of the events on the schedule which run on any of the provided environments.
     *
     * @param  list<string>  $environments
     * @return \Illuminate\Console\Scheduling\Event[]
     */
    public function eventsForEnvironments(array $environments): array
    {}

    /**
     * Specify the cache store that should be used to store mutexes.
     *
     * @param  \UnitEnum|string  $store
     * @return $this
     */
    public function useCache($store)
    {}

    /**
     * Get the job dispatcher, if available.
     *
     * @return \Illuminate\Contracts\Bus\Dispatcher
     *
     * @throws \RuntimeException
     */
    protected function getDispatcher()
    {}

    /**
     * Indicate that the scheduler should not poll for pause or interrupt signals.
     *
     * This prevents the scheduler from hitting the application cache to determine if it needs to pause or interrupt.
     *
     * @return void
     */
    public static function withoutInterruptionPolling()
    {}

    /**
     * Determine if the schedule has been interrupted since the given time.
     *
     * @param  \DateTimeInterface  $time
     * @return bool
     */
    public function hasBeenInterruptedSince(DateTimeInterface $time)
    {}

    /**
     * Indicate that every event on the schedule should only run on one server for each cron expression.
     *
     * @param  bool  $value
     * @return void
     */
    public static function alwaysOnOneServer($value = true)
    {}

    /**
     * Dynamically handle calls into the schedule instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
