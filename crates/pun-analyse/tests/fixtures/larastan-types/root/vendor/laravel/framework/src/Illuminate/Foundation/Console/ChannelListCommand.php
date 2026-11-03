<?php

namespace Illuminate\Foundation\Console;

use Closure;
use Illuminate\Console\Command;
use Illuminate\Contracts\Broadcasting\Broadcaster;
use Illuminate\Support\Collection;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Terminal;

#[AsCommand(name: 'channel:list')]
class ChannelListCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'channel:list';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'List all registered private broadcast channels';

    /**
     * The terminal width resolver callback.
     *
     * @var \Closure|null
     */
    protected static $terminalWidthResolver;

    /**
     * Execute the console command.
     *
     * @param  \Illuminate\Contracts\Broadcasting\Broadcaster  $broadcaster
     * @return void
     */
    public function handle(Broadcaster $broadcaster)
    {}

    /**
     * Display the channel information on the console.
     *
     * @param  Collection  $channels
     * @return void
     */
    protected function displayChannels($channels)
    {}

    /**
     * Convert the given channels to regular CLI output.
     *
     * @param  \Illuminate\Support\Collection  $channels
     * @return array
     */
    protected function forCli($channels)
    {}

    /**
     * Determine and return the output for displaying the number of registered channels in the CLI output.
     *
     * @param  \Illuminate\Support\Collection  $channels
     * @param  int  $terminalWidth
     * @return string
     */
    protected function determineChannelCountOutput($channels, $terminalWidth)
    {}

    /**
     * Get the terminal width.
     *
     * @return int
     */
    public static function getTerminalWidth()
    {}

    /**
     * Set a callback that should be used when resolving the terminal width.
     *
     * @param  \Closure|null  $resolver
     * @return void
     */
    public static function resolveTerminalWidthUsing($resolver)
    {}
}
