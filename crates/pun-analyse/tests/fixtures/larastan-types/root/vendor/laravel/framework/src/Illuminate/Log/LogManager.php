<?php

namespace Illuminate\Log;

use Closure;
use Illuminate\Contracts\Log\ContextLogProcessor;
use Illuminate\Support\Collection;
use Illuminate\Support\RebindsCallbacksToSelf;
use Illuminate\Support\Str;
use InvalidArgumentException;
use Monolog\Formatter\LineFormatter;
use Monolog\Handler\ErrorLogHandler;
use Monolog\Handler\FingersCrossedHandler;
use Monolog\Handler\FormattableHandlerInterface;
use Monolog\Handler\HandlerInterface;
use Monolog\Handler\RotatingFileHandler;
use Monolog\Handler\SlackWebhookHandler;
use Monolog\Handler\StreamHandler;
use Monolog\Handler\SyslogHandler;
use Monolog\Handler\WhatFailureGroupHandler;
use Monolog\Logger as Monolog;
use Monolog\Processor\ProcessorInterface;
use Monolog\Processor\PsrLogMessageProcessor;
use Psr\Log\LoggerInterface;
use ReflectionException;
use RuntimeException;
use Throwable;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Log\Logger
 */
class LogManager implements LoggerInterface
{
    use ParsesLogConfiguration, RebindsCallbacksToSelf;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The array of resolved channels.
     *
     * @var array
     */
    protected $channels = [];

    /**
     * The context shared across channels and stacks.
     *
     * @var array
     */
    protected $sharedContext = [];

    /**
     * The registered custom driver creators.
     *
     * @var array
     */
    protected $customCreators = [];

    /**
     * The standard date format to use when writing logs.
     *
     * @var string
     */
    protected $dateFormat = 'Y-m-d H:i:s';

    /**
     * Create a new Log manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Build an on-demand log channel.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    public function build(array $config)
    {}

    /**
     * Create a new, on-demand aggregate logger instance.
     *
     * @param  array  $channels
     * @param  string|null  $channel
     * @return \Psr\Log\LoggerInterface
     */
    public function stack(array $channels, $channel = null)
    {}

    /**
     * Get a log channel instance.
     *
     * @param  \UnitEnum|string|null  $channel
     * @return \Psr\Log\LoggerInterface
     */
    public function channel($channel = null)
    {}

    /**
     * Get a log driver instance.
     *
     * @param  \UnitEnum|string|null  $driver
     * @return \Psr\Log\LoggerInterface
     */
    public function driver($driver = null)
    {}

    /**
     * Attempt to get the log from the local cache.
     *
     * @param  string|null  $name
     * @param  array|null  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function get($name, ?array $config = null)
    {}

    /**
     * Apply the configured taps for the logger.
     *
     * @param  string|null  $name
     * @param  \Illuminate\Log\Logger  $logger
     * @return \Illuminate\Log\Logger
     */
    protected function tap($name, Logger $logger)
    {}

    /**
     * Parse the given tap class string into a class name and arguments string.
     *
     * @param  string  $tap
     * @return array
     */
    protected function parseTap($tap)
    {}

    /**
     * Create an emergency log handler to avoid white screens of death.
     *
     * @return \Psr\Log\LoggerInterface
     */
    protected function createEmergencyLogger()
    {}

    /**
     * Resolve the given log instance by name.
     *
     * @param  string|null  $name
     * @param  array|null  $config
     * @return \Psr\Log\LoggerInterface
     *
     * @throws \InvalidArgumentException
     */
    protected function resolve($name, ?array $config = null)
    {}

    /**
     * Call a custom driver creator.
     *
     * @param  array  $config
     * @return mixed
     */
    protected function callCustomCreator(array $config)
    {}

    /**
     * Create a custom log driver instance.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createCustomDriver(array $config)
    {}

    /**
     * Create an aggregate log driver instance.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createStackDriver(array $config)
    {}

    /**
     * Create an instance of the single file log driver.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createSingleDriver(array $config)
    {}

    /**
     * Create an instance of the daily file log driver.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createDailyDriver(array $config)
    {}

    /**
     * Create an instance of the monthly file log driver.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createMonthlyDriver(array $config)
    {}

    /**
     * Create an instance of a rotating file log driver.
     *
     * @param  array  $config
     * @param  string  $dateFormat
     * @param  int  $maxFiles
     * @return \Psr\Log\LoggerInterface
     */
    protected function createRotatingDriver(array $config, string $dateFormat, $maxFiles)
    {}

    /**
     * Create an instance of the Slack log driver.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createSlackDriver(array $config)
    {}

    /**
     * Create an instance of the syslog log driver.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createSyslogDriver(array $config)
    {}

    /**
     * Create an instance of the "error log" log driver.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     */
    protected function createErrorlogDriver(array $config)
    {}

    /**
     * Create an instance of any handler available in Monolog.
     *
     * @param  array  $config
     * @return \Psr\Log\LoggerInterface
     *
     * @throws \InvalidArgumentException
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function createMonologDriver(array $config)
    {}

    /**
     * Prepare the handlers for usage by Monolog.
     *
     * @param  array  $handlers
     * @return array
     */
    protected function prepareHandlers(array $handlers)
    {}

    /**
     * Prepare the handler for usage by Monolog.
     *
     * @param  \Monolog\Handler\HandlerInterface  $handler
     * @param  array  $config
     * @return \Monolog\Handler\HandlerInterface
     */
    protected function prepareHandler(HandlerInterface $handler, array $config = [])
    {}

    /**
     * Get a Monolog formatter instance.
     *
     * @return \Monolog\Formatter\FormatterInterface
     */
    protected function formatter()
    {}

    /**
     * Share context across channels and stacks.
     *
     * @param  array  $context
     * @return $this
     */
    public function shareContext(array $context)
    {}

    /**
     * The context shared across channels and stacks.
     *
     * @return array
     */
    public function sharedContext()
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
     * Flush the shared context.
     *
     * @return $this
     */
    public function flushSharedContext()
    {}

    /**
     * Get fallback log channel name.
     *
     * @return string
     */
    protected function getFallbackChannelName()
    {}

    /**
     * Get the log connection configuration.
     *
     * @param  string|null  $name
     * @return array|null
     */
    protected function configurationFor($name)
    {}

    /**
     * Get the default log driver name.
     *
     * @return string|null
     */
    public function getDefaultDriver()
    {}

    /**
     * Set the default log driver name.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultDriver($name)
    {}

    /**
     * Register a custom driver creator Closure.
     *
     * @param  string  $driver
     * @param  \Closure  $callback
     *
     * @param-closure-this  $this  $callback
     *
     * @return $this
     */
    public function extend($driver, Closure $callback)
    {}

    /**
     * Unset the given channel instance.
     *
     * @param  \UnitEnum|string|null  $driver
     * @return void
     */
    public function forgetChannel($driver = null)
    {}

    /**
     * Parse the driver name.
     *
     * @param  string|null  $driver
     * @return string|null
     */
    protected function parseDriver($driver)
    {}

    /**
     * Get all of the resolved log channels.
     *
     * @return array
     */
    public function getChannels()
    {}

    /**
     * System is unusable.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function emergency($message, array $context = []): void
    {}

    /**
     * Action must be taken immediately.
     *
     * Example: Entire website down, database unavailable, etc. This should
     * trigger the SMS alerts and wake you up.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function alert($message, array $context = []): void
    {}

    /**
     * Critical conditions.
     *
     * Example: Application component unavailable, unexpected exception.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function critical($message, array $context = []): void
    {}

    /**
     * Runtime errors that do not require immediate action but should typically
     * be logged and monitored.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function error($message, array $context = []): void
    {}

    /**
     * Exceptional occurrences that are not errors.
     *
     * Example: Use of deprecated APIs, poor use of an API, undesirable things
     * that are not necessarily wrong.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function warning($message, array $context = []): void
    {}

    /**
     * Normal but significant events.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function notice($message, array $context = []): void
    {}

    /**
     * Interesting events.
     *
     * Example: User logs in, SQL logs.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function info($message, array $context = []): void
    {}

    /**
     * Detailed debug information.
     *
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function debug($message, array $context = []): void
    {}

    /**
     * Logs with an arbitrary level.
     *
     * @param  mixed  $level
     * @param  string|\Stringable  $message
     * @param  array  $context
     * @return void
     */
    public function log($level, $message, array $context = []): void
    {}

    /**
     * Set the application instance used by the manager.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return $this
     */
    public function setApplication($app)
    {}

    /**
     * Dynamically call the default driver instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
