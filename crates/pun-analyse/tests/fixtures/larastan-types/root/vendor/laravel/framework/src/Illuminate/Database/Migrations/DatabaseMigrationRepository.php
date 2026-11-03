<?php

namespace Illuminate\Database\Migrations;

use Illuminate\Database\ConnectionResolverInterface as Resolver;

class DatabaseMigrationRepository implements MigrationRepositoryInterface
{
    /**
     * The database connection resolver instance.
     *
     * @var \Illuminate\Database\ConnectionResolverInterface
     */
    protected $resolver;

    /**
     * The name of the migration table.
     *
     * @var string
     */
    protected $table;

    /**
     * The name of the database connection to use.
     *
     * @var string
     */
    protected $connection;

    /**
     * Create a new database migration repository instance.
     *
     * @param  \Illuminate\Database\ConnectionResolverInterface  $resolver
     * @param  string  $table
     */
    public function __construct(Resolver $resolver, $table)
    {}

    /**
     * Get the completed migrations.
     *
     * @return string[]
     */
    public function getRan()
    {}

    /**
     * Get the list of migrations.
     *
     * @param  int  $steps
     * @return object{id: int, migration: string, batch: int}[]
     */
    public function getMigrations($steps)
    {}

    /**
     * Get the list of the migrations by batch number.
     *
     * @param  int  $batch
     * @return object{id: int, migration: string, batch: int}[]
     */
    public function getMigrationsByBatch($batch)
    {}

    /**
     * Get the last migration batch.
     *
     * @return object{id: int, migration: string, batch: int}[]
     */
    public function getLast()
    {}

    /**
     * Get the completed migrations with their batch numbers.
     *
     * @return array<string, int>
     */
    public function getMigrationBatches()
    {}

    /**
     * Log that a migration was run.
     *
     * @param  string  $file
     * @param  int  $batch
     * @return void
     */
    public function log($file, $batch)
    {}

    /**
     * Remove a migration from the log.
     *
     * @param  object{id?: int, migration: string, batch?: int}  $migration
     * @return void
     */
    public function delete($migration)
    {}

    /**
     * Get the next migration batch number.
     *
     * @return int
     */
    public function getNextBatchNumber()
    {}

    /**
     * Get the last migration batch number.
     *
     * @return int
     */
    public function getLastBatchNumber()
    {}

    /**
     * Create the migration repository data store.
     *
     * @return void
     */
    public function createRepository()
    {}

    /**
     * Determine if the migration repository exists.
     *
     * @return bool
     */
    public function repositoryExists()
    {}

    /**
     * Delete the migration repository data store.
     *
     * @return void
     */
    public function deleteRepository()
    {}

    /**
     * Get a query builder for the migration table.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    protected function table()
    {}

    /**
     * Get the connection resolver instance.
     *
     * @return \Illuminate\Database\ConnectionResolverInterface
     */
    public function getConnectionResolver()
    {}

    /**
     * Resolve the database connection instance.
     *
     * @return \Illuminate\Database\Connection
     */
    public function getConnection()
    {}

    /**
     * Set the information source to gather data.
     *
     * @param  string  $name
     * @return void
     */
    public function setSource($name)
    {}
}
