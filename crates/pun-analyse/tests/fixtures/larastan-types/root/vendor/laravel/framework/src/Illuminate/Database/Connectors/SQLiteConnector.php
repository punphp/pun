<?php

namespace Illuminate\Database\Connectors;

use Illuminate\Database\SQLiteDatabaseDoesNotExistException;

class SQLiteConnector extends Connector implements ConnectorInterface
{
    /**
     * Establish a database connection.
     *
     * @param  array  $config
     * @return \PDO
     */
    public function connect(array $config)
    {}

    /**
     * Get the absolute database path.
     *
     * @param  string  $path
     * @return string
     *
     * @throws \Illuminate\Database\SQLiteDatabaseDoesNotExistException
     */
    protected function parseDatabasePath(string $path): string
    {}

    /**
     * Set miscellaneous user-configured pragmas.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configurePragmas($connection, array $config): void
    {}

    /**
     * Enable or disable foreign key constraints if configured.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureForeignKeyConstraints($connection, array $config): void
    {}

    /**
     * Set the busy timeout if configured.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureBusyTimeout($connection, array $config): void
    {}

    /**
     * Set the journal mode if configured.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureJournalMode($connection, array $config): void
    {}

    /**
     * Set the synchronous mode if configured.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureSynchronous($connection, array $config): void
    {}
}
