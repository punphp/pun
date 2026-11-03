<?php

namespace Illuminate\Database\Console\Migrations;

use Illuminate\Console\Command;
use Illuminate\Console\ConfirmableTrait;
use Illuminate\Console\Prohibitable;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\Events\DatabaseRefreshed;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'migrate:refresh')]
class RefreshCommand extends Command
{
    use ConfirmableTrait, Prohibitable;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'migrate:refresh
                    {--database= : The database connection to use}
                    {--force : Force the operation to run when in production}
                    {--path=* : The path(s) to the migrations files to be executed}
                    {--realpath : Indicate any provided migration file paths are pre-resolved absolute paths}
                    {--seed : Indicates if the seed task should be re-run}
                    {--seeder= : The class name of the root seeder}
                    {--step= : The number of migrations to be reverted & re-run}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Reset and re-run all migrations';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Run the rollback command.
     *
     * @param  string  $database
     * @param  string  $path
     * @param  int  $step
     * @return void
     */
    protected function runRollback($database, $path, $step)
    {}

    /**
     * Run the reset command.
     *
     * @param  string  $database
     * @param  string  $path
     * @return void
     */
    protected function runReset($database, $path)
    {}

    /**
     * Determine if the developer has requested database seeding.
     *
     * @return bool
     */
    protected function needsSeeding()
    {}

    /**
     * Run the database seeder command.
     *
     * @param  string  $database
     * @return void
     */
    protected function runSeeder($database)
    {}
}
