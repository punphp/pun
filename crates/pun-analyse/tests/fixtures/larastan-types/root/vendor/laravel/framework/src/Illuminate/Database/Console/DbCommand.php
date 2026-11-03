<?php

namespace Illuminate\Database\Console;

use Illuminate\Console\Command;
use Illuminate\Support\Arr;
use Illuminate\Support\ConfigurationUrlParser;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Process\Exception\ProcessFailedException;
use Symfony\Component\Process\Process;
use UnexpectedValueException;

#[AsCommand(name: 'db')]
class DbCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'db {connection? : The database connection that should be used}
               {--read : Connect to the read connection}
               {--write : Connect to the write connection}
               {--pooled : Connect to the pooled connection}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Start a new database CLI session';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Get the database connection configuration.
     *
     * @return array
     *
     * @throws \UnexpectedValueException
     */
    public function getConnection()
    {}

    /**
     * Merge a nested connection configuration onto the base connection.
     *
     * @param  array  $connection
     * @param  string  $type
     * @return array
     */
    protected function mergeConnectionConfiguration(array $connection, $type)
    {}

    /**
     * Get the arguments for the database client command.
     *
     * @param  array  $connection
     * @return array
     */
    public function commandArguments(array $connection)
    {}

    /**
     * Get the environment variables for the database client command.
     *
     * @param  array  $connection
     * @return array|null
     */
    public function commandEnvironment(array $connection)
    {}

    /**
     * Get the database client command to run.
     *
     * @param  array  $connection
     * @return string
     */
    public function getCommand(array $connection)
    {}

    /**
     * Get the arguments for the MySQL CLI.
     *
     * @param  array  $connection
     * @return array
     */
    protected function getMysqlArguments(array $connection)
    {}

    /**
     * Get the arguments for the MariaDB CLI.
     *
     * @param  array  $connection
     * @return array
     */
    protected function getMariaDbArguments(array $connection)
    {}

    /**
     * Get the arguments for the Postgres CLI.
     *
     * @param  array  $connection
     * @return array
     */
    protected function getPgsqlArguments(array $connection)
    {}

    /**
     * Get the arguments for the SQLite CLI.
     *
     * @param  array  $connection
     * @return array
     */
    protected function getSqliteArguments(array $connection)
    {}

    /**
     * Get the arguments for the SQL Server CLI.
     *
     * @param  array  $connection
     * @return array
     */
    protected function getSqlsrvArguments(array $connection)
    {}

    /**
     * Get the environment variables for the Postgres CLI.
     *
     * @param  array  $connection
     * @return array|null
     */
    protected function getPgsqlEnvironment(array $connection)
    {}

    /**
     * Get the optional arguments based on the connection configuration.
     *
     * @param  array  $args
     * @param  array  $connection
     * @return array
     */
    protected function getOptionalArguments(array $args, array $connection)
    {}
}
