<?php

namespace Illuminate\Database\Console;

use Illuminate\Database\ConnectionInterface;
use Illuminate\Database\ConnectionResolverInterface;
use Illuminate\Database\Console\Concerns\InteractsWithPooledConnections;
use Illuminate\Database\Schema\Builder;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Number;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'db:show')]
class ShowCommand extends DatabaseInspectionCommand
{
    use InteractsWithPooledConnections;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'db:show {--database= : The database connection}
                {--json : Output the database information as JSON}
                {--counts : Show the table row count <bg=red;options=bold> Note: This can be slow on large databases </>}
                {--views : Show the database views <bg=red;options=bold> Note: This can be slow on large databases </>}
                {--types : Show the user defined types}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Display information about the given database';

    /**
     * Execute the console command.
     *
     * @param  \Illuminate\Database\ConnectionResolverInterface  $connections
     * @return int
     */
    public function handle(ConnectionResolverInterface $connections)
    {}

    /**
     * Get information regarding the tables within the database.
     *
     * @param  \Illuminate\Database\ConnectionInterface  $connection
     * @param  \Illuminate\Database\Schema\Builder  $schema
     * @return \Illuminate\Support\Collection
     */
    protected function tables(ConnectionInterface $connection, Builder $schema)
    {}

    /**
     * Get information regarding the views within the database.
     *
     * @param  \Illuminate\Database\ConnectionInterface  $connection
     * @param  \Illuminate\Database\Schema\Builder  $schema
     * @return \Illuminate\Support\Collection
     */
    protected function views(ConnectionInterface $connection, Builder $schema)
    {}

    /**
     * Get information regarding the user-defined types within the database.
     *
     * @param  \Illuminate\Database\ConnectionInterface  $connection
     * @param  \Illuminate\Database\Schema\Builder  $schema
     * @return \Illuminate\Support\Collection
     */
    protected function types(ConnectionInterface $connection, Builder $schema)
    {}

    /**
     * Render the database information.
     *
     * @param  array  $data
     * @return void
     */
    protected function display(array $data)
    {}

    /**
     * Render the database information as JSON.
     *
     * @param  array  $data
     * @return void
     */
    protected function displayJson(array $data)
    {}

    /**
     * Render the database information formatted for the CLI.
     *
     * @param  array  $data
     * @return void
     */
    protected function displayForCli(array $data)
    {}
}
