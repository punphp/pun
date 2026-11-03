<?php

namespace Illuminate\Database\Schema;

use Exception;
use Illuminate\Database\Connection;
use Illuminate\Support\Str;
use Pdo\Mysql;
use Symfony\Component\Process\Exception\ProcessFailedException;
use Symfony\Component\Process\Process;

class MySqlSchemaState extends SchemaState
{
    /**
     * Dump the database's schema into a file.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @param  string  $path
     * @return void
     */
    public function dump(Connection $connection, $path)
    {}

    /**
     * Remove the auto-incrementing state from the given schema dump.
     *
     * @param  string  $path
     * @return void
     */
    protected function removeAutoIncrementingState(string $path)
    {}

    /**
     * Append the migration data to the schema dump.
     *
     * @param  string  $path
     * @return void
     */
    protected function appendMigrationData(string $path)
    {}

    /**
     * Load the given schema file into the database.
     *
     * @param  string  $path
     * @return void
     */
    public function load($path)
    {}

    /**
     * Get the base dump command arguments for MySQL as a string.
     *
     * @return string
     */
    protected function baseDumpCommand()
    {}

    /**
     * Generate a basic connection string (--socket, --host, --port, --user, --password) for the database.
     *
     * @param  array{version: string, isMariaDb: bool}  $versionInfo
     * @return string
     */
    protected function connectionString(array $versionInfo)
    {}

    /**
     * Get the base variables for a dump / load command.
     *
     * @param  array  $config
     * @return array
     */
    protected function baseVariables(array $config)
    {}

    /**
     * Execute the given dump process.
     *
     * @param  \Symfony\Component\Process\Process  $process
     * @param  callable  $output
     * @param  array  $variables
     * @param  int  $depth
     * @return \Symfony\Component\Process\Process
     *
     * @throws \Throwable
     */
    protected function executeDumpProcess(Process $process, $output, array $variables, int $depth = 0)
    {}

    /**
     * Detect the MySQL client version.
     *
     * @return array{version: string, isMariaDb: bool}
     */
    protected function detectClientVersion(): array
    {}
}
