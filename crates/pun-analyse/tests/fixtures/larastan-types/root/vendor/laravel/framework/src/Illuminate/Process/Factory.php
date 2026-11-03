<?php

namespace Illuminate\Process;

use Closure;
use Illuminate\Contracts\Process\ProcessResult as ProcessResultContract;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Macroable;
use PHPUnit\Framework\Assert as PHPUnit;

class Factory
{
    use Macroable {
        __call as macroCall;
    }

    /**
     * Indicates if the process factory has faked process handlers.
     *
     * @var bool
     */
    protected $recording = false;

    /**
     * All of the recorded processes.
     *
     * @var array
     */
    protected $recorded = [];

    /**
     * The registered fake handler callbacks.
     *
     * @var array
     */
    protected $fakeHandlers = [];

    /**
     * Indicates that an exception should be thrown if any process is not faked.
     *
     * @var bool
     */
    protected $preventStrayProcesses = false;

    /**
     * Create a new fake process response for testing purposes.
     *
     * @param  array|string  $output
     * @param  array|string  $errorOutput
     * @param  int  $exitCode
     * @return \Illuminate\Process\FakeProcessResult
     */
    public function result(array|string $output = '', array|string $errorOutput = '', int $exitCode = 0)
    {}

    /**
     * Begin describing a fake process lifecycle.
     *
     * @return \Illuminate\Process\FakeProcessDescription
     */
    public function describe()
    {}

    /**
     * Begin describing a fake process sequence.
     *
     * @param  array  $processes
     * @return \Illuminate\Process\FakeProcessSequence
     */
    public function sequence(array $processes = [])
    {}

    /**
     * Indicate that the process factory should fake processes.
     *
     * @param  \Closure|array|null  $callback
     * @return $this
     */
    public function fake(Closure|array|null $callback = null)
    {}

    /**
     * Determine if the process factory has fake process handlers and is recording processes.
     *
     * @return bool
     */
    public function isRecording()
    {}

    /**
     * Record the given process if processes should be recorded.
     *
     * @param  \Illuminate\Process\PendingProcess  $process
     * @param  \Illuminate\Contracts\Process\ProcessResult  $result
     * @return $this
     */
    public function recordIfRecording(PendingProcess $process, ProcessResultContract $result)
    {}

    /**
     * Record the given process.
     *
     * @param  \Illuminate\Process\PendingProcess  $process
     * @param  \Illuminate\Contracts\Process\ProcessResult  $result
     * @return $this
     */
    public function record(PendingProcess $process, ProcessResultContract $result)
    {}

    /**
     * Indicate that an exception should be thrown if any process is not faked.
     *
     * @param  bool  $prevent
     * @return $this
     */
    public function preventStrayProcesses(bool $prevent = true)
    {}

    /**
     * Determine if stray processes are being prevented.
     *
     * @return bool
     */
    public function preventingStrayProcesses()
    {}

    /**
     * Assert that a process was recorded matching a given truth test.
     *
     * @param  \Closure|array<array-key, string>|string  $callback
     * @return $this
     */
    public function assertRan(Closure|array|string $callback)
    {}

    /**
     * Assert that a process was recorded a given number of times matching a given truth test.
     *
     * @param  \Closure|array<array-key, string>|string  $callback
     * @param  int  $times
     * @return $this
     */
    public function assertRanTimes(Closure|array|string $callback, int $times = 1)
    {}

    /**
     * Assert that the given processes were run in the given order.
     *
     * @param  list<\Closure|array<array-key, string>|string>  $callbacks
     * @return $this
     */
    public function assertRanInOrder(array $callbacks)
    {}

    /**
     * Assert how many processes have been recorded.
     *
     * @param  int  $count
     * @return $this
     */
    protected function assertRanCount(int $count)
    {}

    /**
     * Assert that a process was not recorded matching a given truth test.
     *
     * @param  \Closure|array<array-key, string>|string  $callback
     * @return $this
     */
    public function assertNotRan(Closure|array|string $callback)
    {}

    /**
     * Assert that a process was not recorded matching a given truth test.
     *
     * @param  \Closure|array<array-key, string>|string  $callback
     * @return $this
     */
    public function assertDidntRun(Closure|array|string $callback)
    {}

    /**
     * Assert that no processes were recorded.
     *
     * @return $this
     */
    public function assertNothingRan()
    {}

    /**
     * Start defining a pool of processes.
     *
     * @param  callable  $callback
     * @return \Illuminate\Process\Pool
     */
    public function pool(callable $callback)
    {}

    /**
     * Start defining a series of piped processes.
     *
     * @param  callable|array  $callback
     * @return \Illuminate\Contracts\Process\ProcessResult
     */
    public function pipe(callable|array $callback, ?callable $output = null)
    {}

    /**
     * Run a pool of processes and wait for them to finish executing.
     *
     * @param  callable  $callback
     * @param  callable|null  $output
     * @return \Illuminate\Process\ProcessPoolResults
     */
    public function concurrently(callable $callback, ?callable $output = null)
    {}

    /**
     * Create a new pending process associated with this factory.
     *
     * @return \Illuminate\Process\PendingProcess
     */
    public function newPendingProcess()
    {}

    /**
     * Dynamically proxy methods to a new pending process instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
