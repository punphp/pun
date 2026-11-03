<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Support\Collection;
use Illuminate\Support\ServiceProvider;
use Illuminate\Support\Stringable;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'reload')]
class ReloadCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'reload {--e|except= : The commands to skip}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Reload running services';

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Get the commands that should be reloaded.
     *
     * @return array
     */
    public function getReloadTasks()
    {}
}
