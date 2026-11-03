<?php

namespace Illuminate\Database\Migrations;

use Closure;
use Illuminate\Console\View\Components\BulletList;
use Illuminate\Console\View\Components\Info;
use Illuminate\Console\View\Components\Task;
use Illuminate\Console\View\Components\TwoColumnDetail;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\ConnectionResolverInterface as Resolver;
use Illuminate\Database\Events\MigrationEnded;
use Illuminate\Database\Events\MigrationsEnded;
use Illuminate\Database\Events\MigrationSkipped;
use Illuminate\Database\Events\MigrationsStarted;
use Illuminate\Database\Events\MigrationStarted;
use Illuminate\Database\Events\NoPendingMigrations;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use ReflectionClass;
use Symfony\Component\Console\Output\OutputInterface;

class Migrator
{
    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $events;

    /**
     * The migration repository implementation.
     *
     * @var \Illuminate\Database\Migrations\MigrationRepositoryInterface
     */
    protected $repository;

    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * The connection resolver instance.
     *
     * @var \Illuminate\Database\ConnectionResolverInterface
     */
    protected $resolver;

    /**
     * The custom connection resolver callback.
     *
     * @var (\Closure(\Illuminate\Database\ConnectionResolverInterface, ?string): \Illuminate\Database\Connection)|null
     */
    protected static $connectionResolverCallback;

    /**
     * The name of the default connection.
     *
     * @var string|null
     */
    protected $connection;

    /**
     * The paths to all of the migration files.
     *
     * @var string[]
     */
    protected $paths = [];

    /**
     * The paths that have already been required.
     *
     * @var array<string, \Illuminate\Database\Migrations\Migration|null>
     */
    protected static $requiredPathCache = [];

    /**
     * The output interface implementation.
     *
     * @var \Symfony\Component\Console\Output\OutputInterface|null
     */
    protected $output;

    /**
     * The pending migrations to skip.
     *
     * @var list<string>
     */
    protected static $withoutMigrations = [];

    /**
     * Create a new migrator instance.
     *
     * @param  \Illuminate\Database\Migrations\MigrationRepositoryInterface  $repository
     * @param  \Illuminate\Database\ConnectionResolverInterface  $resolver
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  \Illuminate\Contracts\Events\Dispatcher|null  $dispatcher
     */
    public function __construct(
        MigrationRepositoryInterface $repository,
        Resolver $resolver,
        Filesystem $files,
        ?Dispatcher $dispatcher = null,
    ) {}

    /**
     * Run the pending migrations at a given path.
     *
     * @param  string[]|string  $paths
     * @param  array<string, mixed>  $options
     * @return string[]
     */
    public function run($paths = [], array $options = [])
    {}

    /**
     * Get the migration files that have not yet run.
     *
     * @param  string[]  $files
     * @param  string[]  $ran
     * @return string[]
     */
    protected function pendingMigrations($files, $ran)
    {}

    /**
     * Get list of pending migrations to skip.
     *
     * @return list<string>
     */
    protected function migrationsToSkip()
    {}

    /**
     * Run an array of migrations.
     *
     * @param  string[]  $migrations
     * @param  array<string, mixed>  $options
     * @return void
     */
    public function runPending(array $migrations, array $options = [])
    {}

    /**
     * Run "up" a migration instance.
     *
     * @param  string  $file
     * @param  int  $batch
     * @param  bool  $pretend
     * @return void
     */
    protected function runUp($file, $batch, $pretend)
    {}

    /**
     * Rollback the last migration operation.
     *
     * @param  string[]|string  $paths
     * @param  array<string, mixed>  $options
     * @return string[]
     */
    public function rollback($paths = [], array $options = [])
    {}

    /**
     * Get the migrations for a rollback operation.
     *
     * @param  array<string, mixed>  $options
     * @return object{id: int, migration: string, batch: int}[]
     */
    protected function getMigrationsForRollback(array $options)
    {}

    /**
     * Rollback the given migrations.
     *
     * @param  array  $migrations
     * @param  string[]|string  $paths
     * @param  array<string, mixed>  $options
     * @return string[]
     */
    protected function rollbackMigrations(array $migrations, $paths, array $options)
    {}

    /**
     * Rolls all of the currently applied migrations back.
     *
     * @param  string[]|string  $paths
     * @param  bool  $pretend
     * @return array
     */
    public function reset($paths = [], $pretend = false)
    {}

    /**
     * Reset the given migrations.
     *
     * @param  string[]  $migrations
     * @param  string[]  $paths
     * @param  bool  $pretend
     * @return array
     */
    protected function resetMigrations(array $migrations, array $paths, $pretend = false)
    {}

    /**
     * Run "down" a migration instance.
     *
     * @param  string  $file
     * @param  object  $migration
     * @param  bool  $pretend
     * @return void
     */
    protected function runDown($file, $migration, $pretend)
    {}

    /**
     * Run a migration inside a transaction if the database supports it.
     *
     * @param  object  $migration
     * @param  string  $method
     * @param  string|null  $name
     * @return void
     */
    protected function runMigration($migration, $method, $name = null)
    {}

    /**
     * Pretend to run the migrations.
     *
     * @param  object  $migration
     * @param  string  $method
     * @return void
     */
    protected function pretendToRun($migration, $method)
    {}

    /**
     * Get all of the queries that would be run for a migration.
     *
     * @param  object  $migration
     * @param  string  $method
     * @return array
     */
    protected function getQueries($migration, $method)
    {}

    /**
     * Run a migration method on the given connection.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @param  object  $migration
     * @param  string  $method
     * @return void
     */
    protected function runMethod($connection, $migration, $method)
    {}

    /**
     * Resolve a migration instance from a file.
     *
     * @param  string  $file
     * @return object
     */
    public function resolve($file)
    {}

    /**
     * Resolve a migration instance from a migration path.
     *
     * @param  string  $path
     * @return object
     */
    protected function resolvePath(string $path)
    {}

    /**
     * Determine if the migration at the given path should be skipped.
     *
     * @param  string  $path
     * @return bool
     */
    public function shouldSkip($path)
    {}

    /**
     * Generate a migration class name based on the migration file name.
     *
     * @param  string  $migrationName
     * @return string
     */
    protected function getMigrationClass(string $migrationName): string
    {}

    /**
     * Get all of the migration files in a given path.
     *
     * @param  string|array  $paths
     * @return array<string, string>
     */
    public function getMigrationFiles($paths)
    {}

    /**
     * Require in all the migration files in a given path.
     *
     * @param  string[]  $files
     * @return void
     */
    public function requireFiles(array $files)
    {}

    /**
     * Get the name of the migration.
     *
     * @param  string  $path
     * @return string
     */
    public function getMigrationName($path)
    {}

    /**
     * Register a custom migration path.
     *
     * @param  string  $path
     * @return void
     */
    public function path($path)
    {}

    /**
     * Get all of the custom migration paths.
     *
     * @return string[]
     */
    public function paths()
    {}

    /**
     * Set the pending migrations to skip.
     *
     * @param  list<string>  $migrations
     * @return void
     */
    public static function withoutMigrations(array $migrations)
    {}

    /**
     * Get the default connection name.
     *
     * @return string|null
     */
    public function getConnection()
    {}

    /**
     * Execute the given callback using the given connection as the default connection.
     *
     * @template TReturn
     *
     * @param  string  $name
     * @param  (callable(): TReturn)  $callback
     * @return TReturn
     */
    public function usingConnection($name, callable $callback)
    {}

    /**
     * Set the default connection name.
     *
     * @param  string|null  $name
     * @return void
     */
    public function setConnection($name)
    {}

    /**
     * Resolve the database connection instance.
     *
     * @param  string|null  $connection
     * @return \Illuminate\Database\Connection
     */
    public function resolveConnection($connection)
    {}

    /**
     * Resolve the direct connection variant when one is configured.
     *
     * @param  string|null  $name
     * @return string
     */
    protected function directConnectionName($name)
    {}

    /**
     * Set a connection resolver callback.
     *
     * @param  \Closure(\Illuminate\Database\ConnectionResolverInterface, ?string): \Illuminate\Database\Connection  $callback
     * @return void
     */
    public static function resolveConnectionsUsing(Closure $callback)
    {}

    /**
     * Get the schema grammar out of a migration connection.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @return \Illuminate\Database\Schema\Grammars\Grammar
     */
    protected function getSchemaGrammar($connection)
    {}

    /**
     * Get the migration repository instance.
     *
     * @return \Illuminate\Database\Migrations\MigrationRepositoryInterface
     */
    public function getRepository()
    {}

    /**
     * Determine if the migration repository exists.
     *
     * @return bool
     */
    public function repositoryExists()
    {}

    /**
     * Determine if any migrations have been run.
     *
     * @return bool
     */
    public function hasRunAnyMigrations()
    {}

    /**
     * Delete the migration repository data store.
     *
     * @return void
     */
    public function deleteRepository()
    {}

    /**
     * Get the file system instance.
     *
     * @return \Illuminate\Filesystem\Filesystem
     */
    public function getFilesystem()
    {}

    /**
     * Set the output implementation that should be used by the console.
     *
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @return $this
     */
    public function setOutput(OutputInterface $output)
    {}

    /**
     * Write to the console's output.
     *
     * @param  string  $component
     * @param  array<int, string>|string  ...$arguments
     * @return void
     */
    protected function write($component, ...$arguments)
    {}

    /**
     * Fire the given event for the migration.
     *
     * @param  \Illuminate\Contracts\Database\Events\MigrationEvent  $event
     * @return void
     */
    public function fireMigrationEvent($event)
    {}
}
