<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Support\Arr;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'config:show')]
class ConfigShowCommand extends Command
{
    /**
     * The console command signature.
     *
     * @var string
     */
    protected $signature = 'config:show {config : The configuration file or key to show}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Display all of the values for a given configuration file or key';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Render the configuration values.
     *
     * @param  string  $name
     * @return void
     */
    public function render($name)
    {}

    /**
     * Render the title.
     *
     * @param  string  $title
     * @param  string|null  $subtitle
     * @return void
     */
    public function title($title, $subtitle = null)
    {}

    /**
     * Format the given configuration key.
     *
     * @param  string  $key
     * @return string
     */
    protected function formatKey($key)
    {}

    /**
     * Format the given configuration value.
     *
     * @param  mixed  $value
     * @return string
     */
    protected function formatValue($value)
    {}
}
