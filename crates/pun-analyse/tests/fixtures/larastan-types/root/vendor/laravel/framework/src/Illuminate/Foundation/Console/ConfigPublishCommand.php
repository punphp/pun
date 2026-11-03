<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Support\Collection;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Finder\Finder;

use function Laravel\Prompts\select;

#[AsCommand(name: 'config:publish')]
class ConfigPublishCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'config:publish
                    {name? : The name of the configuration file to publish}
                    {--all : Publish all configuration files}
                    {--force : Overwrite any existing configuration files}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Publish configuration files to your application';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Publish the given file to the given destination.
     *
     * @param  string  $name
     * @param  string  $file
     * @param  string  $destination
     * @return void
     */
    protected function publish(string $name, string $file, string $destination)
    {}

    /**
     * Get an array containing the base configuration files.
     *
     * @return array
     */
    protected function getBaseConfigurationFiles()
    {}
}
