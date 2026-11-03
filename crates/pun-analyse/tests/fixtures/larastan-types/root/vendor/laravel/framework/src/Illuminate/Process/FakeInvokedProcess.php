<?php

namespace Illuminate\Process;

use Illuminate\Contracts\Process\InvokedProcess as InvokedProcessContract;

class FakeInvokedProcess implements InvokedProcessContract
{
    /**
     * The command being faked.
     *
     * @var string
     */
    protected $command;

    /**
     * The underlying process description.
     *
     * @var \Illuminate\Process\FakeProcessDescription
     */
    protected $process;

    /**
     * The signals that have been received.
     *
     * @var array
     */
    protected $receivedSignals = [];

    /**
     * The number of times the process should indicate that it is "running".
     *
     * @var int|null
     */
    protected $remainingRunIterations;

    /**
     * Indicates whether the process has been stopped.
     *
     * @var bool
     */
    protected $stopped = false;

    /**
     * The general output handler callback.
     *
     * @var callable|null
     */
    protected $outputHandler;

    /**
     * The current output's index.
     *
     * @var int
     */
    protected $nextOutputIndex = 0;

    /**
     * The current error output's index.
     *
     * @var int
     */
    protected $nextErrorOutputIndex = 0;

    /**
     * Create a new invoked process instance.
     *
     * @param  string  $command
     * @param  \Illuminate\Process\FakeProcessDescription  $process
     */
    public function __construct(string $command, FakeProcessDescription $process)
    {}

    /**
     * Get the process ID if the process is still running.
     *
     * @return int|null
     */
    public function id()
    {}

    /**
     * Get the command line for the process.
     *
     * @return string
     */
    public function command()
    {}

    /**
     * Send a signal to the process.
     *
     * @param  int  $signal
     * @return $this
     */
    public function signal(int $signal)
    {}

    /**
     * Determine if the process has received the given signal.
     *
     * @param  int  $signal
     * @return bool
     */
    public function hasReceivedSignal(int $signal)
    {}

    /**
     * Determine if the process is still running.
     *
     * @return bool
     */
    public function running()
    {}

    /**
     * Invoke the asynchronous output handler with the next single line of output if necessary.
     *
     * @return array|false
     */
    protected function invokeOutputHandlerWithNextLineOfOutput()
    {}

    /**
     * Get the standard output for the process.
     *
     * @return string
     */
    public function output()
    {}

    /**
     * Get the error output for the process.
     *
     * @return string
     */
    public function errorOutput()
    {}

    /**
     * Get the latest standard output for the process.
     *
     * @return string
     */
    public function latestOutput()
    {}

    /**
     * Get the latest error output for the process.
     *
     * @return string
     */
    public function latestErrorOutput()
    {}

    /**
     * Ensure that the process has not timed out.
     *
     * @return void
     */
    public function ensureNotTimedOut()
    {}

    /**
     * Wait for the process to finish.
     *
     * @param  callable|null  $output
     * @return \Illuminate\Contracts\Process\ProcessResult
     */
    public function wait(?callable $output = null)
    {}

    /**
     * Wait until the given callback returns true.
     *
     * @param  callable|null  $output
     * @return \Illuminate\Contracts\Process\ProcessResult
     */
    public function waitUntil(?callable $output = null)
    {}

    /**
     * Stop the process if it is still running.
     *
     * @param  float  $timeout
     * @param  int|null  $signal
     * @return int|null
     */
    public function stop(float $timeout = 10, ?int $signal = null)
    {}

    /**
     * Get the ultimate process result that will be returned by this "process".
     *
     * @return \Illuminate\Contracts\Process\ProcessResult
     */
    public function predictProcessResult()
    {}

    /**
     * Set the general output handler for the fake invoked process.
     *
     * @param  callable|null  $outputHandler
     * @return $this
     */
    public function withOutputHandler(?callable $outputHandler)
    {}
}
