<?php

namespace Illuminate\Queue;

use Closure;
use Symfony\Component\Process\Process;

use function Illuminate\Support\artisan_binary;
use function Illuminate\Support\php_binary;

class Listener
{
    /**
     * The command working path.
     *
     * @var string
     */
    protected $commandPath;

    /**
     * The environment the workers should run under.
     *
     * @var string
     */
    protected $environment;

    /**
     * The amount of seconds to wait before polling the queue.
     *
     * @var int
     */
    protected $sleep = 3;

    /**
     * The number of times to try a job before logging it failed.
     *
     * @var int
     */
    protected $maxTries = 0;

    /**
     * The output handler callback.
     *
     * @var \Closure|null
     */
    protected $outputHandler;

    /**
     * Create a new queue listener.
     *
     * @param  string  $commandPath
     */
    public function __construct($commandPath)
    {}

    /**
     * Get the PHP binary.
     *
     * @return string
     */
    protected function phpBinary()
    {}

    /**
     * Get the Artisan binary.
     *
     * @return string
     */
    protected function artisanBinary()
    {}

    /**
     * Listen to the given queue connection.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @param  \Illuminate\Queue\ListenerOptions  $options
     * @return void
     */
    public function listen($connection, $queue, ListenerOptions $options)
    {}

    /**
     * Create a new Symfony process for the worker.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @param  \Illuminate\Queue\ListenerOptions  $options
     * @return \Symfony\Component\Process\Process
     */
    public function makeProcess($connection, $queue, ListenerOptions $options)
    {}

    /**
     * Add the environment option to the given command.
     *
     * @param  array  $command
     * @param  \Illuminate\Queue\ListenerOptions  $options
     * @return array
     */
    protected function addEnvironment($command, ListenerOptions $options)
    {}

    /**
     * Create the command with the listener options.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @param  \Illuminate\Queue\ListenerOptions  $options
     * @return array
     */
    protected function createCommand($connection, $queue, ListenerOptions $options)
    {}

    /**
     * Run the given process.
     *
     * @param  \Symfony\Component\Process\Process  $process
     * @param  int  $memory
     * @return void
     */
    public function runProcess(Process $process, $memory)
    {}

    /**
     * Handle output from the worker process.
     *
     * @param  int  $type
     * @param  string  $line
     * @return void
     */
    protected function handleWorkerOutput($type, $line)
    {}

    /**
     * Determine if the memory limit has been exceeded.
     *
     * @param  int  $memoryLimit
     * @return bool
     */
    public function memoryExceeded($memoryLimit)
    {}

    /**
     * Stop listening and bail out of the script.
     *
     * @return never
     */
    public function stop()
    {}

    /**
     * Set the output handler callback.
     *
     * @param  \Closure  $outputHandler
     * @return void
     */
    public function setOutputHandler(Closure $outputHandler)
    {}
}
