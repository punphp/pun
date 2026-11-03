<?php

namespace Illuminate\Validation\Rules;

use Closure;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Collection;

use function Illuminate\Support\enum_value;

trait DatabaseRule
{
    /**
     * The table to run the query against.
     *
     * @var string
     */
    protected $table;

    /**
     * The column to check on.
     *
     * @var string
     */
    protected $column;

    /**
     * The extra where clauses for the query.
     *
     * @var array
     */
    protected $wheres = [];

    /**
     * The array of custom query callbacks.
     *
     * @var array
     */
    protected $using = [];

    /**
     * Create a new rule instance.
     *
     * @param  string  $table
     * @param  string  $column
     */
    public function __construct($table, $column = 'NULL')
    {}

    /**
     * Resolves the name of the table from the given string.
     *
     * @param  string  $table
     * @return string
     */
    public function resolveTableName($table)
    {}

    /**
     * Set a "where" constraint on the query.
     *
     * @param  \Closure|string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|\UnitEnum|\Closure|array|string|int|float|bool|null  $value
     * @return $this
     */
    public function where($column, $value = null)
    {}

    /**
     * Set a "where not" constraint on the query.
     *
     * @param  string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|\UnitEnum|array|string|int|bool  $value
     * @return $this
     */
    public function whereNot($column, $value)
    {}

    /**
     * Set a "where null" constraint on the query.
     *
     * @param  string  $column
     * @return $this
     */
    public function whereNull($column)
    {}

    /**
     * Set a "where not null" constraint on the query.
     *
     * @param  string  $column
     * @return $this
     */
    public function whereNotNull($column)
    {}

    /**
     * Set a "where in" constraint on the query.
     *
     * @param  string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|\BackedEnum|array  $values
     * @return $this
     */
    public function whereIn($column, $values)
    {}

    /**
     * Set a "where not in" constraint on the query.
     *
     * @param  string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|\BackedEnum|array  $values
     * @return $this
     */
    public function whereNotIn($column, $values)
    {}

    /**
     * Ignore soft deleted models during the existence check.
     *
     * @param  string  $deletedAtColumn
     * @return $this
     */
    public function withoutTrashed($deletedAtColumn = 'deleted_at')
    {}

    /**
     * Only include soft deleted models during the existence check.
     *
     * @param  string  $deletedAtColumn
     * @return $this
     */
    public function onlyTrashed($deletedAtColumn = 'deleted_at')
    {}

    /**
     * Register a custom query callback.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function using(Closure $callback)
    {}

    /**
     * Get the custom query callbacks for the rule.
     *
     * @return array
     */
    public function queryCallbacks()
    {}

    /**
     * Format the where clauses.
     *
     * @return string
     */
    protected function formatWheres()
    {}
}
