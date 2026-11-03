<?php

namespace Illuminate\Log;

use Closure;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Log\Events\MessageLogged;
use Illuminate\Support\Traits\Conditionable;
use Psr\Log\LoggerInterface;
use RuntimeException;

class Logger implements LoggerInterface
{
    use Conditionable;

    /**
     * The underlying logger implementation.
     *
     * @var \Psr\Log\LoggerInterface
     */
    protected $logger;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $dispatcher;

    /**
     * Any context to be added to logs.
     *
     * @var array
     */
    protected $context = [];

    /**
     * Create a new log writer instance.
     *
     * @param  \Psr\Log\LoggerInterface  $logger
     * @param  \Illuminate\Contracts\Events\Dispatcher|null  $dispatcher
     */
    public function __construct(LoggerInterface $logger, ?Dispatcher $dispatcher = null)
    {}

    /**
     * Log an emergency message to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function emergency($message, array $context = []): void
    {}

    /**
     * Log an alert message to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function alert($message, array $context = []): void
    {}

    /**
     * Log a critical message to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function critical($message, array $context = []): void
    {}

    /**
     * Log an error message to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function error($message, array $context = []): void
    {}

    /**
     * Log a warning message to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function warning($message, array $context = []): void
    {}

    /**
     * Log a notice to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function notice($message, array $context = []): void
    {}

    /**
     * Log an informational message to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function info($message, array $context = []): void
    {}

    /**
     * Log a debug message to the logs.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function debug($message, array $context = []): void
    {}

    /**
     * Log a message to the logs.
     *
     * @param  string  $level
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function log($level, $message, array $context = []): void
    {}

    /**
     * Dynamically pass log calls into the writer.
     *
     * @param  string  $level
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    public function write($level, $message, array $context = []): void
    {}

    /**
     * Write a message to the log.
     *
     * @param  string  $level
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @param  array  $context
     * @return void
     */
    protected function writeLog($level, $message, $context): void
    {}

    /**
     * Add context to all future logs.
     *
     * @param  array  $context
     * @return $this
     */
    public function withContext(array $context = [])
    {}

    /**
     * Flush the log context on all currently resolved channels.
     *
     * @param  string[]|null  $keys
     * @return $this
     */
    public function withoutContext(?array $keys = null)
    {}

    /**
     * Register a new callback handler for when a log event is triggered.
     *
     * @param  \Closure  $callback
     * @return void
     *
     * @throws \RuntimeException
     */
    public function listen(Closure $callback)
    {}

    /**
     * Fires a log event.
     *
     * @param  string  $level
     * @param  string  $message
     * @param  array  $context
     * @return void
     */
    protected function fireLogEvent($level, $message, array $context = [])
    {}

    /**
     * Format the parameters for the logger.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\Illuminate\Contracts\Support\Jsonable|\Illuminate\Support\Stringable|array|string  $message
     * @return string
     */
    protected function formatMessage($message)
    {}

    /**
     * Get the underlying logger implementation.
     *
     * @return \Psr\Log\LoggerInterface
     */
    public function getLogger()
    {}

    /**
     * Get the event dispatcher instance.
     *
     * @return \Illuminate\Contracts\Events\Dispatcher|null
     */
    public function getEventDispatcher()
    {}

    /**
     * Set the event dispatcher instance.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $dispatcher
     * @return void
     */
    public function setEventDispatcher(Dispatcher $dispatcher)
    {}

    /**
     * Dynamically proxy method calls to the underlying logger.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
