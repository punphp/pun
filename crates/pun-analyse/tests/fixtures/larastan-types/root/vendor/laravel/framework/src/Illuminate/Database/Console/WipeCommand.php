<?php

namespace Illuminate\Database\Console;

use Illuminate\Console\Command;
use Illuminate\Console\ConfirmableTrait;
use Illuminate\Console\Prohibitable;
use Illuminate\Database\Console\Concerns\InteractsWithPooledConnections;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'db:wipe')]
class WipeCommand extends Command
{
    use ConfirmableTrait, Prohibitable, InteractsWithPooledConnections;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'db:wipe
                    {--database= : The database connection to use}
                    {--drop-views : Drop all tables and views}
                    {--drop-types : Drop all tables and types (Postgres only)}
                    {--force : Force the operation to run when in production}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Drop all tables, views, and types';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Drop all of the database tables.
     *
     * @param  string  $database
     * @return void
     */
    protected function dropAllTables($database)
    {}

    /**
     * Drop all of the database views.
     *
     * @param  string  $database
     * @return void
     */
    protected function dropAllViews($database)
    {}

    /**
     * Drop all of the database types.
     *
     * @param  string  $database
     * @return void
     */
    protected function dropAllTypes($database)
    {}

    /**
     * Flush the given database connection.
     *
     * @param  string  $database
     * @return void
     */
    protected function flushDatabaseConnection($database)
    {}
}
