<?php

namespace Illuminate\Process;

use Illuminate\Support\Collection;
use Symfony\Component\Process\Process;

class FakeProcessDescription
{
    /**
     * The process' ID.
     *
     * @var int|null
     */
    public $processId = 1000;

    /**
     * All of the process' output in the order it was described.
     *
     * @var array
     */
    public $output = [];

    /**
     * The process' exit code.
     *
     * @var int
     */
    public $exitCode = 0;

    /**
     * The number of times the process should indicate that it is "running".
     *
     * @var int
     */
    public $runIterations = 0;

    /**
     * Specify the process ID that should be assigned to the process.
     *
     * @param  int  $processId
     * @return $this
     */
    public function id(int $processId)
    {}

    /**
     * Describe a line of standard output.
     *
     * @param  array|string  $output
     * @return $this
     */
    public function output(array|string $output)
    {}

    /**
     * Describe a line of error output.
     *
     * @param  array|string  $output
     * @return $this
     */
    public function errorOutput(array|string $output)
    {}

    /**
     * Replace the entire output buffer with the given string.
     *
     * @param  string  $output
     * @return $this
     */
    public function replaceOutput(string $output)
    {}

    /**
     * Replace the entire error output buffer with the given string.
     *
     * @param  string  $output
     * @return $this
     */
    public function replaceErrorOutput(string $output)
    {}

    /**
     * Specify the process exit code.
     *
     * @param  int  $exitCode
     * @return $this
     */
    public function exitCode(int $exitCode)
    {}

    /**
     * Specify how many times the "isRunning" method should return "true".
     *
     * @param  int  $iterations
     * @return $this
     */
    public function iterations(int $iterations)
    {}

    /**
     * Specify how many times the "isRunning" method should return "true".
     *
     * @param  int  $iterations
     * @return $this
     */
    public function runsFor(int $iterations)
    {}

    /**
     * Turn the fake process description into an actual process.
     *
     * @param  string  $command
     * @return \Symfony\Component\Process\Process
     */
    public function toSymfonyProcess(string $command)
    {}

    /**
     * Convert the process description into a process result.
     *
     * @param  string  $command
     * @return \Illuminate\Contracts\Process\ProcessResult
     */
    public function toProcessResult(string $command)
    {}

    /**
     * Resolve the standard output as a string.
     *
     * @return string
     */
    protected function resolveOutput()
    {}

    /**
     * Resolve the error output as a string.
     *
     * @return string
     */
    protected function resolveErrorOutput()
    {}
}
