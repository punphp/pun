<?php

namespace Illuminate\Process;

use Illuminate\Contracts\Process\ProcessResult as ProcessResultContract;
use Illuminate\Process\Exceptions\ProcessFailedException;
use Illuminate\Support\Collection;

class FakeProcessResult implements ProcessResultContract
{
    /**
     * The command string.
     *
     * @var string
     */
    protected $command;

    /**
     * The process exit code.
     *
     * @var int
     */
    protected $exitCode;

    /**
     * The process output.
     *
     * @var string
     */
    protected $output = '';

    /**
     * The process error output.
     *
     * @var string
     */
    protected $errorOutput = '';

    /**
     * Create a new process result instance.
     *
     * @param  string  $command
     * @param  int  $exitCode
     * @param  array|string  $output
     * @param  array|string  $errorOutput
     */
    public function __construct(string $command = '', int $exitCode = 0, array|string $output = '', array|string $errorOutput = '')
    {}

    /**
     * Normalize the given output into a string with newlines.
     *
     * @param  array|string  $output
     * @return string
     */
    protected function normalizeOutput(array|string $output)
    {}

    /**
     * Get the original command executed by the process.
     *
     * @return string
     */
    public function command()
    {}

    /**
     * Create a new fake process result with the given command.
     *
     * @param  string  $command
     * @return self
     */
    public function withCommand(string $command)
    {}

    /**
     * Determine if the process was successful.
     *
     * @return bool
     */
    public function successful()
    {}

    /**
     * Determine if the process failed.
     *
     * @return bool
     */
    public function failed()
    {}

    /**
     * Get the exit code of the process.
     *
     * @return int
     */
    public function exitCode()
    {}

    /**
     * Get the standard output of the process.
     *
     * @return string
     */
    public function output()
    {}

    /**
     * Determine if the output contains the given string.
     *
     * @param  string  $output
     * @return bool
     */
    public function seeInOutput(string $output)
    {}

    /**
     * Get the error output of the process.
     *
     * @return string
     */
    public function errorOutput()
    {}

    /**
     * Determine if the error output contains the given string.
     *
     * @param  string  $output
     * @return bool
     */
    public function seeInErrorOutput(string $output)
    {}

    /**
     * Throw an exception if the process failed.
     *
     * @param  callable|null  $callback
     * @return $this
     *
     * @throws \Illuminate\Process\Exceptions\ProcessFailedException
     */
    public function throw(?callable $callback = null)
    {}

    /**
     * Throw an exception if the process failed and the given condition is true.
     *
     * @param  bool  $condition
     * @param  callable|null  $callback
     * @return $this
     *
     * @throws \Throwable
     */
    public function throwIf(bool $condition, ?callable $callback = null)
    {}
}
