<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Console\Prohibitable;
use Illuminate\Foundation\DevCommandMode;
use Illuminate\Foundation\DevCommands;
use Illuminate\Support\Facades\Process;
use Illuminate\Support\NodePackageManager;
use Symfony\Component\Console\Attribute\AsCommand;

use function Termwind\terminal;

/**
 * @phpstan-import-type DevCommandArray from \Illuminate\Foundation\DevCommands
 */
#[AsCommand(name: 'dev')]
class DevCommand extends Command
{
    use Prohibitable;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'dev
        {--s|stream : Start in stream mode}
        {--t|tabs : Start in tabs mode}
        {--i|inline : Print output inline instead of rendering the TUI (the default when not a TTY)}
        {--timestamps : Display timestamps on each output line}
        {--no-restart : Disable auto-restart on crash}
        {--json : Emit newline-delimited JSON events. Implies --inline}
        {--buffer-size= : Set the max lines per command buffer}
        {--stream-buffer-size= : Set the max lines in the stream buffer}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Run the dev processes';

    /**
     * Execute the console command.
     *
     * @return int
     *
     * @throws \Exception
     */
    public function handle(NodePackageManager $packageManager): int
    {}

    /**
     * Run the dev commands via `@laravel/multiplex`.
     *
     * @param  DevCommandArray[]  $devCommands
     * @param  NodePackageManager  $packageManager
     * @return int
     */
    protected function runViaMultiplex(array $devCommands, NodePackageManager $packageManager): int
    {}

    /**
     * Build the command to run `@laravel/multiplex` with the given dev commands.
     *
     * @param  DevCommandArray[]  $devCommands
     * @return string
     */
    protected function buildMultiplexCommand(array $devCommands): string
    {}

    /**
     * Run the dev commands via `concurrently`.
     *
     * @param  DevCommandArray[]  $devCommands
     * @param  NodePackageManager  $packageManager
     * @return int
     */
    protected function runViaConcurrently(array $devCommands, NodePackageManager $packageManager): int
    {}

    /**
     * Build the command to run `concurrently` with the given dev commands.
     *
     * @param  DevCommandArray[]  $devCommands
     * @return string
     */
    protected function buildConcurrentlyCommand(array $devCommands): string
    {}
}
