<?php

namespace Illuminate\Database\Eloquent\Concerns;

use BadMethodCallException;
use Closure;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\RelationNotFoundException;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\MorphTo;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Database\Query\Builder as QueryBuilder;
use Illuminate\Database\Query\Expression;
use Illuminate\Support\Collection as BaseCollection;
use Illuminate\Support\Str;
use InvalidArgumentException;

use function Illuminate\Support\enum_value;

/** @mixin \Illuminate\Database\Eloquent\Builder */
trait QueriesRelationships
{
    /**
     * Add a relationship count / exists condition to the query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @param  string  $boolean
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|null  $callback
     * @return $this
     *
     * @throws \RuntimeException
     */
    public function has($relation, $operator = '>=', $count = 1, $boolean = 'and', ?Closure $callback = null)
    {}

    /**
     * Add nested relationship count / exists conditions to the query.
     *
     * Sets up recursive call to whereHas until we finish the nested relation.
     *
     * @param  string  $relations
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @param  string  $boolean
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<*>): mixed)|null  $callback
     * @return $this
     */
    protected function hasNested($relations, $operator = '>=', $count = 1, $boolean = 'and', $callback = null)
    {}

    /**
     * Add a relationship count / exists condition to the query with an "or".
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<*, *, *>|string  $relation
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return $this
     */
    public function orHas($relation, $operator = '>=', $count = 1)
    {}

    /**
     * Add a relationship count / exists condition to the query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  string  $boolean
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|null  $callback
     * @return $this
     */
    public function doesntHave($relation, $boolean = 'and', ?Closure $callback = null)
    {}

    /**
     * Add a relationship count / exists condition to the query with an "or".
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<*, *, *>|string  $relation
     * @return $this
     */
    public function orDoesntHave($relation)
    {}

    /**
     * Add a relationship count / exists condition to the query with where clauses.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|null  $callback
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return $this
     */
    public function whereHas($relation, ?Closure $callback = null, $operator = '>=', $count = 1)
    {}

    /**
     * Add a relationship count / exists condition to the query with where clauses.
     *
     * Also load the relationship with the same condition.
     *
     * @param  string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|null  $callback
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return $this
     */
    public function withWhereHas($relation, ?Closure $callback = null, $operator = '>=', $count = 1)
    {}

    /**
     * Add a relationship count / exists condition to the query with where clauses and an "or".
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|null  $callback
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return $this
     */
    public function orWhereHas($relation, ?Closure $callback = null, $operator = '>=', $count = 1)
    {}

    /**
     * Add a relationship count / exists condition to the query with where clauses.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|null  $callback
     * @return $this
     */
    public function whereDoesntHave($relation, ?Closure $callback = null)
    {}

    /**
     * Add a relationship count / exists condition to the query with where clauses and an "or".
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|null  $callback
     * @return $this
     */
    public function orWhereDoesntHave($relation, ?Closure $callback = null)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @param  string  $boolean
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>, string): mixed)|null  $callback
     * @return $this
     */
    public function hasMorph($relation, $types, $operator = '>=', $count = 1, $boolean = 'and', ?Closure $callback = null)
    {}

    /**
     * Get the BelongsTo relationship for a single polymorphic type.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<*, TDeclaringModel>  $relation
     * @param  class-string<TRelatedModel>  $type
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<TRelatedModel, TDeclaringModel>
     */
    protected function getBelongsToRelation(MorphTo $relation, $type)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query with an "or".
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<*, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return $this
     */
    public function orHasMorph($relation, $types, $operator = '>=', $count = 1)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  string  $boolean
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>, string): mixed)|null  $callback
     * @return $this
     */
    public function doesntHaveMorph($relation, $types, $boolean = 'and', ?Closure $callback = null)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query with an "or".
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<*, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @return $this
     */
    public function orDoesntHaveMorph($relation, $types)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query with where clauses.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>, string): mixed)|null  $callback
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return $this
     */
    public function whereHasMorph($relation, $types, ?Closure $callback = null, $operator = '>=', $count = 1)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query with where clauses and an "or".
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>, string): mixed)|null  $callback
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return $this
     */
    public function orWhereHasMorph($relation, $types, ?Closure $callback = null, $operator = '>=', $count = 1)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query with where clauses.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>, string): mixed)|null  $callback
     * @return $this
     */
    public function whereDoesntHaveMorph($relation, $types, ?Closure $callback = null)
    {}

    /**
     * Add a polymorphic relationship count / exists condition to the query with where clauses and an "or".
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>, string): mixed)|null  $callback
     * @return $this
     */
    public function orWhereDoesntHaveMorph($relation, $types, ?Closure $callback = null)
    {}

    /**
     * Add a basic where clause to a relationship query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function whereRelation($relation, $column, $operator = null, $value = null)
    {}

    /**
     * Add a basic where clause to a relationship query and eager-load the relationship with the same conditions.
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<*, *, *>|string  $relation
     * @param  \Closure|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function withWhereRelation($relation, $column, $operator = null, $value = null)
    {}

    /**
     * Add an "or where" clause to a relationship query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereRelation($relation, $column, $operator = null, $value = null)
    {}

    /**
     * Add a basic count / exists condition to a relationship query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function whereDoesntHaveRelation($relation, $column, $operator = null, $value = null)
    {}

    /**
     * Add an "or where" clause to a relationship query.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, *, *>|string  $relation
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereDoesntHaveRelation($relation, $column, $operator = null, $value = null)
    {}

    /**
     * Add a polymorphic relationship condition to the query with a where clause.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function whereMorphRelation($relation, $types, $column, $operator = null, $value = null)
    {}

    /**
     * Add a polymorphic relationship condition to the query with an "or where" clause.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereMorphRelation($relation, $types, $column, $operator = null, $value = null)
    {}

    /**
     * Add a polymorphic relationship condition to the query with a doesn't have clause.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function whereMorphDoesntHaveRelation($relation, $types, $column, $operator = null, $value = null)
    {}

    /**
     * Add a polymorphic relationship condition to the query with an "or doesn't have" clause.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, *>|string  $relation
     * @param  string|array<int, string>  $types
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TRelatedModel>): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereMorphDoesntHaveRelation($relation, $types, $column, $operator = null, $value = null)
    {}

    /**
     * Add a morph-to relationship condition to the query.
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<*, *>|string  $relation
     * @param  \Illuminate\Database\Eloquent\Model|iterable<int, \Illuminate\Database\Eloquent\Model>|string|null  $model
     * @param  string  $boolean
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function whereMorphedTo($relation, $model, $boolean = 'and')
    {}

    /**
     * Add a not morph-to relationship condition to the query.
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<*, *>|string  $relation
     * @param  \Illuminate\Database\Eloquent\Model|iterable<int, \Illuminate\Database\Eloquent\Model>|string  $model
     * @param  string  $boolean
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function whereNotMorphedTo($relation, $model, $boolean = 'and')
    {}

    /**
     * Add a morph-to relationship condition to the query with an "or where" clause.
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<*, *>|string  $relation
     * @param  \Illuminate\Database\Eloquent\Model|iterable<int, \Illuminate\Database\Eloquent\Model>|string|null  $model
     * @return $this
     */
    public function orWhereMorphedTo($relation, $model)
    {}

    /**
     * Add a not morph-to relationship condition to the query with an "or where" clause.
     *
     * @param  \Illuminate\Database\Eloquent\Relations\MorphTo<*, *>|string  $relation
     * @param  \Illuminate\Database\Eloquent\Model|iterable<int, \Illuminate\Database\Eloquent\Model>|string  $model
     * @return $this
     */
    public function orWhereNotMorphedTo($relation, $model)
    {}

    /**
     * Add a "belongs to" relationship where clause to the query.
     *
     * @param  \Illuminate\Database\Eloquent\Model|\Illuminate\Database\Eloquent\Collection<int, covariant \Illuminate\Database\Eloquent\Model>  $related
     * @param  string|null  $relationshipName
     * @param  string  $boolean
     * @return $this
     *
     * @throws \InvalidArgumentException
     * @throws \Illuminate\Database\Eloquent\RelationNotFoundException
     */
    public function whereBelongsTo($related, $relationshipName = null, $boolean = 'and')
    {}

    /**
     * Add a "BelongsTo" relationship with an "or where" clause to the query.
     *
     * @param  \Illuminate\Database\Eloquent\Model|\Illuminate\Database\Eloquent\Collection<int, covariant \Illuminate\Database\Eloquent\Model>  $related
     * @param  string|null  $relationshipName
     * @return $this
     */
    public function orWhereBelongsTo($related, $relationshipName = null)
    {}

    /**
     * Add a "belongs to many" relationship where clause to the query.
     *
     * @param  \Illuminate\Database\Eloquent\Model|\Illuminate\Database\Eloquent\Collection<int, covariant \Illuminate\Database\Eloquent\Model>  $related
     * @param  string|null  $relationshipName
     * @param  string  $boolean
     * @return $this
     *
     * @throws \InvalidArgumentException
     * @throws \Illuminate\Database\Eloquent\RelationNotFoundException
     */
    public function whereAttachedTo($related, $relationshipName = null, $boolean = 'and')
    {}

    /**
     * Add a "belongs to many" relationship with an "or where" clause to the query.
     *
     * @param  \Illuminate\Database\Eloquent\Model|\Illuminate\Database\Eloquent\Collection<int, covariant \Illuminate\Database\Eloquent\Model>  $related
     * @param  string|null  $relationshipName
     * @return $this
     *
     * @throws \RuntimeException
     */
    public function orWhereAttachedTo($related, $relationshipName = null)
    {}

    /**
     * Add subselect queries to include an aggregate value for a relationship.
     *
     * @param  mixed  $relations
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string|null  $function
     * @return $this
     */
    public function withAggregate($relations, $column, $function = null)
    {}

    /**
     * Get the relation hashed column name for the given column and relation.
     *
     * @param  string  $column
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<*, *, *>  $relation
     * @return string
     */
    protected function getRelationHashedColumn($column, $relation)
    {}

    /**
     * Add subselect queries to count the relations.
     *
     * @param  mixed  $relations
     * @return $this
     */
    public function withCount($relations)
    {}

    /**
     * Add subselect queries to include the max of the relation's column.
     *
     * @param  string|array  $relation
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function withMax($relation, $column)
    {}

    /**
     * Add subselect queries to include the min of the relation's column.
     *
     * @param  string|array  $relation
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function withMin($relation, $column)
    {}

    /**
     * Add subselect queries to include the sum of the relation's column.
     *
     * @param  string|array  $relation
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function withSum($relation, $column)
    {}

    /**
     * Add subselect queries to include the average of the relation's column.
     *
     * @param  string|array  $relation
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function withAvg($relation, $column)
    {}

    /**
     * Add subselect queries to include the existence of related models.
     *
     * @param  string|array  $relation
     * @return $this
     */
    public function withExists($relation)
    {}

    /**
     * Add the "has" condition where clause to the query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $hasQuery
     * @param  \Illuminate\Database\Eloquent\Relations\Relation<*, *, *>  $relation
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @param  string  $boolean
     * @return $this
     */
    protected function addHasWhere(Builder $hasQuery, Relation $relation, $operator, $count, $boolean)
    {}

    /**
     * Merge the where constraints from another query to the current query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<*>  $from
     * @return $this
     */
    public function mergeConstraintsFrom(Builder $from)
    {}

    /**
     * Updates the table name for any columns with a new qualified name.
     *
     * @param  array  $wheres
     * @param  string  $from
     * @param  string  $to
     * @return array
     */
    protected function requalifyWhereTables(array $wheres, string $from, string $to): array
    {}

    /**
     * Add a sub-query count clause to this query.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @param  string  $boolean
     * @return $this
     */
    protected function addWhereCountQuery(QueryBuilder $query, $operator = '>=', $count = 1, $boolean = 'and')
    {}

    /**
     * Get the "has relation" base query instance.
     *
     * @param  string  $relation
     * @return \Illuminate\Database\Eloquent\Relations\Relation<*, *, *>
     */
    protected function getRelationWithoutConstraints($relation)
    {}

    /**
     * Check if we can run an "exists" query to optimize performance.
     *
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|int  $count
     * @return bool
     */
    protected function canUseExistsForExistenceCheck($operator, $count)
    {}
}
