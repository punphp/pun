<?php

namespace Illuminate\Database;

use DateTimeInterface;
use Exception;
use Illuminate\Database\Query\Grammars\PostgresGrammar as QueryGrammar;
use Illuminate\Database\Query\Processors\PostgresProcessor;
use Illuminate\Database\Schema\Grammars\PostgresGrammar as SchemaGrammar;
use Illuminate\Database\Schema\PostgresBuilder;
use Illuminate\Database\Schema\PostgresSchemaState;
use Illuminate\Filesystem\Filesystem;
use PDO;
use RuntimeException;

class PostgresConnection extends Connection
{
    /**
     * {@inheritdoc}
     */
    public function getDriverTitle()
    {}

    /**
     * Prepare the query bindings for execution.
     *
     * @param  array  $bindings
     * @return array
     */
    public function prepareBindings(array $bindings)
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
     * Escape a bool value for safe SQL embedding.
     *
     * @param  bool  $value
     * @return string
     */
    protected function escapeBool($value)
    {}

    /**
     * Determine if the given database exception was caused by a unique constraint violation.
     *
     * @param  \Exception  $exception
     * @return bool
     */
    protected function isUniqueConstraintError(Exception $exception)
    {}

    /**
     * Extract the index and columns that caused a unique constraint violation.
     *
     * @param  Exception  $exception
     * @return array{index: string|null, columns: list<string>}
     */
    protected function parseUniqueConstraintViolation(Exception $exception): array
    {}

    /**
     * Get the ID of the last committed transaction.
     *
     * @return string
     */
    public function getLastCommitId()
    {}

    /**
     * Calculate the commit ID for a set of commit IDs that includes all of them.
     *
     * @param  string[]  $values
     * @return string
     *
     * @throws \RuntimeException
     */
    public function mergeCommitIds(array $values)
    {}

    /**
     * Determine if the given commit ID has been applied.
     *
     * @param  string  $value
     * @param  int  $wait
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function hasCommitId(string $value, int $wait = 0)
    {}

    /**
     * Get the default query grammar instance.
     *
     * @return \Illuminate\Database\Query\Grammars\PostgresGrammar
     */
    protected function getDefaultQueryGrammar()
    {}

    /**
     * Get a schema builder instance for the connection.
     *
     * @return \Illuminate\Database\Schema\PostgresBuilder
     */
    public function getSchemaBuilder()
    {}

    /**
     * Get the default schema grammar instance.
     *
     * @return \Illuminate\Database\Schema\Grammars\PostgresGrammar
     */
    protected function getDefaultSchemaGrammar()
    {}

    /**
     * Get the schema state for the connection.
     *
     * @param  \Illuminate\Filesystem\Filesystem|null  $files
     * @param  callable|null  $processFactory
     * @return \Illuminate\Database\Schema\PostgresSchemaState
     */
    public function getSchemaState(?Filesystem $files = null, ?callable $processFactory = null)
    {}

    /**
     * Get the default post processor instance.
     *
     * @return \Illuminate\Database\Query\Processors\PostgresProcessor
     */
    protected function getDefaultPostProcessor()
    {}

    /**
     * Determine if the active PDO configuration uses emulated prepares.
     *
     * @return bool
     */
    protected function usesEmulatedPrepares()
    {}
}
