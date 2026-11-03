<?php

namespace Illuminate\Database\Schema;

use Closure;
use Illuminate\Database\Connection;
use Illuminate\Database\Eloquent\Concerns\HasUlids;
use Illuminate\Database\Query\Expression;
use Illuminate\Database\Schema\Grammars\Grammar;
use Illuminate\Database\Schema\Grammars\MariaDbGrammar;
use Illuminate\Database\Schema\Grammars\MySqlGrammar;
use Illuminate\Database\Schema\Grammars\SQLiteGrammar;
use Illuminate\Support\Collection;
use Illuminate\Support\Fluent;
use Illuminate\Support\Traits\Macroable;

use function Illuminate\Support\enum_value;

class Blueprint
{
    use Macroable;

    /**
     * The database connection instance.
     */
    protected Connection $connection;

    /**
     * The schema grammar instance.
     */
    protected Grammar $grammar;

    /**
     * The table the blueprint describes.
     *
     * @var string
     */
    protected $table;

    /**
     * The columns that should be added to the table.
     *
     * @var \Illuminate\Database\Schema\ColumnDefinition[]
     */
    protected $columns = [];

    /**
     * The commands that should be run for the table.
     *
     * @var \Illuminate\Support\Fluent[]
     */
    protected $commands = [];

    /**
     * The storage engine that should be used for the table.
     *
     * @var string|null
     */
    public $engine;

    /**
     * The default character set that should be used for the table.
     *
     * @var string|null
     */
    public $charset;

    /**
     * The collation that should be used for the table.
     *
     * @var string|null
     */
    public $collation;

    /**
     * Whether to make the table temporary.
     *
     * @var bool
     */
    public $temporary = false;

    /**
     * The column to add new columns after.
     *
     * @var string
     */
    public $after;

    /**
     * The blueprint state instance.
     *
     * @var \Illuminate\Database\Schema\BlueprintState|null
     */
    protected $state;

    /**
     * Create a new schema blueprint.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @param  string  $table
     * @param  (\Closure(self): void)|null  $callback
     */
    public function __construct(Connection $connection, $table, ?Closure $callback = null)
    {}

    /**
     * Execute the blueprint against the database.
     *
     * @return void
     */
    public function build()
    {}

    /**
     * Get the raw SQL statements for the blueprint.
     *
     * @return array
     */
    public function toSql()
    {}

    /**
     * Ensure the commands on the blueprint are valid for the connection type.
     *
     * @return void
     *
     * @throws \BadMethodCallException
     */
    protected function ensureCommandsAreValid()
    {}

    /**
     * Get all of the commands matching the given names.
     *
     * @deprecated Will be removed in a future Laravel version.
     *
     * @param  array  $names
     * @return \Illuminate\Support\Collection
     */
    protected function commandsNamed(array $names)
    {}

    /**
     * Add the commands that are implied by the blueprint's state.
     *
     * @return void
     */
    protected function addImpliedCommands()
    {}

    /**
     * Add the index commands fluently specified on columns.
     *
     * @return void
     */
    protected function addFluentIndexes()
    {}

    /**
     * Add the fluent commands specified on any columns.
     *
     * @return void
     */
    public function addFluentCommands()
    {}

    /**
     * Add the alter commands if whenever needed.
     *
     * @return void
     */
    public function addAlterCommands()
    {}

    /**
     * Determine if the blueprint has a create command.
     *
     * @return bool
     */
    public function creating()
    {}

    /**
     * Indicate that the table needs to be created.
     *
     * @return \Illuminate\Support\Fluent
     */
    public function create()
    {}

    /**
     * Specify the storage engine that should be used for the table.
     *
     * @param  string  $engine
     * @return void
     */
    public function engine($engine)
    {}

    /**
     * Specify that the InnoDB storage engine should be used for the table (MySQL only).
     *
     * @return void
     */
    public function innoDb()
    {}

    /**
     * Specify the character set that should be used for the table.
     *
     * @param  string  $charset
     * @return void
     */
    public function charset($charset)
    {}

    /**
     * Specify the collation that should be used for the table.
     *
     * @param  string  $collation
     * @return void
     */
    public function collation($collation)
    {}

    /**
     * Indicate that the table needs to be temporary.
     *
     * @return void
     */
    public function temporary()
    {}

    /**
     * Indicate that the table should be dropped.
     *
     * @return \Illuminate\Support\Fluent
     */
    public function drop()
    {}

    /**
     * Indicate that the table should be dropped if it exists.
     *
     * @return \Illuminate\Support\Fluent
     */
    public function dropIfExists()
    {}

    /**
     * Indicate that the given columns should be dropped.
     *
     * @param  mixed  $columns
     * @return \Illuminate\Support\Fluent
     */
    public function dropColumn($columns)
    {}

    /**
     * Indicate that the given columns should be renamed.
     *
     * @param  string  $from
     * @param  string  $to
     * @return \Illuminate\Support\Fluent
     */
    public function renameColumn($from, $to)
    {}

    /**
     * Indicate that the given primary key should be dropped.
     *
     * @param  string|array|null  $index
     * @return \Illuminate\Support\Fluent
     */
    public function dropPrimary($index = null)
    {}

    /**
     * Indicate that the given unique key should be dropped.
     *
     * @param  string|array  $index
     * @return \Illuminate\Support\Fluent
     */
    public function dropUnique($index)
    {}

    /**
     * Indicate that the given index should be dropped.
     *
     * @param  string|array  $index
     * @return \Illuminate\Support\Fluent
     */
    public function dropIndex($index)
    {}

    /**
     * Indicate that the given fulltext index should be dropped.
     *
     * @param  string|array  $index
     * @return \Illuminate\Support\Fluent
     */
    public function dropFullText($index)
    {}

    /**
     * Indicate that the given spatial index should be dropped.
     *
     * @param  string|array  $index
     * @return \Illuminate\Support\Fluent
     */
    public function dropSpatialIndex($index)
    {}

    /**
     * Indicate that the given vector index should be dropped.
     *
     * @param  string|array  $index
     * @return \Illuminate\Support\Fluent
     */
    public function dropVectorIndex($index)
    {}

    /**
     * Indicate that the given foreign key should be dropped.
     *
     * @param  string|array  $index
     * @return \Illuminate\Support\Fluent
     */
    public function dropForeign($index)
    {}

    /**
     * Indicate that the given column and foreign key should be dropped.
     *
     * @param  string  $column
     * @return \Illuminate\Support\Fluent
     */
    public function dropConstrainedForeignId($column)
    {}

    /**
     * Indicate that the given foreign key should be dropped.
     *
     * @param  \Illuminate\Database\Eloquent\Model|string  $model
     * @param  string|null  $column
     * @return \Illuminate\Support\Fluent
     */
    public function dropForeignIdFor($model, $column = null)
    {}

    /**
     * Indicate that the given foreign key should be dropped.
     *
     * @param  \Illuminate\Database\Eloquent\Model|string  $model
     * @param  string|null  $column
     * @return \Illuminate\Support\Fluent
     */
    public function dropConstrainedForeignIdFor($model, $column = null)
    {}

    /**
     * Indicate that the given indexes should be renamed.
     *
     * @param  string  $from
     * @param  string  $to
     * @return \Illuminate\Support\Fluent
     */
    public function renameIndex($from, $to)
    {}

    /**
     * Indicate that the timestamp columns should be dropped.
     *
     * @return void
     */
    public function dropTimestamps()
    {}

    /**
     * Indicate that the timestamp columns should be dropped.
     *
     * @return void
     */
    public function dropTimestampsTz()
    {}

    /**
     * Indicate that the soft delete column should be dropped.
     *
     * @param  string  $column
     * @return void
     */
    public function dropSoftDeletes($column = 'deleted_at')
    {}

    /**
     * Indicate that the soft delete column should be dropped.
     *
     * @param  string  $column
     * @return void
     */
    public function dropSoftDeletesTz($column = 'deleted_at')
    {}

    /**
     * Indicate that the remember token column should be dropped.
     *
     * @return void
     */
    public function dropRememberToken()
    {}

    /**
     * Indicate that the polymorphic columns should be dropped.
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @return void
     */
    public function dropMorphs($name, $indexName = null)
    {}

    /**
     * Rename the table to a given name.
     *
     * @param  string  $to
     * @return \Illuminate\Support\Fluent
     */
    public function rename($to)
    {}

    /**
     * Specify the primary key(s) for the table.
     *
     * @param  string|array  $columns
     * @param  string|null  $name
     * @param  string|null  $algorithm
     * @return \Illuminate\Database\Schema\IndexDefinition
     */
    public function primary($columns, $name = null, $algorithm = null)
    {}

    /**
     * Specify a unique index for the table.
     *
     * @param  string|array  $columns
     * @param  string|null  $name
     * @param  string|null  $algorithm
     * @return \Illuminate\Database\Schema\IndexDefinition
     */
    public function unique($columns, $name = null, $algorithm = null)
    {}

    /**
     * Specify an index for the table.
     *
     * @param  string|array  $columns
     * @param  string|null  $name
     * @param  string|null  $algorithm
     * @return \Illuminate\Database\Schema\IndexDefinition
     */
    public function index($columns, $name = null, $algorithm = null)
    {}

    /**
     * Specify a fulltext index for the table.
     *
     * @param  string|array  $columns
     * @param  string|null  $name
     * @param  string|null  $algorithm
     * @return \Illuminate\Database\Schema\IndexDefinition
     */
    public function fullText($columns, $name = null, $algorithm = null)
    {}

    /**
     * Specify a spatial index for the table.
     *
     * @param  string|array  $columns
     * @param  string|null  $name
     * @param  string|null  $operatorClass
     * @return \Illuminate\Database\Schema\IndexDefinition
     */
    public function spatialIndex($columns, $name = null, $operatorClass = null)
    {}

    /**
     * Specify a vector index for the table.
     *
     * @param  string  $column
     * @param  string|null  $name
     * @return \Illuminate\Database\Schema\IndexDefinition
     */
    public function vectorIndex($column, $name = null)
    {}

    /**
     * Specify a raw index for the table.
     *
     * @param  string  $expression
     * @param  string  $name
     * @return \Illuminate\Database\Schema\IndexDefinition
     */
    public function rawIndex($expression, $name)
    {}

    /**
     * Specify a foreign key for the table.
     *
     * @param  string|array  $columns
     * @param  string|null  $name
     * @return \Illuminate\Database\Schema\ForeignKeyDefinition
     */
    public function foreign($columns, $name = null)
    {}

    /**
     * Create a new auto-incrementing big integer column on the table (8-byte, 0 to 18,446,744,073,709,551,615).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function id($column = 'id')
    {}

    /**
     * Create a new auto-incrementing integer column on the table (4-byte, 0 to 4,294,967,295).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function increments($column)
    {}

    /**
     * Create a new auto-incrementing integer column on the table (4-byte, 0 to 4,294,967,295).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function integerIncrements($column)
    {}

    /**
     * Create a new auto-incrementing tiny integer column on the table (1-byte, 0 to 255).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function tinyIncrements($column)
    {}

    /**
     * Create a new auto-incrementing small integer column on the table (2-byte, 0 to 65,535).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function smallIncrements($column)
    {}

    /**
     * Create a new auto-incrementing medium integer column on the table (3-byte, 0 to 16,777,215).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function mediumIncrements($column)
    {}

    /**
     * Create a new auto-incrementing big integer column on the table (8-byte, 0 to 18,446,744,073,709,551,615).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function bigIncrements($column)
    {}

    /**
     * Create a new char column on the table.
     *
     * @param  string  $column
     * @param  int|null  $length
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function char($column, $length = null)
    {}

    /**
     * Create a new string column on the table.
     *
     * @param  string  $column
     * @param  int|null  $length
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function string($column, $length = null)
    {}

    /**
     * Create a new tiny text column on the table (up to 255 characters).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function tinyText($column)
    {}

    /**
     * Create a new text column on the table (up to 65,535 characters / ~64 KB).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function text($column)
    {}

    /**
     * Create a new medium text column on the table (up to 16,777,215 characters / ~16 MB).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function mediumText($column)
    {}

    /**
     * Create a new long text column on the table (up to 4,294,967,295 characters / ~4 GB).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function longText($column)
    {}

    /**
     * Create a new integer (4-byte) column on the table.
     * Range: -2,147,483,648 to 2,147,483,647 (signed) or 0 to 4,294,967,295 (unsigned).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @param  bool  $unsigned
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function integer($column, $autoIncrement = false, $unsigned = false)
    {}

    /**
     * Create a new tiny integer (1-byte) column on the table.
     * Range: -128 to 127 (signed) or 0 to 255 (unsigned).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @param  bool  $unsigned
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function tinyInteger($column, $autoIncrement = false, $unsigned = false)
    {}

    /**
     * Create a new small integer (2-byte) column on the table.
     * Range: -32,768 to 32,767 (signed) or 0 to 65,535 (unsigned).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @param  bool  $unsigned
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function smallInteger($column, $autoIncrement = false, $unsigned = false)
    {}

    /**
     * Create a new medium integer (3-byte) column on the table.
     * Range: -8,388,608 to 8,388,607 (signed) or 0 to 16,777,215 (unsigned).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @param  bool  $unsigned
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function mediumInteger($column, $autoIncrement = false, $unsigned = false)
    {}

    /**
     * Create a new big integer (8-byte) column on the table.
     * Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 (signed) or 0 to 18,446,744,073,709,551,615 (unsigned).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @param  bool  $unsigned
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function bigInteger($column, $autoIncrement = false, $unsigned = false)
    {}

    /**
     * Create a new unsigned integer column on the table (4-byte, 0 to 4,294,967,295).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function unsignedInteger($column, $autoIncrement = false)
    {}

    /**
     * Create a new unsigned tiny integer column on the table (1-byte, 0 to 255).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function unsignedTinyInteger($column, $autoIncrement = false)
    {}

    /**
     * Create a new unsigned small integer column on the table (2-byte, 0 to 65,535).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function unsignedSmallInteger($column, $autoIncrement = false)
    {}

    /**
     * Create a new unsigned medium integer column on the table (3-byte, 0 to 16,777,215).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function unsignedMediumInteger($column, $autoIncrement = false)
    {}

    /**
     * Create a new unsigned big integer column on the table (8-byte, 0 to 18,446,744,073,709,551,615).
     *
     * @param  string  $column
     * @param  bool  $autoIncrement
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function unsignedBigInteger($column, $autoIncrement = false)
    {}

    /**
     * Create a new unsigned big integer column on the table (8-byte, 0 to 18,446,744,073,709,551,615).
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ForeignIdColumnDefinition
     */
    public function foreignId($column)
    {}

    /**
     * Create a foreign ID column for the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model|string  $model
     * @param  string|null  $column
     * @return \Illuminate\Database\Schema\ForeignIdColumnDefinition
     */
    public function foreignIdFor($model, $column = null)
    {}

    /**
     * Create a foreign UUID column for the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model|string  $model
     * @param  string|null  $column
     * @return \Illuminate\Database\Schema\ForeignIdColumnDefinition
     */
    public function foreignUuidFor($model, $column = null)
    {}

    /**
     * Create a foreign ULID column for the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model|string  $model
     * @param  string|null  $column
     * @return \Illuminate\Database\Schema\ForeignIdColumnDefinition
     */
    public function foreignUlidFor($model, $column = null)
    {}

    /**
     * Create a new float column on the table.
     *
     * @param  string  $column
     * @param  int  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function float($column, $precision = 53)
    {}

    /**
     * Create a new double column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function double($column)
    {}

    /**
     * Create a new decimal column on the table.
     *
     * @param  string  $column
     * @param  int  $total
     * @param  int  $places
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function decimal($column, $total = 8, $places = 2)
    {}

    /**
     * Create a new boolean column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function boolean($column)
    {}

    /**
     * Create a new enum column on the table.
     *
     * @param  string  $column
     * @param  array  $allowed
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function enum($column, array $allowed)
    {}

    /**
     * Create a new set column on the table.
     *
     * @param  string  $column
     * @param  array  $allowed
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function set($column, array $allowed)
    {}

    /**
     * Create a new json column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function json($column)
    {}

    /**
     * Create a new jsonb column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function jsonb($column)
    {}

    /**
     * Create a new date column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function date($column)
    {}

    /**
     * Create a new date-time column on the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function dateTime($column, $precision = null)
    {}

    /**
     * Create a new date-time column (with time zone) on the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function dateTimeTz($column, $precision = null)
    {}

    /**
     * Create a new time column on the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function time($column, $precision = null)
    {}

    /**
     * Create a new time column (with time zone) on the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function timeTz($column, $precision = null)
    {}

    /**
     * Create a new timestamp column on the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function timestamp($column, $precision = null)
    {}

    /**
     * Create a new timestamp (with time zone) column on the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function timestampTz($column, $precision = null)
    {}

    /**
     * Add nullable creation and update timestamps to the table.
     *
     * @param  int|null  $precision
     * @return \Illuminate\Support\Collection<int, \Illuminate\Database\Schema\ColumnDefinition>
     */
    public function timestamps($precision = null)
    {}

    /**
     * Add nullable creation and update timestamps to the table.
     *
     * Alias for self::timestamps().
     *
     * @param  int|null  $precision
     * @return \Illuminate\Support\Collection<int, \Illuminate\Database\Schema\ColumnDefinition>
     */
    public function nullableTimestamps($precision = null)
    {}

    /**
     * Add nullable creation and update timestampTz columns to the table.
     *
     * @param  int|null  $precision
     * @return \Illuminate\Support\Collection<int, \Illuminate\Database\Schema\ColumnDefinition>
     */
    public function timestampsTz($precision = null)
    {}

    /**
     * Add nullable creation and update timestampTz columns to the table.
     *
     * Alias for self::timestampsTz().
     *
     * @param  int|null  $precision
     * @return \Illuminate\Support\Collection<int, \Illuminate\Database\Schema\ColumnDefinition>
     */
    public function nullableTimestampsTz($precision = null)
    {}

    /**
     * Add creation and update datetime columns to the table.
     *
     * @param  int|null  $precision
     * @return \Illuminate\Support\Collection<int, \Illuminate\Database\Schema\ColumnDefinition>
     */
    public function datetimes($precision = null)
    {}

    /**
     * Add a "deleted at" timestamp for the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function softDeletes($column = 'deleted_at', $precision = null)
    {}

    /**
     * Add a "deleted at" timestampTz for the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function softDeletesTz($column = 'deleted_at', $precision = null)
    {}

    /**
     * Add a "deleted at" datetime column to the table.
     *
     * @param  string  $column
     * @param  int|null  $precision
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function softDeletesDatetime($column = 'deleted_at', $precision = null)
    {}

    /**
     * Create a new year column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function year($column)
    {}

    /**
     * Create a new binary column on the table.
     *
     * @param  string  $column
     * @param  int|null  $length
     * @param  bool  $fixed
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function binary($column, $length = null, $fixed = false)
    {}

    /**
     * Create a new UUID column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function uuid($column = 'uuid')
    {}

    /**
     * Create a new UUID foreign ID column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ForeignIdColumnDefinition
     */
    public function foreignUuid($column)
    {}

    /**
     * Create a new ULID column on the table.
     *
     * @param  string  $column
     * @param  int|null  $length
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function ulid($column = 'ulid', $length = 26)
    {}

    /**
     * Create a new ULID foreign ID column on the table.
     *
     * @param  string  $column
     * @param  int|null  $length
     * @return \Illuminate\Database\Schema\ForeignIdColumnDefinition
     */
    public function foreignUlid($column, $length = 26)
    {}

    /**
     * Create a new IP address column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function ipAddress($column = 'ip_address')
    {}

    /**
     * Create a new MAC address column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function macAddress($column = 'mac_address')
    {}

    /**
     * Create a new geometry column on the table.
     *
     * @param  string  $column
     * @param  string|null  $subtype
     * @param  int  $srid
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function geometry($column, $subtype = null, $srid = 0)
    {}

    /**
     * Create a new geography column on the table.
     *
     * @param  string  $column
     * @param  string|null  $subtype
     * @param  int  $srid
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function geography($column, $subtype = null, $srid = 4326)
    {}

    /**
     * Create a new generated, computed column on the table.
     *
     * @param  string  $column
     * @param  string  $expression
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function computed($column, $expression)
    {}

    /**
     * Create a new vector column on the table.
     *
     * @param  string  $column
     * @param  int|null  $dimensions
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function vector($column, $dimensions = null)
    {}

    /**
     * Create a new tsvector column on the table.
     *
     * @param  string  $column
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function tsvector($column)
    {}

    /**
     * Add the proper columns for a polymorphic table.
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function morphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add nullable columns for a polymorphic table.
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function nullableMorphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add the proper columns for a polymorphic table using numeric IDs (incremental).
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function numericMorphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add nullable columns for a polymorphic table using numeric IDs (incremental).
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function nullableNumericMorphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add the proper columns for a polymorphic table using UUIDs.
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function uuidMorphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add nullable columns for a polymorphic table using UUIDs.
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function nullableUuidMorphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add the proper columns for a polymorphic table using ULIDs.
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function ulidMorphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add nullable columns for a polymorphic table using ULIDs.
     *
     * @param  string  $name
     * @param  string|null  $indexName
     * @param  string|null  $after
     * @return void
     */
    public function nullableUlidMorphs($name, $indexName = null, $after = null)
    {}

    /**
     * Add the `remember_token` column to the table.
     *
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function rememberToken()
    {}

    /**
     * Create a new custom column on the table.
     *
     * @param  string  $column
     * @param  string  $definition
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function rawColumn($column, $definition)
    {}

    /**
     * Add a comment to the table.
     *
     * @param  string  $comment
     * @return \Illuminate\Support\Fluent
     */
    public function comment($comment)
    {}

    /**
     * Create a new index command on the blueprint.
     *
     * @param  string  $type
     * @param  string|array  $columns
     * @param  string  $index
     * @param  string|null  $algorithm
     * @param  string|null  $operatorClass
     * @return \Illuminate\Support\Fluent
     */
    protected function indexCommand($type, $columns, $index, $algorithm = null, $operatorClass = null)
    {}

    /**
     * Create a new drop index command on the blueprint.
     *
     * @param  string  $command
     * @param  string  $type
     * @param  string|array  $index
     * @return \Illuminate\Support\Fluent
     */
    protected function dropIndexCommand($command, $type, $index)
    {}

    /**
     * Create a default index name for the table.
     *
     * @param  string  $type
     * @param  array  $columns
     * @return string
     */
    protected function createIndexName($type, array $columns)
    {}

    /**
     * Add a new column to the blueprint.
     *
     * @param  string  $type
     * @param  string  $name
     * @param  array  $parameters
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    public function addColumn($type, $name, array $parameters = [])
    {}

    /**
     * Add a new column definition to the blueprint.
     *
     * @param  \Illuminate\Database\Schema\ColumnDefinition  $definition
     * @return \Illuminate\Database\Schema\ColumnDefinition
     */
    protected function addColumnDefinition($definition)
    {}

    /**
     * Add the columns from the callback after the given column.
     *
     * @param  string  $column
     * @param  (\Closure(self): void)  $callback
     * @return void
     */
    public function after($column, Closure $callback)
    {}

    /**
     * Remove a column from the schema blueprint.
     *
     * @param  string  $name
     * @return $this
     */
    public function removeColumn($name)
    {}

    /**
     * Add a new command to the blueprint.
     *
     * @param  string  $name
     * @param  array  $parameters
     * @return \Illuminate\Support\Fluent
     */
    protected function addCommand($name, array $parameters = [])
    {}

    /**
     * Create a new Fluent command.
     *
     * @param  string  $name
     * @param  array  $parameters
     * @return \Illuminate\Support\Fluent
     */
    protected function createCommand($name, array $parameters = [])
    {}

    /**
     * Get the table the blueprint describes.
     *
     * @return string
     */
    public function getTable()
    {}

    /**
     * Get the table prefix.
     *
     * @deprecated Use DB::getTablePrefix()
     *
     * @return string
     */
    public function getPrefix()
    {}

    /**
     * Get the columns on the blueprint.
     *
     * @return \Illuminate\Database\Schema\ColumnDefinition[]
     */
    public function getColumns()
    {}

    /**
     * Get the commands on the blueprint.
     *
     * @return \Illuminate\Support\Fluent[]
     */
    public function getCommands()
    {}

    /**
     * Determine if the blueprint has state.
     *
     * @return bool
     */
    private function hasState(): bool
    {}

    /**
     * Get the state of the blueprint.
     *
     * @return \Illuminate\Database\Schema\BlueprintState
     */
    public function getState()
    {}

    /**
     * Get the columns on the blueprint that should be added.
     *
     * @return \Illuminate\Database\Schema\ColumnDefinition[]
     */
    public function getAddedColumns()
    {}

    /**
     * Get the columns on the blueprint that should be changed.
     *
     * @deprecated Will be removed in a future Laravel version.
     *
     * @return \Illuminate\Database\Schema\ColumnDefinition[]
     */
    public function getChangedColumns()
    {}

    /**
     * Get the default time precision.
     */
    protected function defaultTimePrecision(): ?int
    {}
}
