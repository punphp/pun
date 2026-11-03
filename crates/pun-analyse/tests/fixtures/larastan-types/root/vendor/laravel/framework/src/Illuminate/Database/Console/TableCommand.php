<?php

namespace Illuminate\Database\Console;

use Illuminate\Database\ConnectionResolverInterface;
use Illuminate\Database\Console\Concerns\InteractsWithPooledConnections;
use Illuminate\Database\Schema\Builder;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Number;
use Symfony\Component\Console\Attribute\AsCommand;

use function Laravel\Prompts\search;

#[AsCommand(name: 'db:table')]
class TableCommand extends DatabaseInspectionCommand
{
    use InteractsWithPooledConnections;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'db:table
                            {table? : The name of the table}
                            {--database= : The database connection}
                            {--json : Output the table information as JSON}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Display information about the given database table';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle(ConnectionResolverInterface $connections)
    {}

    /**
     * Get the information regarding the table's columns.
     *
     * @param  \Illuminate\Database\Schema\Builder  $schema
     * @param  string  $table
     * @return \Illuminate\Support\Collection
     */
    protected function columns(Builder $schema, string $table)
    {}

    /**
     * Get the attributes for a table column.
     *
     * @param  array  $column
     * @return \Illuminate\Support\Collection
     */
    protected function getAttributesForColumn($column)
    {}

    /**
     * Get the information regarding the table's indexes.
     *
     * @param  \Illuminate\Database\Schema\Builder  $schema
     * @param  string  $table
     * @return \Illuminate\Support\Collection
     */
    protected function indexes(Builder $schema, string $table)
    {}

    /**
     * Get the attributes for a table index.
     *
     * @param  array  $index
     * @return \Illuminate\Support\Collection
     */
    protected function getAttributesForIndex($index)
    {}

    /**
     * Get the information regarding the table's foreign keys.
     *
     * @param  \Illuminate\Database\Schema\Builder  $schema
     * @param  string  $table
     * @return \Illuminate\Support\Collection
     */
    protected function foreignKeys(Builder $schema, string $table)
    {}

    /**
     * Render the table information.
     *
     * @param  array  $data
     * @return void
     */
    protected function display(array $data)
    {}

    /**
     * Render the table information as JSON.
     *
     * @param  array  $data
     * @return void
     */
    protected function displayJson(array $data)
    {}

    /**
     * Render the table information formatted for the CLI.
     *
     * @param  array  $data
     * @return void
     */
    protected function displayForCli(array $data)
    {}
}
