<?php

namespace Illuminate\Database\Schema\Grammars;

use Illuminate\Contracts\Database\Query\Expression;
use Illuminate\Database\Concerns\CompilesJsonPaths;
use Illuminate\Database\Grammar as BaseGrammar;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Fluent;
use RuntimeException;
use UnitEnum;

use function Illuminate\Support\enum_value;

abstract class Grammar extends BaseGrammar
{
    use CompilesJsonPaths;

    /**
     * The possible column modifiers.
     *
     * @var string[]
     */
    protected $modifiers = [];

    /**
     * If this Grammar supports schema changes wrapped in a transaction.
     *
     * @var bool
     */
    protected $transactions = false;

    /**
     * The commands to be executed outside of create or alter command.
     *
     * @var array
     */
    protected $fluentCommands = [];

    /**
     * Compile a create database command.
     *
     * @param  string  $name
     * @return string
     */
    public function compileCreateDatabase($name)
    {}

    /**
     * Compile a drop database if exists command.
     *
     * @param  string  $name
     * @return string
     */
    public function compileDropDatabaseIfExists($name)
    {}

    /**
     * Compile the query to determine the schemas.
     *
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileSchemas()
    {}

    /**
     * Compile the query to determine if the given table exists.
     *
     * @param  string|null  $schema
     * @param  string  $table
     * @return string|null
     */
    public function compileTableExists($schema, $table)
    {}

    /**
     * Compile the query to determine the tables.
     *
     * @param  string|string[]|null  $schema
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileTables($schema)
    {}

    /**
     * Compile the query to determine the views.
     *
     * @param  string|string[]|null  $schema
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileViews($schema)
    {}

    /**
     * Compile the query to determine the user-defined types.
     *
     * @param  string|string[]|null  $schema
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileTypes($schema)
    {}

    /**
     * Compile the query to determine the columns.
     *
     * @param  string|null  $schema
     * @param  string  $table
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileColumns($schema, $table)
    {}

    /**
     * Compile the query to determine the indexes.
     *
     * @param  string|null  $schema
     * @param  string  $table
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileIndexes($schema, $table)
    {}

    /**
     * Compile a vector index key command.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return void
     *
     * @throws \RuntimeException
     */
    public function compileVectorIndex(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile a drop vector index command.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return void
     *
     * @throws \RuntimeException
     */
    public function compileDropVectorIndex(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile the query to determine the foreign keys.
     *
     * @param  string|null  $schema
     * @param  string  $table
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileForeignKeys($schema, $table)
    {}

    /**
     * Compile a rename column command.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return list<string>|string
     */
    public function compileRenameColumn(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile a change column command into a series of SQL statements.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return list<string>|string
     *
     * @throws \RuntimeException
     */
    public function compileChange(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile a fulltext index key command.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileFulltext(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile a drop fulltext index command.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileDropFullText(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile a foreign key command.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return string
     */
    public function compileForeign(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile a drop foreign key command.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $command
     * @return string
     *
     * @throws \RuntimeException
     */
    public function compileDropForeign(Blueprint $blueprint, Fluent $command)
    {}

    /**
     * Compile the blueprint's added column definitions.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @return array
     */
    protected function getColumns(Blueprint $blueprint)
    {}

    /**
     * Compile the column definition.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Database\Schema\ColumnDefinition  $column
     * @return string
     */
    protected function getColumn(Blueprint $blueprint, $column)
    {}

    /**
     * Get the SQL for the column data type.
     *
     * @param  \Illuminate\Support\Fluent  $column
     * @return string
     */
    protected function getType(Fluent $column)
    {}

    /**
     * Create the column definition for a generated, computed column type.
     *
     * @param  \Illuminate\Support\Fluent  $column
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function typeComputed(Fluent $column)
    {}

    /**
     * Create the column definition for a vector type.
     *
     * @param  \Illuminate\Support\Fluent  $column
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function typeVector(Fluent $column)
    {}

    /**
     * Create the column definition for a tsvector type.
     *
     * @param  \Illuminate\Support\Fluent  $column
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function typeTsvector(Fluent $column)
    {}

    /**
     * Create the column definition for a raw column type.
     *
     * @param  \Illuminate\Support\Fluent  $column
     * @return string
     */
    protected function typeRaw(Fluent $column)
    {}

    /**
     * Add the column modifiers to the definition.
     *
     * @param  string  $sql
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  \Illuminate\Support\Fluent  $column
     * @return string
     */
    protected function addModifiers($sql, Blueprint $blueprint, Fluent $column)
    {}

    /**
     * Get the command with a given name if it exists on the blueprint.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  string  $name
     * @return \Illuminate\Support\Fluent|null
     */
    protected function getCommandByName(Blueprint $blueprint, $name)
    {}

    /**
     * Get all of the commands with a given name.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  string  $name
     * @return array
     */
    protected function getCommandsByName(Blueprint $blueprint, $name)
    {}

    /**
     * Determine if a command with a given name exists on the blueprint.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @param  string  $name
     * @return bool
     */
    protected function hasCommand(Blueprint $blueprint, $name)
    {}

    /**
     * Add a prefix to an array of values.
     *
     * @param  string  $prefix
     * @param  array<string>  $values
     * @return array<string>
     */
    public function prefixArray($prefix, array $values)
    {}

    /**
     * Wrap a table in keyword identifiers.
     *
     * @param  mixed  $table
     * @param  string|null  $prefix
     * @return string
     */
    public function wrapTable($table, $prefix = null)
    {}

    /**
     * Wrap a value in keyword identifiers.
     *
     * @param  \Illuminate\Support\Fluent|\Illuminate\Contracts\Database\Query\Expression|string  $value
     * @return string
     */
    public function wrap($value)
    {}

    /**
     * Format a value so that it can be used in "default" clauses.
     *
     * @param  mixed  $value
     * @return string
     */
    protected function getDefaultValue($value)
    {}

    /**
     * Get the fluent commands for the grammar.
     *
     * @return array
     */
    public function getFluentCommands()
    {}

    /**
     * Check if this Grammar supports schema changes wrapped in a transaction.
     *
     * @return bool
     */
    public function supportsSchemaTransactions()
    {}
}
