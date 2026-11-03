<?php

namespace Illuminate\Foundation\Testing;

use Illuminate\Contracts\Console\Kernel;
use Illuminate\Database\ConnectionInterface;
use Illuminate\Foundation\Testing\Traits\CanConfigureMigrationCommands;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;

trait DatabaseTruncation
{
    use CanConfigureMigrationCommands;

    /**
     * The cached names of the database tables for each connection.
     *
     * @var array
     */
    protected static array $allTables;

    /**
     * Truncate the database tables for all configured connections.
     *
     * @return void
     */
    protected function truncateDatabaseTables(): void
    {}

    /**
     * Truncate the database tables for all configured connections.
     *
     * @return void
     */
    protected function truncateTablesForAllConnections(): void
    {}

    /**
     * Truncate the database tables for the given database connection.
     *
     * @param  \Illuminate\Database\ConnectionInterface  $connection
     * @param  string|null  $name
     * @return void
     */
    protected function truncateTablesForConnection(ConnectionInterface $connection, ?string $name): void
    {}

    /**
     * Get all the tables that belong to the connection.
     */
    protected function getAllTablesForConnection(ConnectionInterface $connection, ?string $name): array
    {}

    /**
     * Determine if a table exists in the given list, with or without its schema.
     */
    protected function tableExistsIn(array $table, array $tables): bool
    {}

    /**
     * The database connections that should have their tables truncated.
     *
     * @return array
     */
    protected function connectionsToTruncate(): array
    {}

    /**
     * Get the tables that should be truncated.
     */
    protected function tablesToTruncate(ConnectionInterface $connection, ?string $connectionName): ?array
    {}

    /**
     * Get the tables that should not be truncated.
     */
    protected function exceptTables(ConnectionInterface $connection, ?string $connectionName): array
    {}

    /**
     * Perform any work that should take place before the database has started truncating.
     *
     * @return void
     */
    protected function beforeTruncatingDatabase(): void
    {}

    /**
     * Perform any work that should take place once the database has finished truncating.
     *
     * @return void
     */
    protected function afterTruncatingDatabase(): void
    {}
}
