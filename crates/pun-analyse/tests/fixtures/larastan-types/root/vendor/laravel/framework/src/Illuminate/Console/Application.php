<?php

namespace Illuminate\Console;

use Closure;
use Illuminate\Console\Events\ArtisanStarting;
use Illuminate\Contracts\Console\Application as ApplicationContract;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Support\ProcessUtils;
use ReflectionClass;
use Symfony\Component\Console\Application as SymfonyApplication;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Command\Command as SymfonyCommand;
use Symfony\Component\Console\Exception\CommandNotFoundException;
use Symfony\Component\Console\Input\ArrayInput;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Input\StringInput;
use Symfony\Component\Console\Output\BufferedOutput;

use function Illuminate\Support\artisan_binary;
use function Illuminate\Support\php_binary;

class Application extends SymfonyApplication implements ApplicationContract
{
    /**
     * The Laravel application instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $laravel;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The output from the previous command.
     *
     * @var \Symfony\Component\Console\Output\OutputInterface|null
     */
    protected $lastOutput;

    /**
     * The console application bootstrappers.
     *
     * @var array<array-key, \Closure($this): void>
     */
    protected static $bootstrappers = [];

    /**
     * A map of command names to classes.
     *
     * @var array<string, \Illuminate\Console\Command|string>
     */
    protected $commandMap = [];

    /**
     * Create a new Artisan console application.
     *
     * @param  \Illuminate\Contracts\Container\Container  $laravel
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @param  string  $version
     */
    public function __construct(Container $laravel, Dispatcher $events, $version)
    {}

    /**
     * Determine the proper PHP executable.
     *
     * @return string
     */
    public static function phpBinary()
    {}

    /**
     * Determine the proper Artisan executable.
     *
     * @return string
     */
    public static function artisanBinary()
    {}

    /**
     * Format the given command as a fully-qualified executable command.
     *
     * @param  string  $string
     * @return string
     */
    public static function formatCommandString($string)
    {}

    /**
     * Register a console "starting" bootstrapper.
     *
     * @param  \Closure($this): void  $callback
     * @return void
     */
    public static function starting(Closure $callback)
    {}

    /**
     * Bootstrap the console application.
     *
     * @return void
     */
    protected function bootstrap()
    {}

    /**
     * Clear the console application bootstrappers.
     *
     * @return void
     */
    public static function forgetBootstrappers()
    {}

    /**
     * Run an Artisan console command by name.
     *
     * @param  \Symfony\Component\Console\Command\Command|string  $command
     * @param  array  $parameters
     * @param  \Symfony\Component\Console\Output\OutputInterface|null  $outputBuffer
     * @return int
     *
     * @throws \Symfony\Component\Console\Exception\CommandNotFoundException
     */
    public function call($command, array $parameters = [], $outputBuffer = null)
    {}

    /**
     * Parse the incoming Artisan command and its input.
     *
     * @param  \Symfony\Component\Console\Command\Command|string  $command
     * @param  array  $parameters
     * @return array{0: string, 1: \Symfony\Component\Console\Input\ArrayInput|\Symfony\Component\Console\Input\StringInput}
     */
    protected function parseCommand($command, $parameters)
    {}

    /**
     * Get the output for the last run command.
     *
     * @return string
     */
    public function output()
    {}

    /**
     * Alias for addCommand() since Symfony's add() method was deprecated.
     *
     * @param  \Symfony\Component\Console\Command\Command  $command
     * @return \Symfony\Component\Console\Command\Command|null
     */
    public function add(SymfonyCommand $command): ?SymfonyCommand
    {}

    /**
     * Add a command to the console.
     *
     * @param  \Symfony\Component\Console\Command\Command|callable  $command
     * @return \Symfony\Component\Console\Command\Command|null
     */
    public function addCommand(SymfonyCommand|callable $command): ?SymfonyCommand
    {}

    /**
     * Add the command to the parent instance.
     *
     * @param  \Symfony\Component\Console\Command\Command|callable  $command
     * @return \Symfony\Component\Console\Command\Command
     */
    protected function addToParent(SymfonyCommand|callable $command)
    {}

    /**
     * Add a command, resolving through the application.
     *
     * @param  \Illuminate\Console\Command|string  $command
     * @return \Symfony\Component\Console\Command\Command|null
     */
    public function resolve($command)
    {}

    /**
     * Resolve an array of commands through the application.
     *
     * @param  mixed  $commands
     * @return $this
     */
    public function resolveCommands($commands)
    {}

    /**
     * Set the container command loader for lazy resolution.
     *
     * @return $this
     */
    public function setContainerCommandLoader()
    {}

    /**
     * Get the default input definition for the application.
     *
     * This is used to add the --env option to every available command.
     *
     * @return \Symfony\Component\Console\Input\InputDefinition
     */
    #[\Override]
    protected function getDefaultInputDefinition(): InputDefinition
    {}

    /**
     * Get the global environment option for the definition.
     *
     * @return \Symfony\Component\Console\Input\InputOption
     */
    protected function getEnvironmentOption()
    {}

    /**
     * Get the Laravel application instance.
     *
     * @return \Illuminate\Contracts\Foundation\Application
     */
    public function getLaravel()
    {}
}
