<?php

namespace Illuminate\Foundation\Console;

use Composer\InstalledVersions;
use Illuminate\Console\Command;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Arr;
use Illuminate\Support\Env;
use Illuminate\Support\Facades\Process;
use Symfony\Component\Console\Attribute\AsCommand;

use function Illuminate\Support\artisan_binary;
use function Illuminate\Support\php_binary;
use function Laravel\Prompts\confirm;
use function Laravel\Prompts\note;
use function Laravel\Prompts\password;
use function Laravel\Prompts\select;
use function Laravel\Prompts\text;

#[AsCommand(name: 'install:broadcasting')]
class BroadcastingInstallCommand extends Command
{
    use InteractsWithComposerPackages;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'install:broadcasting
                    {--composer=global : Absolute path to the Composer binary which should be used to install packages}
                    {--force : Overwrite any existing broadcasting routes file}
                    {--without-reverb : Do not prompt to install Laravel Reverb}
                    {--reverb : Install Laravel Reverb as the default broadcaster}
                    {--pusher : Install Pusher as the default broadcaster}
                    {--ably : Install Ably as the default broadcaster}
                    {--mercure : Install Mercure as the default broadcaster}
                    {--pretend : Run dependency installation commands in dry-run mode}
                    {--without-node : Do not prompt to install Node dependencies}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Create a broadcasting channel routes file';

    /**
     * The broadcasting driver to use.
     *
     * @var string|null
     */
    protected $driver = null;

    /**
     * The Composer packages required by each broadcasting driver.
     *
     * @var array<string, array<string, string>>
     */
    protected $driverPackages = [
        'pusher' => ['pusher/pusher-php-server' => '*'],
        'ably' => ['ably/ably-php' => '*'],
        'mercure' => ['symfony/mercure' => '^0.8', 'web-token/jwt-library' => '^4.1'],
    ];

    /**
     * The framework packages to install.
     *
     * @var array
     */
    protected $frameworkPackages = [
        'react' => '@laravel/echo-react',
        'vue' => '@laravel/echo-vue',
    ];

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Uncomment the "channels" routes file in the application bootstrap file.
     *
     * @return void
     */
    protected function uncommentChannelsRoutesFile()
    {}

    /**
     * Uncomment the "BroadcastServiceProvider" in the application configuration.
     *
     * @return void
     */
    protected function enableBroadcastServiceProvider()
    {}

    /**
     * Collect the driver configuration.
     *
     * @return void
     */
    protected function collectDriverConfig()
    {}

    /**
     * Install the driver packages.
     *
     * @return void
     */
    protected function installDriverPackages()
    {}

    /**
     * Collect the Pusher configuration.
     *
     * @return void
     */
    protected function collectPusherConfig()
    {}

    /**
     * Collect the Ably configuration.
     *
     * @return void
     */
    protected function collectAblyConfig()
    {}

    /**
     * Collect the Mercure configuration.
     *
     * @return void
     */
    protected function collectMercureConfig()
    {}

    /**
     * Inject Echo configuration into the application's main file.
     *
     * @return void
     */
    protected function injectFrameworkSpecificConfiguration()
    {}

    /**
     * Install Laravel Reverb into the application if desired.
     *
     * @return void
     */
    protected function installReverb()
    {}

    /**
     * Install and build Node dependencies.
     *
     * @return void
     */
    protected function installNodeDependencies()
    {}

    /**
     * Resolve the provider to use based on the user's choice.
     *
     * @return string
     */
    protected function resolveDriver(): string
    {}

    /**
     * Detect if the user is using a supported framework (React or Vue).
     *
     * @return bool
     */
    protected function isUsingSupportedFramework(): bool
    {}

    /**
     * Detect if the user is using React.
     *
     * @return bool
     */
    protected function appUsesReact(): bool
    {}

    /**
     * Detect if the user is using Vue.
     *
     * @return bool
     */
    protected function appUsesVue(): bool
    {}

    /**
     * Detect if the package is installed.
     *
     * @return bool
     */
    protected function packageDependenciesInclude(string $package): bool
    {}
}
