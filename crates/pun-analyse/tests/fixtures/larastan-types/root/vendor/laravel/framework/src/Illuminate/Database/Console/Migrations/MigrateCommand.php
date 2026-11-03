<?php

namespace Illuminate\Database\Console\Migrations;

use Illuminate\Console\ConfirmableTrait;
use Illuminate\Contracts\Console\Isolatable;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\Events\SchemaLoaded;
use Illuminate\Database\Migrations\Migrator;
use Illuminate\Database\SQLiteDatabaseDoesNotExistException;
use Illuminate\Database\SqlServerConnection;
use Illuminate\Support\Str;
use PDOException;
use RuntimeException;
use Symfony\Component\Console\Attribute\AsCommand;
use Throwable;

use function Laravel\Prompts\confirm;

#[AsCommand(name: 'migrate')]
class MigrateCommand extends BaseCommand implements Isolatable
{
    use ConfirmableTrait;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'migrate {--database= : The database connection to use}
                {--force : Force the operation to run when in production}
                {--path=* : The path(s) to the migrations files to be executed}
                {--realpath : Indicate any provided migration file paths are pre-resolved absolute paths}
                {--schema-path= : The path to a schema dump file}
                {--pretend : Dump the SQL queries that would be run}
                {--seed : Indicates if the seed task should be re-run}
                {--seeder= : The class name of the root seeder}
                {--step : Force the migrations to be run so they can be rolled back individually}
                {--graceful : Return a successful exit code even if an error occurs}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Run the database migrations';

    /**
     * The migrator instance.
     *
     * @var \Illuminate\Database\Migrations\Migrator
     */
    protected $migrator;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $dispatcher;

    /**
     * Create a new migration command instance.
     *
     * @param  \Illuminate\Database\Migrations\Migrator  $migrator
     * @param  \Illuminate\Contracts\Events\Dispatcher  $dispatcher
     */
    public function __construct(Migrator $migrator, Dispatcher $dispatcher)
    {}

    /**
     * Execute the console command.
     *
     * @return int
     *
     * @throws \Throwable
     */
    public function handle()
    {}

    /**
     * Run the pending migrations.
     *
     * @return void
     */
    protected function runMigrations()
    {}

    /**
     * Prepare the migration database for running.
     *
     * @return void
     */
    protected function prepareDatabase()
    {}

    /**
     * Determine if the migrator repository exists.
     *
     * @return bool
     */
    protected function repositoryExists()
    {}

    /**
     * Attempt to create the database if it is missing.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    protected function handleMissingDatabase(Throwable $e)
    {}

    /**
     * Create a missing SQLite database.
     *
     * @param  string  $path
     * @return bool
     *
     * @throws \RuntimeException
     */
    protected function createMissingSqliteDatabase($path)
    {}

    /**
     * Create a missing MySQL or Postgres database.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @return bool
     *
     * @throws \RuntimeException
     */
    protected function createMissingMySqlOrPgsqlDatabase($connection)
    {}

    /**
     * Load the schema state to seed the initial database schema structure.
     *
     * @return void
     */
    protected function loadSchemaState()
    {}

    /**
     * Get the path to the stored schema for the given connection.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @return string
     */
    protected function schemaPath($connection)
    {}
}
