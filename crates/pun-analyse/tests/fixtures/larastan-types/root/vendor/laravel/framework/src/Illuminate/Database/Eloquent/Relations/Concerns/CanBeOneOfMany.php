<?php

namespace Illuminate\Database\Eloquent\Relations\Concerns;

use Closure;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Query\JoinClause;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use InvalidArgumentException;

trait CanBeOneOfMany
{
    /**
     * Determines whether the relationship is one-of-many.
     *
     * @var bool
     */
    protected $isOneOfMany = false;

    /**
     * The name of the relationship.
     *
     * @var string
     */
    protected $relationName;

    /**
     * The one of many inner join subselect query builder instance.
     *
     * @var \Illuminate\Database\Eloquent\Builder<*>|null
     */
    protected $oneOfManySubQuery;

    /**
     * The model of the query the one of many subselect is currently being joined to.
     *
     * @var \Illuminate\Database\Eloquent\Model|null
     */
    protected $oneOfManyJoinModel;

    /**
     * Add constraints for inner join subselect for one of many relationships.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $query
     * @param  string|null  $column
     * @param  string|null  $aggregate
     * @return void
     */
    abstract public function addOneOfManySubQueryConstraints(Builder $query, $column = null, $aggregate = null);

    /**
     * Get the columns the determine the relationship groups.
     *
     * @return array|string
     */
    abstract public function getOneOfManySubQuerySelectColumns();

    /**
     * Add join query constraints for one of many relationships.
     *
     * @param  \Illuminate\Database\Query\JoinClause  $join
     * @return void
     */
    abstract public function addOneOfManyJoinSubQueryConstraints(JoinClause $join);

    /**
     * Indicate that the relation is a single result of a larger one-to-many relationship.
     *
     * @param  string|array|null  $column
     * @param  string|\Closure|null  $aggregate
     * @param  string|null  $relation
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function ofMany($column = 'id', $aggregate = 'MAX', $relation = null)
    {}

    /**
     * Indicate that the relation is the latest single result of a larger one-to-many relationship.
     *
     * @param  string|array|null  $column
     * @param  string|null  $relation
     * @return $this
     */
    public function latestOfMany($column = 'id', $relation = null)
    {}

    /**
     * Indicate that the relation is the oldest single result of a larger one-to-many relationship.
     *
     * @param  string|array|null  $column
     * @param  string|null  $relation
     * @return $this
     */
    public function oldestOfMany($column = 'id', $relation = null)
    {}

    /**
     * Get the default alias for the one of many inner join clause.
     *
     * @param  string  $relation
     * @return string
     */
    protected function getDefaultOneOfManyJoinAlias($relation)
    {}

    /**
     * Get a new query for the related model, grouping the query by the given column, often the foreign key of the relationship.
     *
     * @param  string|array  $groupBy
     * @param  array<string>|null  $columns
     * @param  string|null  $aggregate
     * @return \Illuminate\Database\Eloquent\Builder<*>
     */
    protected function newOneOfManySubQuery($groupBy, $columns = null, $aggregate = null)
    {}

    /**
     * Add the join subquery to the given query on the given column and the relationship's foreign key.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $parent
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $subQuery
     * @param  array<string>  $on
     * @return void
     */
    protected function addOneOfManyJoinSubQuery(Builder $parent, Builder $subQuery, $on)
    {}

    /**
     * Merge the relationship query joins to the given query builder.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $query
     * @return void
     */
    protected function mergeOneOfManyJoinsTo(Builder $query)
    {}

    /**
     * Get the query builder that will contain the relationship constraints.
     *
     * @return \Illuminate\Database\Eloquent\Builder<*>
     */
    protected function getRelationQuery()
    {}

    /**
     * Get the one of many inner join subselect builder instance.
     *
     * @return \Illuminate\Database\Eloquent\Builder<*>|void
     */
    public function getOneOfManySubQuery()
    {}

    /**
     * Get the qualified column name for the one-of-many relationship using the subselect join query's alias.
     *
     * @param  string  $column
     * @return string
     */
    public function qualifySubSelectColumn($column)
    {}

    /**
     * Qualify related column using the related table name if it is not already qualified.
     *
     * @param  string  $column
     * @return string
     */
    protected function qualifyRelatedColumn($column)
    {}

    /**
     * Guess the "hasOne" relationship's name via backtrace.
     *
     * @return string
     */
    protected function guessRelationship()
    {}

    /**
     * Determine whether the relationship is a one-of-many relationship.
     *
     * @return bool
     */
    public function isOneOfMany()
    {}

    /**
     * Get the name of the relationship.
     *
     * @return string
     */
    public function getRelationName()
    {}
}
