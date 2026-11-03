<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Process;
use Symfony\Component\Console\Attribute\AsCommand;

use function Illuminate\Support\artisan_binary;
use function Illuminate\Support\php_binary;

#[AsCommand(name: 'install:api')]
class ApiInstallCommand extends Command
{
    use InteractsWithComposerPackages;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'install:api
                    {--composer=global : Absolute path to the Composer binary which should be used to install packages}
                    {--force : Overwrite any existing API routes file}
                    {--passport : Install Laravel Passport instead of Laravel Sanctum}
                    {--without-migration-prompt : Do not prompt to run pending migrations}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Create an API routes file and install Laravel Sanctum or Laravel Passport';

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Uncomment the API routes file in the application bootstrap file.
     *
     * @return void
     */
    protected function uncommentApiRoutesFile()
    {}

    /**
     * Install Laravel Sanctum into the application.
     *
     * @return void
     */
    protected function installSanctum()
    {}

    /**
     * Install Laravel Passport into the application.
     *
     * @return void
     */
    protected function installPassport()
    {}
}
