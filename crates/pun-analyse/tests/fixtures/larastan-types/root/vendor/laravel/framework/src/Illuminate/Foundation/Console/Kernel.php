<?php

namespace Illuminate\Foundation\Console;

use Carbon\CarbonInterval;
use Closure;
use DateTimeInterface;
use Illuminate\Console\Application as Artisan;
use Illuminate\Console\Command;
use Illuminate\Console\Events\CommandFinished;
use Illuminate\Console\Events\CommandStarting;
use Illuminate\Console\Scheduling\Schedule;
use Illuminate\Contracts\Console\Kernel as KernelContract;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Foundation\Events\Terminating;
use Illuminate\Support\Arr;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Env;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Str;
use ReflectionClass;
use SplFileInfo;
use Symfony\Component\Console\ConsoleEvents;
use Symfony\Component\Console\Event\ConsoleCommandEvent;
use Symfony\Component\Console\Event\ConsoleTerminateEvent;
use Symfony\Component\EventDispatcher\EventDispatcher;
use Symfony\Component\Finder\Finder;
use Throwable;
use WeakMap;

class Kernel implements KernelContract
{
    use InteractsWithTime;

    /**
     * The application implementation.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The event dispatcher implementation.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The Symfony event dispatcher implementation.
     *
     * @var \Symfony\Contracts\EventDispatcher\EventDispatcherInterface|null
     */
    protected $symfonyDispatcher;

    /**
     * The Artisan application instance.
     *
     * @var \Illuminate\Console\Application|null
     */
    protected $artisan;

    /**
     * The Artisan commands provided by the application.
     *
     * @var array
     */
    protected $commands = [];

    /**
     * The paths where Artisan commands should be automatically discovered.
     *
     * @var array
     */
    protected $commandPaths = [];

    /**
     * The paths where Artisan "routes" should be automatically discovered.
     *
     * @var array
     */
    protected $commandRoutePaths = [];

    /**
     * Indicates if the Closure commands have been loaded.
     *
     * @var bool
     */
    protected $commandsLoaded = false;

    /**
     * The commands paths that have been "loaded".
     *
     * @var array
     */
    protected $loadedPaths = [];

    /**
     * The commands registered through the "load" and "command" methods.
     *
     * @var array
     */
    protected $kernelCommands = [];

    /**
     * All of the registered command duration handlers.
     *
     * @var array
     */
    protected $commandLifecycleDurationHandlers = [];

    /**
     * When the currently handled command started.
     *
     * @var \Illuminate\Support\Carbon|null
     */
    protected $commandStartedAt;

    /**
     * The bootstrap classes for the application.
     *
     * @var string[]
     */
    protected $bootstrappers = [
        \Illuminate\Foundation\Bootstrap\LoadEnvironmentVariables::class,
        \Illuminate\Foundation\Bootstrap\LoadConfiguration::class,
        \Illuminate\Foundation\Bootstrap\HandleExceptions::class,
        \Illuminate\Foundation\Bootstrap\RegisterFacades::class,
        \Illuminate\Foundation\Bootstrap\SetRequestForConsole::class,
        \Illuminate\Foundation\Bootstrap\RegisterProviders::class,
        \Illuminate\Foundation\Bootstrap\BootProviders::class,
    ];

    /**
     * Create a new console kernel instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     */
    public function __construct(Application $app, Dispatcher $events)
    {}

    /**
     * Re-route the Symfony command events to their Laravel counterparts.
     *
     * @internal
     *
     * @return $this
     */
    public function rerouteSymfonyCommandEvents()
    {}

    /**
     * Run the console application.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @param  \Symfony\Component\Console\Output\OutputInterface|null  $output
     * @return int
     */
    public function handle($input, $output = null)
    {}

    /**
     * Terminate the application.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @param  int  $status
     * @return void
     */
    public function terminate($input, $status)
    {}

    /**
     * Register a callback to be invoked when the command lifecycle duration exceeds a given amount of time.
     *
     * @param  \DateTimeInterface|\Carbon\CarbonInterval|float|int  $threshold
     * @param  callable  $handler
     * @return void
     */
    public function whenCommandLifecycleIsLongerThan($threshold, $handler)
    {}

    /**
     * When the command being handled started.
     *
     * @return \Illuminate\Support\Carbon|null
     */
    public function commandStartedAt()
    {}

    /**
     * Define the application's command schedule.
     *
     * @param  \Illuminate\Console\Scheduling\Schedule  $schedule
     * @return void
     */
    protected function schedule(Schedule $schedule)
    {}

    /**
     * Resolve a console schedule instance.
     *
     * @return \Illuminate\Console\Scheduling\Schedule
     */
    public function resolveConsoleSchedule()
    {}

    /**
     * Get the timezone that should be used by default for scheduled events.
     *
     * @return \DateTimeZone|string|null
     */
    protected function scheduleTimezone()
    {}

    /**
     * Get the name of the cache store that should manage scheduling mutexes.
     *
     * @return string|null
     */
    protected function scheduleCache()
    {}

    /**
     * Register the commands for the application.
     *
     * @return void
     */
    protected function commands()
    {}

    /**
     * Register a Closure based command with the application.
     *
     * @param  string  $signature
     * @param  \Closure  $callback
     * @return \Illuminate\Foundation\Console\ClosureCommand
     */
    public function command($signature, Closure $callback)
    {}

    /**
     * Register all of the commands in the given directory.
     *
     * @param  array|string  $paths
     * @return void
     */
    protected function load($paths)
    {}

    /**
     * Get the Finder instance for discovering command files.
     *
     * @param  array  $paths
     * @return \Symfony\Component\Finder\Finder
     */
    protected function findCommands(array $paths)
    {}

    /**
     * Extract the command class name from the given file path.
     *
     * @param  \SplFileInfo  $file
     * @param  string  $namespace
     * @return string
     */
    protected function commandClassFromFile(SplFileInfo $file, string $namespace): string
    {}

    /**
     * Register the given command with the console application.
     *
     * @param  \Symfony\Component\Console\Command\Command  $command
     * @return void
     */
    public function registerCommand($command)
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
     * Queue the given console command.
     *
     * @param  string  $command
     * @param  array  $parameters
     * @return \Illuminate\Foundation\Bus\PendingDispatch
     */
    public function queue($command, array $parameters = [])
    {}

    /**
     * Get the registered command instance with the given name, if any.
     *
     * @param  string  $name
     * @return \Symfony\Component\Console\Command\Command|null
     */
    public function findCommand($name)
    {}

    /**
     * Get all of the commands registered with the console.
     *
     * @return array
     */
    public function all()
    {}

    /**
     * Get the output for the last run command.
     *
     * @return string
     */
    public function output()
    {}

    /**
     * Bootstrap the application for artisan commands.
     *
     * @return void
     */
    public function bootstrap()
    {}

    /**
     * Discover the commands that should be automatically loaded.
     *
     * @return void
     */
    protected function discoverCommands()
    {}

    /**
     * Bootstrap the application without booting service providers.
     *
     * @return void
     */
    public function bootstrapWithoutBootingProviders()
    {}

    /**
     * Determine if the kernel should discover commands.
     *
     * @return bool
     */
    protected function shouldDiscoverCommands()
    {}

    /**
     * Get the Artisan application instance.
     *
     * @return \Illuminate\Console\Application
     */
    protected function getArtisan()
    {}

    /**
     * Set the Artisan application instance.
     *
     * @param  \Illuminate\Console\Application|null  $artisan
     * @return void
     */
    public function setArtisan($artisan)
    {}

    /**
     * Set the Artisan commands provided by the application.
     *
     * @param  array  $commands
     * @return $this
     */
    public function addCommands(array $commands)
    {}

    /**
     * Set the paths that should have their Artisan commands automatically discovered.
     *
     * @param  array  $paths
     * @return $this
     */
    public function addCommandPaths(array $paths)
    {}

    /**
     * Set the paths that should have their Artisan "routes" automatically discovered.
     *
     * @param  array  $paths
     * @return $this
     */
    public function addCommandRoutePaths(array $paths)
    {}

    /**
     * Get the bootstrap classes for the application.
     *
     * @return array
     */
    protected function bootstrappers()
    {}

    /**
     * Report the exception to the exception handler.
     *
     * @param  \Throwable  $e
     * @return void
     */
    protected function reportException(Throwable $e)
    {}

    /**
     * Render the given exception.
     *
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @param  \Throwable  $e
     * @return void
     */
    protected function renderException($output, Throwable $e)
    {}
}
