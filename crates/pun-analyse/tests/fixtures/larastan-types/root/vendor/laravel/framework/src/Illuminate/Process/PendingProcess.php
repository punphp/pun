<?php

namespace Illuminate\Process;

use Carbon\CarbonInterval;
use Closure;
use Illuminate\Process\Exceptions\ProcessTimedOutException;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use LogicException;
use RuntimeException;
use Symfony\Component\Process\Exception\ProcessTimedOutException as SymfonyTimeoutException;
use Symfony\Component\Process\Process;

class PendingProcess
{
    use Conditionable;

    /**
     * The process factory instance.
     *
     * @var \Illuminate\Process\Factory
     */
    protected $factory;

    /**
     * The command to invoke the process.
     *
     * @var array<array-key, string>|string|null
     */
    public $command;

    /**
     * The working directory of the process.
     *
     * @var string|null
     */
    public $path;

    /**
     * The maximum number of seconds the process may run.
     *
     * @var int|float|null
     */
    public $timeout = 60;

    /**
     * The maximum number of seconds the process may go without returning output.
     *
     * @var int|float|null
     */
    public $idleTimeout;

    /**
     * The additional environment variables for the process.
     *
     * @var array
     */
    public $environment = [];

    /**
     * The standard input data that should be piped into the command.
     *
     * @var string|int|float|bool|resource|\Traversable|null
     */
    public $input;

    /**
     * Indicates whether output should be disabled for the process.
     *
     * @var bool
     */
    public $quietly = false;

    /**
     * Indicates if TTY mode should be enabled.
     *
     * @var bool
     */
    public $tty = false;

    /**
     * The options that will be passed to "proc_open".
     *
     * @var array
     */
    public $options = [];

    /**
     * The registered fake handler callbacks.
     *
     * @var array
     */
    protected $fakeHandlers = [];

    /**
     * Create a new pending process instance.
     *
     * @param  \Illuminate\Process\Factory  $factory
     */
    public function __construct(Factory $factory)
    {}

    /**
     * Specify the command that will invoke the process.
     *
     * @param  array<array-key, string>|string  $command
     * @return $this
     */
    public function command(array|string $command)
    {}

    /**
     * Specify the working directory of the process.
     *
     * @param  string  $path
     * @return $this
     */
    public function path(string $path)
    {}

    /**
     * Specify the maximum number of seconds the process may run.
     *
     * @param  CarbonInterval|int  $timeout
     * @return $this
     */
    public function timeout(CarbonInterval|int $timeout)
    {}

    /**
     * Specify the maximum number of seconds a process may go without returning output.
     *
     * @param  CarbonInterval|int  $timeout
     * @return $this
     */
    public function idleTimeout(CarbonInterval|int $timeout)
    {}

    /**
     * Indicate that the process may run forever without timing out.
     *
     * @return $this
     */
    public function forever()
    {}

    /**
     * Set the additional environment variables for the process.
     *
     * @param  array  $environment
     * @return $this
     */
    public function env(array $environment)
    {}

    /**
     * Set the standard input that should be provided when invoking the process.
     *
     * @param  \Traversable|resource|string|int|float|bool|null  $input
     * @return $this
     */
    public function input($input)
    {}

    /**
     * Disable output for the process.
     *
     * @return $this
     */
    public function quietly()
    {}

    /**
     * Enable TTY mode for the process.
     *
     * @param  bool  $tty
     * @return $this
     */
    public function tty(bool $tty = true)
    {}

    /**
     * Set the "proc_open" options that should be used when invoking the process.
     *
     * @param  array  $options
     * @return $this
     */
    public function options(array $options)
    {}

    /**
     * Run the process.
     *
     * @param  array<array-key, string>|string|null  $command
     * @param  callable|null  $output
     * @return \Illuminate\Contracts\Process\ProcessResult
     *
     * @throws \Illuminate\Process\Exceptions\ProcessTimedOutException
     * @throws \RuntimeException
     */
    public function run(array|string|null $command = null, ?callable $output = null)
    {}

    /**
     * Start the process in the background.
     *
     * @param  array<array-key, string>|string|null  $command
     * @param  callable|null  $output
     * @return \Illuminate\Process\InvokedProcess|\Illuminate\Process\FakeInvokedProcess
     *
     * @throws \RuntimeException
     */
    public function start(array|string|null $command = null, ?callable $output = null)
    {}

    /**
     * Get a Symfony Process instance from the current pending command.
     *
     * @param  array<array-key, string>|string|null  $command
     * @return \Symfony\Component\Process\Process
     */
    protected function toSymfonyProcess(array|string|null $command)
    {}

    /**
     * Determine whether TTY is supported on the current operating system.
     *
     * @return bool
     */
    public function supportsTty()
    {}

    /**
     * Specify the fake process result handlers for the pending process.
     *
     * @param  array  $fakeHandlers
     * @return $this
     */
    public function withFakeHandlers(array $fakeHandlers)
    {}

    /**
     * Get the fake handler for the given command, if applicable.
     *
     * @param  string  $command
     * @return \Closure|null
     */
    protected function fakeFor(string $command)
    {}

    /**
     * Resolve the given fake handler for a synchronous process.
     *
     * @param  string  $command
     * @param  \Closure  $fake
     * @return mixed
     *
     * @throws \LogicException
     * @throws \Throwable
     */
    protected function resolveSynchronousFake(string $command, Closure $fake)
    {}

    /**
     * Resolve the given fake handler for an asynchronous process.
     *
     * @param  string  $command
     * @param  callable|null  $output
     * @param  \Closure  $fake
     * @return \Illuminate\Process\FakeInvokedProcess
     *
     * @throws \LogicException
     * @throws \Throwable
     */
    protected function resolveAsynchronousFake(string $command, ?callable $output, Closure $fake)
    {}
}
