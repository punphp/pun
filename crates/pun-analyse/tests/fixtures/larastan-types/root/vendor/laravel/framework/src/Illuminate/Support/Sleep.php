<?php

namespace Illuminate\Support;

use Carbon\CarbonInterval;
use Closure;
use DateInterval;
use Illuminate\Support\Traits\Macroable;
use PHPUnit\Framework\Assert as PHPUnit;
use RuntimeException;

class Sleep
{
    use Macroable;

    /**
     * The fake sleep callbacks.
     *
     * @var array
     */
    public static $fakeSleepCallbacks = [];

    /**
     * Keep Carbon's "now" in sync when sleeping.
     *
     * @var bool
     */
    protected static $syncWithCarbon = false;

    /**
     * The total duration to sleep.
     *
     * @var \Carbon\CarbonInterval
     */
    public $duration;

    /**
     * The callback that determines if sleeping should continue.
     *
     * @var \Closure
     */
    public $while;

    /**
     * The pending duration to sleep.
     *
     * @var int|float|null
     */
    protected $pending = null;

    /**
     * Indicates that all sleeping should be faked.
     *
     * @var bool
     */
    protected static $fake = false;

    /**
     * The sequence of sleep durations encountered while faking.
     *
     * @var array<int, \Carbon\CarbonInterval>
     */
    protected static $sequence = [];

    /**
     * Indicates if the instance should sleep.
     *
     * @var bool
     */
    protected $shouldSleep = true;

    /**
     * Indicates if the instance already slept via `then()`.
     *
     * @var bool
     */
    protected $alreadySlept = false;

    /**
     * Create a new class instance.
     *
     * @param  int|float|\DateInterval  $duration
     */
    public function __construct($duration)
    {}

    /**
     * Sleep for the given duration.
     *
     * @param  \DateInterval|int|float  $duration
     * @return static
     */
    public static function for($duration)
    {}

    /**
     * Sleep until the given timestamp.
     *
     * @param  \DateTimeInterface|int|float|numeric-string  $timestamp
     * @return static
     */
    public static function until($timestamp)
    {}

    /**
     * Sleep for the given number of microseconds.
     *
     * @param  int  $duration
     * @return static
     */
    public static function usleep($duration)
    {}

    /**
     * Sleep for the given number of seconds.
     *
     * @param  int|float  $duration
     * @return static
     */
    public static function sleep($duration)
    {}

    /**
     * Sleep for the given duration. Replaces any previously defined duration.
     *
     * @param  \DateInterval|int|float  $duration
     * @return $this
     */
    protected function duration($duration)
    {}

    /**
     * Sleep for the given number of minutes.
     *
     * @return $this
     */
    public function minutes()
    {}

    /**
     * Sleep for one minute.
     *
     * @return $this
     */
    public function minute()
    {}

    /**
     * Sleep for the given number of seconds.
     *
     * @return $this
     */
    public function seconds()
    {}

    /**
     * Sleep for one second.
     *
     * @return $this
     */
    public function second()
    {}

    /**
     * Sleep for the given number of milliseconds.
     *
     * @return $this
     */
    public function milliseconds()
    {}

    /**
     * Sleep for one millisecond.
     *
     * @return $this
     */
    public function millisecond()
    {}

    /**
     * Sleep for the given number of microseconds.
     *
     * @return $this
     */
    public function microseconds()
    {}

    /**
     * Sleep for one microsecond.
     *
     * @return $this
     */
    public function microsecond()
    {}

    /**
     * Add additional time to sleep for.
     *
     * @param  int|float  $duration
     * @return $this
     */
    public function and($duration)
    {}

    /**
     * Sleep while a given callback returns "true".
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function while(Closure $callback)
    {}

    /**
     * Specify a callback that should be executed after sleeping.
     *
     * @param  callable  $then
     * @return mixed
     */
    public function then(callable $then)
    {}

    /**
     * Handle the object's destruction.
     *
     * @return void
     */
    public function __destruct()
    {}

    /**
     * Handle the object's destruction.
     *
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function goodnight()
    {}

    /**
     * Resolve the pending duration.
     *
     * @return int|float
     *
     * @throws \RuntimeException
     */
    protected function pullPending()
    {}

    /**
     * Stay awake and capture any attempts to sleep.
     *
     * @param  bool  $value
     * @param  bool  $syncWithCarbon
     * @return void
     */
    public static function fake($value = true, $syncWithCarbon = false)
    {}

    /**
     * Assert a given amount of sleeping occurred a specific number of times.
     *
     * @param  \Closure  $expected
     * @param  int  $times
     * @return void
     */
    public static function assertSlept($expected, $times = 1)
    {}

    /**
     * Assert sleeping occurred a given number of times.
     *
     * @param  int  $expected
     * @return void
     */
    public static function assertSleptTimes($expected)
    {}

    /**
     * Assert the given sleep sequence was encountered.
     *
     * @param  array  $sequence
     * @return void
     */
    public static function assertSequence($sequence)
    {}

    /**
     * Assert that no sleeping occurred.
     *
     * @return void
     */
    public static function assertNeverSlept()
    {}

    /**
     * Assert that no sleeping occurred.
     *
     * @return void
     */
    public static function assertInsomniac()
    {}

    /**
     * Indicate that the instance should not sleep.
     *
     * @return $this
     */
    protected function shouldNotSleep()
    {}

    /**
     * Only sleep when the given condition is true.
     *
     * @param  (\Closure($this): bool)|bool  $condition
     * @return $this
     */
    public function when($condition)
    {}

    /**
     * Don't sleep when the given condition is true.
     *
     * @param  (\Closure($this): bool)|bool  $condition
     * @return $this
     */
    public function unless($condition)
    {}

    /**
     * Specify a callback that should be invoked when faking sleep within a test.
     *
     * @param  callable  $callback
     * @return void
     */
    public static function whenFakingSleep($callback)
    {}

    /**
     * Indicate that Carbon's "now" should be kept in sync when sleeping.
     *
     * @return void
     */
    public static function syncWithCarbon($value = true)
    {}
}
