<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Env;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Stringable;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Process\Process;

use function Illuminate\Support\php_binary;
use function Termwind\terminal;

#[AsCommand(name: 'serve')]
class ServeCommand extends Command
{
    use InteractsWithTime;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'serve
                    {--host= : The host address to serve the application on}
                    {--port= : The port to serve the application on}
                    {--tries=10 : The max number of ports to attempt to serve from}
                    {--no-reload : Do not reload the development server on .env file changes}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Serve the application on the PHP development server';

    /**
     * The number of PHP CLI server workers.
     *
     * @var int<2, max>|false
     */
    protected $phpServerWorkers = 1;

    /**
     * The current port offset.
     *
     * @var int
     */
    protected $portOffset = 0;

    /**
     * The list of lines that are pending to be output.
     *
     * @var string
     */
    protected $outputBuffer = '';

    /**
     * The list of requests being handled and their start time.
     *
     * @var array<int, \Illuminate\Support\Carbon>
     */
    protected $requestsPool;

    /**
     * Indicates if the "Server running on..." output message has been displayed.
     *
     * @var bool
     */
    protected $serverRunningHasBeenDisplayed = false;

    /**
     * The environment variables that should be passed from host machine to the PHP server process.
     *
     * @var string[]
     */
    public static $passthroughVariables = [
        'APP_ENV',
        'HERD_PHP_81_INI_SCAN_DIR',
        'HERD_PHP_82_INI_SCAN_DIR',
        'HERD_PHP_83_INI_SCAN_DIR',
        'HERD_PHP_84_INI_SCAN_DIR',
        'HERD_PHP_85_INI_SCAN_DIR',
        'IGNITION_LOCAL_SITES_PATH',
        'LARAVEL_SAIL',
        'PATH',
        'PHP_IDE_CONFIG',
        'SYSTEMROOT',
        'XDEBUG_CONFIG',
        'XDEBUG_MODE',
        'XDEBUG_SESSION',
    ];

    #[\Override]
    protected function configureDefaults(): void
    {}

    /** {@inheritdoc} */
    #[\Override]
    protected function initialize(InputInterface $input, OutputInterface $output): void
    {}

    /**
     * Execute the console command.
     *
     * @return int
     *
     * @throws \Exception
     */
    public function handle()
    {}

    /**
     * Start a new server process.
     *
     * @param  bool  $hasEnvironment
     * @return \Symfony\Component\Process\Process
     */
    protected function startProcess($hasEnvironment)
    {}

    /**
     * Get the full server command.
     *
     * @return array
     */
    protected function serverCommand()
    {}

    /**
     * Get the host for the command.
     *
     * @return string
     */
    protected function host()
    {}

    /**
     * Get the port for the command.
     *
     * @return int
     */
    protected function port()
    {}

    /**
     * Get the host and port from the host option string.
     *
     * @return array
     */
    protected function getHostAndPort()
    {}

    /**
     * Check if the command has reached its maximum number of port tries.
     *
     * @return bool
     */
    protected function canTryAnotherPort()
    {}

    /**
     * Determine if the environment variable should be passed to the PHP server process.
     *
     * @param  string  $key
     * @return bool
     */
    protected function shouldPassThroughEnvironmentVariable($key)
    {}

    /**
     * Returns a "callable" to handle the process output.
     *
     * @return callable(string, string): void
     */
    protected function handleProcessOutput()
    {}

    /**
     * Flush the output buffer.
     *
     * @return void
     */
    protected function flushOutputBuffer()
    {}

    /**
     * Get the date from the given PHP server output.
     *
     * @param  string  $line
     * @return \Illuminate\Support\Carbon
     */
    protected function getDateFromLine($line)
    {}

    /**
     * Get the request port from the given PHP server output.
     *
     * @param  string  $line
     * @return int
     *
     * @throws \InvalidArgumentException
     */
    public static function getRequestPortFromLine($line)
    {}
}
