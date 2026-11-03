<?php

namespace Illuminate\Database;

use Illuminate\Database\Query\Grammars\MariaDbGrammar as QueryGrammar;
use Illuminate\Database\Query\Processors\MariaDbProcessor;
use Illuminate\Database\Schema\Grammars\MariaDbGrammar as SchemaGrammar;
use Illuminate\Database\Schema\MariaDbBuilder;
use Illuminate\Database\Schema\MariaDbSchemaState;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Str;

class MariaDbConnection extends MySqlConnection
{
    /**
     * {@inheritdoc}
     */
    public function getDriverTitle()
    {}

    /**
     * Determine if the connected database is a MariaDB database.
     *
     * @return bool
     */
    public function isMaria()
    {}

    /**
     * Get the server version for the connection.
     *
     * @return string
     */
    public function getServerVersion(): string
    {}

    /**
     * Get the default query grammar instance.
     *
     * @return \Illuminate\Database\Query\Grammars\MariaDbGrammar
     */
    protected function getDefaultQueryGrammar()
    {}

    /**
     * Get a schema builder instance for the connection.
     *
     * @return \Illuminate\Database\Schema\MariaDbBuilder
     */
    public function getSchemaBuilder()
    {}

    /**
     * Get the default schema grammar instance.
     *
     * @return \Illuminate\Database\Schema\Grammars\MariaDbGrammar
     */
    protected function getDefaultSchemaGrammar()
    {}

    /**
     * Get the schema state for the connection.
     *
     * @param  \Illuminate\Filesystem\Filesystem|null  $files
     * @param  callable|null  $processFactory
     * @return \Illuminate\Database\Schema\MariaDbSchemaState
     */
    public function getSchemaState(?Filesystem $files = null, ?callable $processFactory = null)
    {}

    /**
     * Get the default post processor instance.
     *
     * @return \Illuminate\Database\Query\Processors\MariaDbProcessor
     */
    protected function getDefaultPostProcessor()
    {}
}
