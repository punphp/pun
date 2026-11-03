<?php

namespace Illuminate\Database;

use Closure;
use Exception;
use Illuminate\Database\Query\Grammars\SqlServerGrammar as QueryGrammar;
use Illuminate\Database\Query\Processors\SqlServerProcessor;
use Illuminate\Database\Schema\Grammars\SqlServerGrammar as SchemaGrammar;
use Illuminate\Database\Schema\SqlServerBuilder;
use Illuminate\Filesystem\Filesystem;
use RuntimeException;
use Throwable;

class SqlServerConnection extends Connection
{
    /**
     * {@inheritdoc}
     */
    public function getDriverTitle()
    {}

    /**
     * Execute a Closure within a transaction.
     *
     * @template TReturn
     *
     * @param  (\Closure(static): TReturn)  $callback
     * @param  int  $attempts
     * @return TReturn
     *
     * @throws \Throwable
     */
    public function transaction(Closure $callback, $attempts = 1)
    {}

    /**
     * Escape a binary value for safe SQL embedding.
     *
     * @param  string  $value
     * @return string
     */
    protected function escapeBinary($value)
    {}

    /**
     * Determine if the given database exception was caused by a unique constraint violation.
     *
     * @param  \Exception  $exception
     * @return bool
     */
    protected function isUniqueConstraintError(Exception $exception): bool
    {}

    /**
     * Extract the index that caused a unique constraint violation.
     *
     * @param  Exception  $exception
     * @return array{index: string|null, columns: list<string>}
     */
    protected function parseUniqueConstraintViolation(Exception $exception): array
    {}

    /**
     * Get the default query grammar instance.
     *
     * @return \Illuminate\Database\Query\Grammars\SqlServerGrammar
     */
    protected function getDefaultQueryGrammar()
    {}

    /**
     * Get a schema builder instance for the connection.
     *
     * @return \Illuminate\Database\Schema\SqlServerBuilder
     */
    public function getSchemaBuilder()
    {}

    /**
     * Get the default schema grammar instance.
     *
     * @return \Illuminate\Database\Schema\Grammars\SqlServerGrammar
     */
    protected function getDefaultSchemaGrammar()
    {}

    /**
     * Get the schema state for the connection.
     *
     * @param  \Illuminate\Filesystem\Filesystem|null  $files
     * @param  callable|null  $processFactory
     *
     * @throws \RuntimeException
     */
    public function getSchemaState(?Filesystem $files = null, ?callable $processFactory = null)
    {}

    /**
     * Get the default post processor instance.
     *
     * @return \Illuminate\Database\Query\Processors\SqlServerProcessor
     */
    protected function getDefaultPostProcessor()
    {}
}
