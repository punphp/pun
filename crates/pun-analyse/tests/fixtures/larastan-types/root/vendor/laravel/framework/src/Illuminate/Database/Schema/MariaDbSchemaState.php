<?php

namespace Illuminate\Database\Schema;

use Symfony\Component\Process\Exception\ProcessFailedException;

class MariaDbSchemaState extends MySqlSchemaState
{
    /**
     * Load the given schema file into the database.
     *
     * @param  string  $path
     * @return void
     */
    public function load($path)
    {}

    /**
     * Get the base dump command arguments for MariaDB as a string.
     *
     * @return string
     */
    protected function baseDumpCommand()
    {}

    /**
     * Detect the MariaDB client version.
     *
     * @return array{version: string, isMariaDb: bool}
     */
    protected function detectClientVersion(): array
    {}
}
