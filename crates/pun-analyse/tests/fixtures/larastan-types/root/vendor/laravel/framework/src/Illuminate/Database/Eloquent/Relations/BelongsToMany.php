<?php

namespace Illuminate\Database\Eloquent\Relations;

use Closure;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\Eloquent\Relations\Concerns\AsPivot;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithPivotTable;
use Illuminate\Database\Eloquent\Relations\Concerns\SupportsPivotInverseRelations;
use Illuminate\Database\Query\Grammars\MySqlGrammar;
use Illuminate\Database\UniqueConstraintViolationException;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection as BaseCollection;
use Illuminate\Support\Str;
use InvalidArgumentException;
use SortDirection;

use function Illuminate\Support\enum_value;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 * @template TPivotModel of \Illuminate\Database\Eloquent\Relations\Pivot = \Illuminate\Database\Eloquent\Relations\Pivot
 * @template TAccessor of string = 'pivot'
 *
 * @extends \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, TDeclaringModel, \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>>
 *
 * @todo use TAccessor when PHPStan bug is fixed: https://github.com/phpstan/phpstan/issues/12756
 */
class BelongsToMany extends Relation
{
    use InteractsWithDictionary, InteractsWithPivotTable, SupportsPivotInverseRelations;

    /**
     * The intermediate table for the relation.
     *
     * @var string
     */
    protected $table;

    /**
     * The foreign key of the parent model.
     *
     * @var string
     */
    protected $foreignPivotKey;

    /**
     * The associated key of the relation.
     *
     * @var string
     */
    protected $relatedPivotKey;

    /**
     * The key name of the parent model.
     *
     * @var string
     */
    protected $parentKey;

    /**
     * The key name of the related model.
     *
     * @var string
     */
    protected $relatedKey;

    /**
     * The "name" of the relationship.
     *
     * @var string
     */
    protected $relationName;

    /**
     * The pivot table columns to retrieve.
     *
     * @var array<string|\Illuminate\Contracts\Database\Query\Expression>
     */
    protected $pivotColumns = [];

    /**
     * Any pivot table restrictions for where clauses.
     *
     * @var array
     */
    protected $pivotWheres = [];

    /**
     * Any pivot table restrictions for whereIn clauses.
     *
     * @var array
     */
    protected $pivotWhereIns = [];

    /**
     * Any pivot table restrictions for whereBetween clauses.
     *
     * @var array
     */
    protected $pivotWhereBetweens = [];

    /**
     * Any pivot table restrictions for whereNull clauses.
     *
     * @var array
     */
    protected $pivotWhereNulls = [];

    /**
     * The default values for the pivot columns.
     *
     * @var array
     */
    protected $pivotValues = [];

    /**
     * Indicates if timestamps are available on the pivot table.
     *
     * @var bool
     */
    public $withTimestamps = false;

    /**
     * The custom pivot table column for the created_at timestamp.
     *
     * @var string|null
     */
    protected $pivotCreatedAt;

    /**
     * The custom pivot table column for the updated_at timestamp.
     *
     * @var string|null
     */
    protected $pivotUpdatedAt;

    /**
     * The class name of the custom pivot model to use for the relationship.
     *
     * @var class-string<TPivotModel>
     */
    protected $using;

    /**
     * The name of the accessor to use for the "pivot" relationship.
     *
     * @var TAccessor
     */
    protected $accessor = 'pivot';

    /**
     * Create a new belongs to many relationship instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string|class-string<TRelatedModel>  $table
     * @param  string  $foreignPivotKey
     * @param  string  $relatedPivotKey
     * @param  string  $parentKey
     * @param  string  $relatedKey
     * @param  string|null  $relationName
     */
    public function __construct(
        Builder $query,
        Model $parent,
        $table,
        $foreignPivotKey,
        $relatedPivotKey,
        $parentKey,
        $relatedKey,
        $relationName = null,
    ) {}

    /**
     * Attempt to resolve the intermediate table name from the given string.
     *
     * @param  string  $table
     * @return string
     */
    protected function resolveTableName($table)
    {}

    /**
     * Set the base constraints on the relation query.
     *
     * @return void
     */
    public function addConstraints()
    {}

    /**
     * Set the join clause for the relation query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>|null  $query
     * @return $this
     */
    protected function performJoin($query = null)
    {}

    /**
     * Set the where clause for the relation query.
     *
     * @return $this
     */
    protected function addWhereConstraints()
    {}

    /** @inheritDoc */
    public function addEagerConstraints(array $models)
    {}

    /** @inheritDoc */
    public function initRelation(array $models, $relation)
    {}

    /** @inheritDoc */
    public function match(array $models, EloquentCollection $results, $relation)
    {}

    /**
     * Build model dictionary keyed by the relation's foreign key.
     *
     * @param  \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>  $results
     * @return array<array<array-key, TRelatedModel>>
     */
    protected function buildDictionary(EloquentCollection $results)
    {}

    /**
     * Get the class being used for pivot models.
     *
     * @return class-string<TPivotModel>
     */
    public function getPivotClass()
    {}

    /**
     * Specify the custom pivot model to use for the relationship.
     *
     * @template TNewPivotModel of \Illuminate\Database\Eloquent\Relations\Pivot
     *
     * @param  class-string<TNewPivotModel>  $class
     * @return $this
     *
     * @phpstan-this-out static<TRelatedModel, TDeclaringModel, TNewPivotModel, TAccessor>
     */
    public function using($class)
    {}

    /**
     * Specify the custom pivot accessor to use for the relationship.
     *
     * @template TNewAccessor of string
     *
     * @param  TNewAccessor  $accessor
     * @return $this
     *
     * @phpstan-this-out static<TRelatedModel, TDeclaringModel, TPivotModel, TNewAccessor>
     */
    public function as($accessor)
    {}

    /**
     * Set a where clause for a pivot table column.
     *
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TPivotModel>): mixed)|string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function wherePivot($column, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Set a "where between" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  array  $values
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function wherePivotBetween($column, array $values, $boolean = 'and', $not = false)
    {}

    /**
     * Set a "or where between" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  array  $values
     * @return $this
     */
    public function orWherePivotBetween($column, array $values)
    {}

    /**
     * Set a "where pivot not between" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  array  $values
     * @param  string  $boolean
     * @return $this
     */
    public function wherePivotNotBetween($column, array $values, $boolean = 'and')
    {}

    /**
     * Set a "or where not between" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  array  $values
     * @return $this
     */
    public function orWherePivotNotBetween($column, array $values)
    {}

    /**
     * Set a "where in" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $values
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function wherePivotIn($column, $values, $boolean = 'and', $not = false)
    {}

    /**
     * Set an "or where" clause for a pivot table column.
     *
     * @param  (\Closure(\Illuminate\Database\Eloquent\Builder<TPivotModel>): mixed)|string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWherePivot($column, $operator = null, $value = null)
    {}

    /**
     * Set a where clause for a pivot table column.
     *
     * In addition, new pivot records will receive this value.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|array<string, string>  $column
     * @param  mixed  $value
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function withPivotValue($column, $value = null)
    {}

    /**
     * Set an "or where in" clause for a pivot table column.
     *
     * @param  string  $column
     * @param  mixed  $values
     * @return $this
     */
    public function orWherePivotIn($column, $values)
    {}

    /**
     * Set a "where not in" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $values
     * @param  string  $boolean
     * @return $this
     */
    public function wherePivotNotIn($column, $values, $boolean = 'and')
    {}

    /**
     * Set an "or where not in" clause for a pivot table column.
     *
     * @param  string  $column
     * @param  mixed  $values
     * @return $this
     */
    public function orWherePivotNotIn($column, $values)
    {}

    /**
     * Set a "where null" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function wherePivotNull($column, $boolean = 'and', $not = false)
    {}

    /**
     * Set a "where not null" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  string  $boolean
     * @return $this
     */
    public function wherePivotNotNull($column, $boolean = 'and')
    {}

    /**
     * Set a "or where null" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  bool  $not
     * @return $this
     */
    public function orWherePivotNull($column, $not = false)
    {}

    /**
     * Set a "or where not null" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return $this
     */
    public function orWherePivotNotNull($column)
    {}

    /**
     * Add an "order by" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  SortDirection|'asc'|'desc'  $direction
     * @return $this
     */
    public function orderByPivot($column, $direction = SortDirection::Ascending)
    {}

    /**
     * Add an "order by desc" clause for a pivot table column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return $this
     */
    public function orderByPivotDesc($column)
    {}

    /**
     * Find a related model by its primary key or return a new instance of the related model.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return (
     *     $id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>)
     *     ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>
     *     : TRelatedModel&object{pivot: TPivotModel}
     * )
     */
    public function findOrNew($id, $columns = ['*'])
    {}

    /**
     * Get the first related model record matching the attributes or instantiate it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function firstOrNew(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Get the first record matching the attributes. If the record is not found, create it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function firstOrCreate(array $attributes = [], Closure|array $values = [], array $joining = [], $touch = true)
    {}

    /**
     * Attempt to create the record. If a unique constraint violation occurs, attempt to find the matching record.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     *
     * @throws \Illuminate\Database\UniqueConstraintViolationException
     */
    public function createOrFirst(array $attributes = [], Closure|array $values = [], array $joining = [], $touch = true)
    {}

    /**
     * Get the first record matching the attributes, including trashed records, and restore it. If the record is not found, create it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function restoreOrCreate(array $attributes = [], Closure|array $values = [], array $joining = [], $touch = true)
    {}

    /**
     * Attempt to create the record. If a unique constraint violation occurs, find the matching record, including trashed records, and restore it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     *
     * @throws \Illuminate\Database\UniqueConstraintViolationException
     */
    public function createOrRestore(array $attributes = [], Closure|array $values = [], array $joining = [], $touch = true)
    {}

    /**
     * Create or update a related record matching the attributes, and fill it with values.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function updateOrCreate(array $attributes, Closure|array $values = [], array $joining = [], $touch = true)
    {}

    /**
     * Create a related record matching the attributes, or increment the existing record.
     *
     * @param  array  $attributes
     * @param  string  $column
     * @param  int|float  $default
     * @param  int|float  $step
     * @param  array  $extra
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function incrementOrCreate(array $attributes, string $column = 'count', $default = 1, $step = 1, array $extra = [])
    {}

    /**
     * Find a related model by its primary key.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return (
     *     $id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>)
     *     ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>
     *     : (TRelatedModel&object{pivot: TPivotModel})|null
     * )
     */
    public function find($id, $columns = ['*'])
    {}

    /**
     * Find a sole related model by its primary key.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return TRelatedModel&object{pivot: TPivotModel}
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TRelatedModel>
     * @throws \Illuminate\Database\MultipleRecordsFoundException
     */
    public function findSole($id, $columns = ['*'])
    {}

    /**
     * Find multiple related models by their primary keys.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $ids
     * @param  array  $columns
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function findMany($ids, $columns = ['*'])
    {}

    /**
     * Find a related model by its primary key or throw an exception.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return (
     *     $id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>)
     *     ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>
     *     : TRelatedModel&object{pivot: TPivotModel}
     * )
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TRelatedModel>
     */
    public function findOrFail($id, $columns = ['*'])
    {}

    /**
     * Find a related model by its primary key or call a callback.
     *
     * @template TValue
     *
     * @param  mixed  $id
     * @param  (\Closure(): TValue)|list<string>|string  $columns
     * @param  (\Closure(): TValue)|null  $callback
     * @return (
     *     $id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>)
     *     ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>|TValue
     *     : (TRelatedModel&object{pivot: TPivotModel})|TValue
     * )
     */
    public function findOr($id, $columns = ['*'], ?Closure $callback = null)
    {}

    /**
     * Add a basic where clause to the query, and return the first result.
     *
     * @param  \Closure|string|array  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return (TRelatedModel&object{pivot: TPivotModel})|null
     */
    public function firstWhere($column, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Execute the query and get the first result.
     *
     * @param  array  $columns
     * @return (TRelatedModel&object{pivot: TPivotModel})|null
     */
    public function first($columns = ['*'])
    {}

    /**
     * Execute the query and get the first result or throw an exception.
     *
     * @param  array  $columns
     * @return TRelatedModel&object{pivot: TPivotModel}
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TRelatedModel>
     */
    public function firstOrFail($columns = ['*'])
    {}

    /**
     * Execute the query and get the first result or call a callback.
     *
     * @template TValue
     *
     * @param  (\Closure(): TValue)|list<string>  $columns
     * @param  (\Closure(): TValue)|null  $callback
     * @return (TRelatedModel&object{pivot: TPivotModel})|TValue
     */
    public function firstOr($columns = ['*'], ?Closure $callback = null)
    {}

    /**
     * Get the results of the relationship.
     *
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function getResults()
    {}

    /**
     * Execute the query as a "select" statement.
     *
     * @param  array  $columns
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function get($columns = ['*'])
    {}

    /**
     * Get the select columns for the relation query.
     *
     * @param  array  $columns
     * @return array
     */
    protected function shouldSelect(array $columns = ['*'])
    {}

    /**
     * Get the pivot columns for the relation.
     *
     * "pivot_" is prefixed at each column for easy removal later.
     *
     * @return array
     */
    protected function aliasedPivotColumns()
    {}

    /**
     * Get a paginator for the "select" statement.
     *
     * @param  int|null  $perPage
     * @param  array  $columns
     * @param  string  $pageName
     * @param  int|null  $page
     * @return \Illuminate\Pagination\LengthAwarePaginator<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function paginate($perPage = null, $columns = ['*'], $pageName = 'page', $page = null)
    {}

    /**
     * Paginate the given query into a simple paginator.
     *
     * @param  int|null  $perPage
     * @param  array  $columns
     * @param  string  $pageName
     * @param  int|null  $page
     * @return \Illuminate\Contracts\Pagination\Paginator<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function simplePaginate($perPage = null, $columns = ['*'], $pageName = 'page', $page = null)
    {}

    /**
     * Paginate the given query into a cursor paginator.
     *
     * @param  int|null  $perPage
     * @param  array  $columns
     * @param  string  $cursorName
     * @param  string|null  $cursor
     * @return \Illuminate\Contracts\Pagination\CursorPaginator<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function cursorPaginate($perPage = null, $columns = ['*'], $cursorName = 'cursor', $cursor = null)
    {}

    /**
     * Chunk the results of the query.
     *
     * @param  int  $count
     * @param  callable  $callback
     * @return bool
     */
    public function chunk($count, callable $callback)
    {}

    /**
     * Chunk the results of a query by comparing numeric IDs.
     *
     * @param  int  $count
     * @param  callable  $callback
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return bool
     */
    public function chunkById($count, callable $callback, $column = null, $alias = null)
    {}

    /**
     * Chunk the results of a query by comparing IDs in descending order.
     *
     * @param  int  $count
     * @param  callable  $callback
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return bool
     */
    public function chunkByIdDesc($count, callable $callback, $column = null, $alias = null)
    {}

    /**
     * Execute a callback over each item while chunking by ID.
     *
     * @param  callable  $callback
     * @param  int  $count
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return bool
     */
    public function eachById(callable $callback, $count = 1000, $column = null, $alias = null)
    {}

    /**
     * Chunk the results of a query by comparing IDs in a given order.
     *
     * @param  int  $count
     * @param  callable  $callback
     * @param  string|null  $column
     * @param  string|null  $alias
     * @param  SortDirection|bool  $descending
     * @return bool
     */
    public function orderedChunkById($count, callable $callback, $column = null, $alias = null, $descending = false)
    {}

    /**
     * Execute a callback over each item while chunking.
     *
     * @param  callable  $callback
     * @param  int  $count
     * @return bool
     */
    public function each(callable $callback, $count = 1000)
    {}

    /**
     * Run a map over each item while chunking.
     *
     * @template TReturn
     *
     * @param  callable(TRelatedModel&object{pivot: TPivotModel}): TReturn  $callback
     * @param  int  $count
     * @return \Illuminate\Support\Collection<int, TReturn>
     */
    public function chunkMap(callable $callback, $count = 1000)
    {}

    /**
     * Query lazily, by chunks of the given size.
     *
     * @param  int  $chunkSize
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function lazy($chunkSize = 1000)
    {}

    /**
     * Query lazily, by chunking the results of a query by comparing IDs.
     *
     * @param  int  $chunkSize
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function lazyById($chunkSize = 1000, $column = null, $alias = null)
    {}

    /**
     * Query lazily, by chunking the results of a query by comparing IDs in descending order.
     *
     * @param  int  $chunkSize
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function lazyByIdDesc($chunkSize = 1000, $column = null, $alias = null)
    {}

    /**
     * Get a lazy collection for the given query.
     *
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function cursor()
    {}

    /**
     * Prepare the query builder for query execution.
     *
     * @return \Illuminate\Database\Eloquent\Builder<TRelatedModel>
     */
    protected function prepareQueryBuilder()
    {}

    /**
     * Hydrate the pivot table relationship on the models.
     *
     * @param  array<int, TRelatedModel>  $models
     * @return void
     */
    protected function hydratePivotRelation(array $models)
    {}

    /**
     * Get the pivot attributes from a model.
     *
     * @param  TRelatedModel  $model
     * @return array
     */
    protected function migratePivotAttributes(Model $model)
    {}

    /**
     * If we're touching the parent model, touch.
     *
     * @return void
     */
    public function touchIfTouching()
    {}

    /**
     * Determine if we should touch the parent on sync.
     *
     * @return bool
     */
    protected function touchingParent()
    {}

    /**
     * Attempt to guess the name of the inverse of the relation.
     *
     * @return string
     */
    protected function guessInverseRelation()
    {}

    /**
     * Touch all of the related models for the relationship.
     *
     * E.g.: Touch all roles associated with this user.
     *
     * @return void
     */
    public function touch()
    {}

    /**
     * Get all of the IDs for the related models.
     *
     * @return \Illuminate\Support\Collection<int, int|string>
     */
    public function allRelatedIds()
    {}

    /**
     * Save a new model and attach it to the parent model.
     *
     * @param  TRelatedModel  $model
     * @param  array  $pivotAttributes
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function save(Model $model, array $pivotAttributes = [], $touch = true)
    {}

    /**
     * Save a new model without raising any events and attach it to the parent model.
     *
     * @param  TRelatedModel  $model
     * @param  array  $pivotAttributes
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function saveQuietly(Model $model, array $pivotAttributes = [], $touch = true)
    {}

    /**
     * Save an array of new models and attach them to the parent model.
     *
     * @template TContainer of \Illuminate\Support\Collection<array-key, TRelatedModel>|array<array-key, TRelatedModel>
     *
     * @param  TContainer  $models
     * @param  array  $pivotAttributes
     * @return TContainer
     */
    public function saveMany($models, array $pivotAttributes = [])
    {}

    /**
     * Save an array of new models without raising any events and attach them to the parent model.
     *
     * @template TContainer of \Illuminate\Support\Collection<array-key, TRelatedModel>|array<array-key, TRelatedModel>
     *
     * @param  TContainer  $models
     * @param  array  $pivotAttributes
     * @return TContainer
     */
    public function saveManyQuietly($models, array $pivotAttributes = [])
    {}

    /**
     * Create a new instance of the related model.
     *
     * @param  array  $attributes
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function create(array $attributes = [], array $joining = [], $touch = true)
    {}

    /**
     * Create an array of new instances of the related models.
     *
     * @param  iterable  $records
     * @param  array  $joinings
     * @return array<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function createMany(iterable $records, array $joinings = [])
    {}

    /**
     * Create a new instance of the related model without raising any events and attach it to the parent model.
     *
     * @param  array  $attributes
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function createQuietly(array $attributes = [], array $joining = [], $touch = true)
    {}

    /**
     * Create an array of new instances of the related models without raising any events and attach them to the parent model.
     *
     * @param  iterable  $records
     * @param  array  $joinings
     * @return array<int, TRelatedModel&object{pivot: TPivotModel}>
     */
    public function createManyQuietly(iterable $records, array $joinings = [])
    {}

    /**
     * Create a new instance of the related model and attach it to the parent model. Allow mass-assignment.
     *
     * @param  array  $attributes
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function forceCreate(array $attributes = [], array $joining = [], $touch = true)
    {}

    /**
     * Create a new instance of the related model with mass assignment without raising any events and attach it to the parent model.
     *
     * @param  array  $attributes
     * @param  array  $joining
     * @param  bool  $touch
     * @return TRelatedModel&object{pivot: TPivotModel}
     */
    public function forceCreateQuietly(array $attributes = [], array $joining = [], $touch = true)
    {}

    /** @inheritDoc */
    public function getRelationExistenceQuery(Builder $query, Builder $parentQuery, $columns = ['*'])
    {}

    /**
     * Add the constraints for a relationship query on the same table.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  \Illuminate\Database\Eloquent\Builder<TDeclaringModel>  $parentQuery
     * @param  mixed  $columns
     * @return \Illuminate\Database\Eloquent\Builder<TRelatedModel>
     */
    public function getRelationExistenceQueryForSelfJoin(Builder $query, Builder $parentQuery, $columns = ['*'])
    {}

    /**
     * Alias to set the "limit" value of the query.
     *
     * @param  int  $value
     * @return $this
     */
    public function take($value)
    {}

    /**
     * Set the "limit" value of the query.
     *
     * @param  int  $value
     * @return $this
     */
    public function limit($value)
    {}

    /**
     * Get the key for comparing against the parent key in "has" query.
     *
     * @return string
     */
    public function getExistenceCompareKey()
    {}

    /**
     * Specify that the pivot table has creation and update timestamps.
     *
     * @param  string|null|false  $createdAt
     * @param  string|null|false  $updatedAt
     * @return $this
     */
    public function withTimestamps($createdAt = null, $updatedAt = null)
    {}

    /**
     * Get the name of the "created at" column.
     *
     * @return string
     */
    public function createdAt()
    {}

    /**
     * Get the name of the "updated at" column.
     *
     * @return string
     */
    public function updatedAt()
    {}

    /**
     * Get the foreign key for the relation.
     *
     * @return string
     */
    public function getForeignPivotKeyName()
    {}

    /**
     * Get the fully-qualified foreign key for the relation.
     *
     * @return string
     */
    public function getQualifiedForeignPivotKeyName()
    {}

    /**
     * Get the "related key" for the relation.
     *
     * @return string
     */
    public function getRelatedPivotKeyName()
    {}

    /**
     * Get the fully-qualified "related key" for the relation.
     *
     * @return string
     */
    public function getQualifiedRelatedPivotKeyName()
    {}

    /**
     * Get the parent key for the relationship.
     *
     * @return string
     */
    public function getParentKeyName()
    {}

    /**
     * Get the fully-qualified parent key name for the relation.
     *
     * @return string
     */
    public function getQualifiedParentKeyName()
    {}

    /**
     * Get the related key for the relationship.
     *
     * @return string
     */
    public function getRelatedKeyName()
    {}

    /**
     * Get the fully-qualified related key name for the relation.
     *
     * @return string
     */
    public function getQualifiedRelatedKeyName()
    {}

    /**
     * Get the intermediate table for the relationship.
     *
     * @return string
     */
    public function getTable()
    {}

    /**
     * Get the relationship name for the relationship.
     *
     * @return string
     */
    public function getRelationName()
    {}

    /**
     * Get the name of the pivot accessor for this relationship.
     *
     * @return TAccessor
     */
    public function getPivotAccessor()
    {}

    /**
     * Get the pivot columns for this relationship.
     *
     * @return array<string|\Illuminate\Contracts\Database\Query\Expression>
     */
    public function getPivotColumns()
    {}

    /**
     * Qualify the columns of the given pivot query's where clauses.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @return void
     */
    protected function qualifyPivotWheres($query)
    {}

    /**
     * Qualify the given column name by the pivot table.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return string|\Illuminate\Contracts\Database\Query\Expression
     */
    public function qualifyPivotColumn($column)
    {}
}
