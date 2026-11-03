<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Foundation\DevCommand;
use Illuminate\Foundation\DevCommands;
use Illuminate\Support\Stringable;
use Laravel\Prompts\Prompt;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Input\InputOption;

#[AsCommand(name: 'dev:list')]
class DevListCommand extends Command
{
    /**
     * The console command name.
     *
     * @var string
     */
    protected $name = 'dev:list';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'List the registered dev processes';

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
     * Determine if the given command is registered by a vendor package.
     *
     * @param  array  $command
     * @return bool
     */
    protected function isVendorCommand(array $command): bool
    {}

    /**
     * Determine if any filtering options are active.
     *
     * @return bool
     */
    protected function isFiltering(): bool
    {}

    /**
     * Filter the given commands based on the provided options.
     *
     * @param  array  $commands
     * @return array
     */
    protected function filterCommands(array $commands): array
    {}

    /**
     * Format the source information for display.
     *
     * @param  array{'file': string, 'line': int, 'class'?: string, 'function'?: string}  $source
     * @return string
     */
    protected function formatSource($source): string
    {}

    /**
     * Get the console command options.
     *
     * @return array
     */
    protected function getOptions()
    {}
}
