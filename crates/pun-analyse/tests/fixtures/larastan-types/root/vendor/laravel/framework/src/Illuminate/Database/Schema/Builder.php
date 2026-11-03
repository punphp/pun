<?php

namespace Illuminate\Database\Schema;

use Closure;
use Illuminate\Container\Container;
use Illuminate\Database\Connection;
use Illuminate\Database\PostgresConnection;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use LogicException;
use RuntimeException;

class Builder
{
    use Macroable;

    /**
     * The database connection instance.
     *
     * @var \Illuminate\Database\Connection
     */
    protected $connection;

    /**
     * The schema grammar instance.
     *
     * @var \Illuminate\Database\Schema\Grammars\Grammar
     */
    protected $grammar;

    /**
     * The Blueprint resolver callback.
     *
     * @var (\Closure(\Illuminate\Database\Connection, string, \Closure|null): \Illuminate\Database\Schema\Blueprint)|null
     */
    protected $resolver;

    /**
     * The default string length for migrations.
     *
     * @var non-negative-int|null
     */
    public static $defaultStringLength = 255;

    /**
     * The default time precision for migrations.
     */
    public static ?int $defaultTimePrecision = 0;

    /**
     * The default relationship morph key type.
     *
     * @var 'int'|'uuid'|'ulid'
     */
    public static $defaultMorphKeyType = 'int';

    /**
     * Create a new database Schema manager.
     *
     * @param  \Illuminate\Database\Connection  $connection
     */
    public function __construct(Connection $connection)
    {}

    /**
     * Set the default string length for migrations.
     *
     * @param  non-negative-int  $length
     * @return void
     */
    public static function defaultStringLength($length)
    {}

    /**
     * Set the default time precision for migrations.
     */
    public static function defaultTimePrecision(?int $precision): void
    {}

    /**
     * Set the default morph key type for migrations.
     *
     * @param  string  $type
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    public static function defaultMorphKeyType(string $type)
    {}

    /**
     * Set the default morph key type for migrations to UUIDs.
     *
     * @return void
     */
    public static function morphUsingUuids()
    {}

    /**
     * Set the default morph key type for migrations to ULIDs.
     *
     * @return void
     */
    public static function morphUsingUlids()
    {}

    /**
     * Create a database in the schema.
     *
     * @param  string  $name
     * @return bool
     */
    public function createDatabase($name)
    {}

    /**
     * Drop a database from the schema if the database exists.
     *
     * @param  string  $name
     * @return bool
     */
    public function dropDatabaseIfExists($name)
    {}

    /**
     * Get the schemas that belong to the connection.
     *
     * @return list<array{name: string, path: string|null, default: bool}>
     */
    public function getSchemas()
    {}

    /**
     * Determine if the given table exists.
     *
     * @param  string  $table
     * @return bool
     */
    public function hasTable($table)
    {}

    /**
     * Determine if the given view exists.
     *
     * @param  string  $view
     * @return bool
     */
    public function hasView($view)
    {}

    /**
     * Get the tables that belong to the connection.
     *
     * @param  string|string[]|null  $schema
     * @return list<array{name: string, schema: string|null, schema_qualified_name: string, size: int|null, comment: string|null, collation: string|null, engine: string|null}>
     */
    public function getTables($schema = null)
    {}

    /**
     * Get the names of the tables that belong to the connection.
     *
     * @param  string|string[]|null  $schema
     * @param  bool  $schemaQualified
     * @return list<string>
     */
    public function getTableListing($schema = null, $schemaQualified = true)
    {}

    /**
     * Get the views that belong to the connection.
     *
     * @param  string|string[]|null  $schema
     * @return list<array{name: string, schema: string|null, schema_qualified_name: string, definition: string}>
     */
    public function getViews($schema = null)
    {}

    /**
     * Get the user-defined types that belong to the connection.
     *
     * @param  string|string[]|null  $schema
     * @return list<array{name: string, schema: string, schema_qualified_name: string, type: string, category: string, implicit: bool}>
     */
    public function getTypes($schema = null)
    {}

    /**
     * Determine if the given table has a given column.
     *
     * @param  string  $table
     * @param  string  $column
     * @return bool
     */
    public function hasColumn($table, $column)
    {}

    /**
     * Determine if the given table has given columns.
     *
     * @param  string  $table
     * @param  array<string>  $columns
     * @return bool
     */
    public function hasColumns($table, array $columns)
    {}

    /**
     * Execute a table builder callback if the given table has a given column.
     *
     * @param  string  $table
     * @param  string  $column
     * @param  \Closure  $callback
     * @return void
     */
    public function whenTableHasColumn(string $table, string $column, Closure $callback)
    {}

    /**
     * Execute a table builder callback if the given table doesn't have a given column.
     *
     * @param  string  $table
     * @param  string  $column
     * @param  \Closure  $callback
     * @return void
     */
    public function whenTableDoesntHaveColumn(string $table, string $column, Closure $callback)
    {}

    /**
     * Execute a table builder callback if the given table has a given index.
     *
     * @param  string  $table
     * @param  string|array  $index
     * @param  \Closure  $callback
     * @param  string|null  $type
     * @return void
     */
    public function whenTableHasIndex(string $table, string|array $index, Closure $callback, ?string $type = null)
    {}

    /**
     * Execute a table builder callback if the given table doesn't have a given index.
     *
     * @param  string  $table
     * @param  string|array  $index
     * @param  \Closure  $callback
     * @param  string|null  $type
     * @return void
     */
    public function whenTableDoesntHaveIndex(string $table, string|array $index, Closure $callback, ?string $type = null)
    {}

    /**
     * Get the data type for the given column name.
     *
     * @param  string  $table
     * @param  string  $column
     * @param  bool  $fullDefinition
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function getColumnType($table, $column, $fullDefinition = false)
    {}

    /**
     * Get the column listing for a given table.
     *
     * @param  string  $table
     * @return list<string>
     */
    public function getColumnListing($table)
    {}

    /**
     * Get the column for a given table.
     *
     * @param  string  $table
     * @param  string  $column
     * @return array{name: string, type: string, type_name: string, collation: string|null, nullable: bool, default: mixed, auto_increment: bool, comment: string|null, generation: array{type: string, expression: string|null}|null}
     */
    public function getColumn($table, $column)
    {}

    /**
     * Get the columns for a given table.
     *
     * @param  string  $table
     * @return list<array{name: string, type: string, type_name: string, collation: string|null, nullable: bool, default: mixed, auto_increment: bool, comment: string|null, generation: array{type: string, expression: string|null}|null}>
     */
    public function getColumns($table)
    {}

    /**
     * Get the indexes for a given table.
     *
     * @param  string  $table
     * @return list<array{name: string, columns: list<string>, type: string, unique: bool, primary: bool}>
     */
    public function getIndexes($table)
    {}

    /**
     * Get the names of the indexes for a given table.
     *
     * @param  string  $table
     * @return list<string>
     */
    public function getIndexListing($table)
    {}

    /**
     * Determine if the given table has a given index.
     *
     * @param  string  $table
     * @param  string|array  $index
     * @param  string|null  $type
     * @return bool
     */
    public function hasIndex($table, $index, $type = null)
    {}

    /**
     * Determine if the table has a given foreign key.
     *
     * @param  string  $table
     * @param  array|string  $foreignKey
     * @return bool
     */
    public function hasForeignKey($table, $foreignKey)
    {}

    /**
     * Get the foreign keys for a given table.
     *
     * @param  string  $table
     * @return list<array{name: string|null, columns: list<string>, foreign_schema: string|null, foreign_table: string, foreign_columns: list<string>, on_update: string|null, on_delete: string|null}>
     */
    public function getForeignKeys($table)
    {}

    /**
     * Modify a table on the schema.
     *
     * @param  string  $table
     * @param  \Closure  $callback
     * @return void
     */
    public function table($table, Closure $callback)
    {}

    /**
     * Create a new table on the schema.
     *
     * @param  string  $table
     * @param  \Closure  $callback
     * @return void
     */
    public function create($table, Closure $callback)
    {}

    /**
     * Drop a table from the schema.
     *
     * @param  string  $table
     * @return void
     */
    public function drop($table)
    {}

    /**
     * Drop a table from the schema if it exists.
     *
     * @param  string  $table
     * @return void
     */
    public function dropIfExists($table)
    {}

    /**
     * Drop columns from a table schema.
     *
     * @param  string  $table
     * @param  string|array<string>  $columns
     * @return void
     */
    public function dropColumns($table, $columns)
    {}

    /**
     * Drop all tables from the database.
     *
     * @return void
     *
     * @throws \LogicException
     */
    public function dropAllTables()
    {}

    /**
     * Drop all views from the database.
     *
     * @return void
     *
     * @throws \LogicException
     */
    public function dropAllViews()
    {}

    /**
     * Drop all types from the database.
     *
     * @return void
     *
     * @throws \LogicException
     */
    public function dropAllTypes()
    {}

    /**
     * Rename a table on the schema.
     *
     * @param  string  $from
     * @param  string  $to
     * @return void
     */
    public function rename($from, $to)
    {}

    /**
     * Enable foreign key constraints.
     *
     * @return bool
     */
    public function enableForeignKeyConstraints()
    {}

    /**
     * Disable foreign key constraints.
     *
     * @return bool
     */
    public function disableForeignKeyConstraints()
    {}

    /**
     * Disable foreign key constraints during the execution of a callback.
     *
     * @template TReturn
     *
     * @param  (\Closure(): TReturn)  $callback
     * @return TReturn
     */
    public function withoutForeignKeyConstraints(Closure $callback)
    {}

    /**
     * Create the vector extension on the schema if it does not exist.
     *
     * @param  string|null  $schema
     * @return void
     */
    public function ensureVectorExtensionExists($schema = null)
    {}

    /**
     * Create a new extension on the schema if it does not exist.
     *
     * @param  string  $name
     * @param  string|null  $schema
     * @return void
     *
     * @throws \RuntimeException
     */
    public function ensureExtensionExists($name, $schema = null)
    {}

    /**
     * Execute the blueprint to build / modify the table.
     *
     * @param  \Illuminate\Database\Schema\Blueprint  $blueprint
     * @return void
     */
    protected function build(Blueprint $blueprint)
    {}

    /**
     * Create a new command set with a Closure.
     *
     * @param  string  $table
     * @param  \Closure|null  $callback
     * @return \Illuminate\Database\Schema\Blueprint
     */
    protected function createBlueprint($table, ?Closure $callback = null)
    {}

    /**
     * Get the names of the current schemas for the connection.
     *
     * @return string[]|null
     */
    public function getCurrentSchemaListing()
    {}

    /**
     * Get the default schema name for the connection.
     *
     * @return string|null
     */
    public function getCurrentSchemaName()
    {}

    /**
     * Parse the given database object reference and extract the schema and table.
     *
     * @param  string  $reference
     * @param  string|bool|null  $withDefaultSchema
     * @return array{string|null, string}
     *
     * @throws \InvalidArgumentException
     */
    public function parseSchemaAndTable($reference, $withDefaultSchema = null)
    {}

    /**
     * Get the database connection instance.
     *
     * @return \Illuminate\Database\Connection
     */
    public function getConnection()
    {}

    /**
     * Set the Schema Blueprint resolver callback.
     *
     * @param  \Closure(\Illuminate\Database\Connection, string, \Closure|null): \Illuminate\Database\Schema\Blueprint  $resolver
     * @return void
     */
    public function blueprintResolver(Closure $resolver)
    {}
}
