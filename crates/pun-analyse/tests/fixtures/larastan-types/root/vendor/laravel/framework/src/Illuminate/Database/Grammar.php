<?php

namespace Illuminate\Database;

use Illuminate\Contracts\Database\Query\Expression;
use Illuminate\Support\Collection;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\Macroable;
use RuntimeException;

abstract class Grammar
{
    use Macroable;

    /**
     * The connection used for escaping values.
     *
     * @var \Illuminate\Database\Connection
     */
    protected $connection;

    /**
     * Create a new grammar instance.
     *
     * @param  \Illuminate\Database\Connection  $connection
     */
    public function __construct(Connection $connection)
    {}

    /**
     * Wrap an array of values.
     *
     * @param  array<\Illuminate\Contracts\Database\Query\Expression|string>  $values
     * @return array<string>
     */
    public function wrapArray(array $values)
    {}

    /**
     * Wrap a table in keyword identifiers.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  string|null  $prefix
     * @return string
     */
    public function wrapTable($table, $prefix = null)
    {}

    /**
     * Wrap a value in keyword identifiers.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $value
     * @return string
     */
    public function wrap($value)
    {}

    /**
     * Wrap a value that has an alias.
     *
     * @param  string  $value
     * @return string
     */
    protected function wrapAliasedValue($value)
    {}

    /**
     * Wrap a table that has an alias.
     *
     * @param  string  $value
     * @param  string|null  $prefix
     * @return string
     */
    protected function wrapAliasedTable($value, $prefix = null)
    {}

    /**
     * Wrap the given value segments.
     *
     * @param  list<string>  $segments
     * @return string
     */
    protected function wrapSegments($segments)
    {}

    /**
     * Wrap a single string in keyword identifiers.
     *
     * @param  string  $value
     * @return string
     */
    protected function wrapValue($value)
    {}

    /**
     * Wrap the given JSON selector.
     *
     * @param  string  $value
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function wrapJsonSelector($value)
    {}

    /**
     * Determine if the given string is a JSON selector.
     *
     * @param  string  $value
     * @return bool
     */
    protected function isJsonSelector($value)
    {}

    /**
     * Convert an array of column names into a delimited string.
     *
     * @param  array<\Illuminate\Contracts\Database\Query\Expression|string>  $columns
     * @return string
     */
    public function columnize(array $columns)
    {}

    /**
     * Create query parameter place-holders for an array.
     *
     * @param  array<mixed>  $values
     * @return string
     */
    public function parameterize(array $values)
    {}

    /**
     * Get the appropriate query parameter place-holder for a value.
     *
     * @param  mixed  $value
     * @return string
     */
    public function parameter($value)
    {}

    /**
     * Quote the given string literal.
     *
     * @param  string|array<string>  $value
     * @return string
     */
    public function quoteString($value)
    {}

    /**
     * Escapes a value for safe SQL embedding.
     *
     * @param  string|float|int|bool|null  $value
     * @param  bool  $binary
     * @return string
     */
    public function escape($value, $binary = false)
    {}

    /**
     * Determine if the given value is a raw expression.
     *
     * @param  mixed  $value
     * @return bool
     */
    public function isExpression($value)
    {}

    /**
     * Transforms expressions to their scalar types.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|int|float  $expression
     * @return string|int|float
     */
    public function getValue($expression)
    {}

    /**
     * Get the format for database stored dates.
     *
     * @return string
     */
    public function getDateFormat()
    {}

    /**
     * Get the grammar's table prefix.
     *
     * @deprecated Use DB::getTablePrefix()
     *
     * @return string
     */
    public function getTablePrefix()
    {}

    /**
     * Set the grammar's table prefix.
     *
     * @deprecated Use DB::setTablePrefix()
     *
     * @param  string  $prefix
     * @return $this
     */
    public function setTablePrefix($prefix)
    {}
}
