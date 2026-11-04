<?php

namespace Illuminate\Database\Eloquent;

use BadMethodCallException;
use Closure;
use Exception;
use Illuminate\Contracts\Database\Eloquent\Builder as BuilderContract;
use Illuminate\Contracts\Database\Eloquent\SupportsPartialRelations;
use Illuminate\Contracts\Database\Query\Expression;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Concerns\BuildsQueries;
use Illuminate\Database\Eloquent\Concerns\QueriesRelationships;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasOneOrManyThrough;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Database\Query\Builder as QueryBuilder;
use Illuminate\Database\RecordsNotFoundException;
use Illuminate\Database\UniqueConstraintViolationException;
use Illuminate\Pagination\Paginator;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection as BaseCollection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ForwardsCalls;
use ReflectionClass;
use ReflectionMethod;
use SortDirection;

use function Illuminate\Support\enum_value;

/**
 * @template TModel of \Illuminate\Database\Eloquent\Model
 *
 * @property-read HigherOrderBuilderProxy|$this $orWhere
 * @property-read HigherOrderBuilderProxy|$this $whereNot
 * @property-read HigherOrderBuilderProxy|$this $orWhereNot
 *
 * @mixin \Illuminate\Database\Query\Builder
 */
class Builder implements BuilderContract
{
    /** @use \Illuminate\Database\Concerns\BuildsQueries<TModel> */
    use BuildsQueries, ForwardsCalls, QueriesRelationships {
        BuildsQueries::sole as baseSole;
    }

    /**
     * The base query builder instance.
     *
     * @var \Illuminate\Database\Query\Builder
     */
    protected $query;

    /**
     * The model being queried.
     *
     * @var TModel
     */
    protected $model;

    /**
     * The attributes that should be added to new models created by this builder.
     *
     * @var array
     */
    public $pendingAttributes = [];

    /**
     * The relationships that should be eager loaded.
     *
     * @var array
     */
    protected $eagerLoad = [];

    /**
     * All of the globally registered builder macros.
     *
     * @var array
     */
    protected static $macros = [];

    /**
     * All of the locally registered builder macros.
     *
     * @var array
     */
    protected $localMacros = [];

    /**
     * A replacement for the typical delete function.
     *
     * @var \Closure
     */
    protected $onDelete;

    /**
     * The properties that should be returned from query builder.
     *
     * @var string[]
     */
    protected $propertyPassthru = [
        'from',
    ];

    /**
     * The methods that should be returned from query builder.
     *
     * @var string[]
     */
    protected $passthru = [
        'aggregate',
        'average',
        'avg',
        'count',
        'dd',
        'ddrawsql',
        'doesntexist',
        'doesntexistor',
        'dump',
        'dumprawsql',
        'exists',
        'existsor',
        'explain',
        'getbindings',
        'getconnection',
        'getcountforpagination',
        'getgrammar',
        'getrawbindings',
        'implode',
        'insert',
        'insertgetid',
        'insertorignore',
        'insertorignorereturning',
        'insertusing',
        'insertorignoreusing',
        'max',
        'min',
        'numericaggregate',
        'raw',
        'rawvalue',
        'sum',
        'tosql',
        'torawsql',
    ];

    /**
     * Applied global scopes.
     *
     * @var array
     */
    protected $scopes = [];

    /**
     * Removed global scopes.
     *
     * @var array
     */
    protected $removedScopes = [];

    /**
     * The callbacks that should be invoked after retrieving data from the database.
     *
     * @var array
     */
    protected $afterQueryCallbacks = [];

    /**
     * The callbacks that should be invoked on clone.
     *
     * @var array
     */
    protected $onCloneCallbacks = [];

    /**
     * Create a new Eloquent query builder instance.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     */
    public function __construct(QueryBuilder $query)
    {}

    /**
     * Create and return an un-saved model instance.
     *
     * @param  array  $attributes
     * @return TModel
     */
    public function make(array $attributes = [])
    {}

    /**
     * Register a new global scope.
     *
     * @param  string  $identifier
     * @param  \Illuminate\Database\Eloquent\Scope|\Closure  $scope
     * @return $this
     */
    public function withGlobalScope($identifier, $scope)
    {}

    /**
     * Remove a registered global scope.
     *
     * @param  \Illuminate\Database\Eloquent\Scope|string  $scope
     * @return $this
     */
    public function withoutGlobalScope($scope)
    {}

    /**
     * Remove all or passed registered global scopes.
     *
     * @param  array|null  $scopes
     * @return $this
     */
    public function withoutGlobalScopes(?array $scopes = null)
    {}

    /**
     * Remove all global scopes except the given scopes.
     *
     * @param  array  $scopes
     * @return $this
     */
    public function withoutGlobalScopesExcept(array $scopes = [])
    {}

    /**
     * Get an array of global scopes that were removed from the query.
     *
     * @return array
     */
    public function removedScopes()
    {}

    /**
     * Add a where clause on the primary key to the query.
     *
     * @param  mixed  $id
     * @return $this
     */
    public function whereKey($id)
    {}

    /**
     * Add a where clause on the primary key to the query.
     *
     * @param  mixed  $id
     * @return $this
     */
    public function whereKeyNot($id)
    {}

    /**
     * Add an "or where" clause on the primary key to the query.
     *
     * @param  mixed  $id
     * @return $this
     */
    public function orWhereKey($id)
    {}

    /**
     * Add an "or where not" clause on the primary key to the query.
     *
     * @param  mixed  $id
     * @return $this
     */
    public function orWhereKeyNot($id)
    {}

    /**
     * Exclude the given models from the query results.
     *
     * @param  iterable|mixed  $models
     * @return static
     */
    public function except($models)
    {}

    /**
     * Add a basic where clause to the query.
     *
     * @param  (\Closure(static): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function where($column, $operator = null, $value = null, $boolean = 'and')
    { func_get_args(); }

    /**
     * Add a basic where clause to the query, and return the first result.
     *
     * @param  (\Closure(static): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return TModel|null
     */
    public function firstWhere($column, $operator = null, $value = null, $boolean = 'and')
    { func_get_args(); }

    /**
     * Add an "or where" clause to the query.
     *
     * @param  (\Closure(static): mixed)|array|string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhere($column, $operator = null, $value = null)
    { func_get_args(); }

    /**
     * Add a basic "where not" clause to the query.
     *
     * @param  (\Closure(static): mixed)|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereNot($column, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where not" clause to the query.
     *
     * @param  (\Closure(static): mixed)|array|string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereNot($column, $operator = null, $value = null)
    {}

    /**
     * Add an "order by" clause for a timestamp to the query.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|null  $column
     * @return $this
     */
    public function latest($column = null)
    {}

    /**
     * Add an "order by" clause for a timestamp to the query.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|null  $column
     * @return $this
     */
    public function oldest($column = null)
    {}

    /**
     * Create a collection of models from plain arrays.
     *
     * @param  array  $items
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>
     */
    public function hydrate(array $items)
    {}

    /**
     * Insert into the database after merging the model's default attributes, setting timestamps, and casting values.
     *
     * @param  array<int, array<string, mixed>>  $values
     * @return bool
     */
    public function fillAndInsert(array $values)
    {}

    /**
     * Insert (ignoring errors) into the database after merging the model's default attributes, setting timestamps, and casting values.
     *
     * @param  array<int, array<string, mixed>>  $values
     * @return int
     */
    public function fillAndInsertOrIgnore(array $values)
    {}

    /**
     * Insert a record into the database and get its ID after merging the model's default attributes, setting timestamps, and casting values.
     *
     * @param  array<string, mixed>  $values
     * @return int
     */
    public function fillAndInsertGetId(array $values)
    {}

    /**
     * Enrich the given values by merging in the model's default attributes, adding timestamps, and casting values.
     *
     * @param  array<int, array<string, mixed>>  $values
     * @return array<int, array<string, mixed>>
     */
    public function fillForInsert(array $values)
    {}

    /**
     * Create a collection of models from a raw query.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>
     */
    public function fromQuery($query, $bindings = [])
    {}

    /**
     * Find a model by its primary key.
     *
     * @param  mixed  $id
     * @param  array|string  $columns
     * @return ($id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>) ? \Illuminate\Database\Eloquent\Collection<int, TModel> : TModel|null)
     */
    public function find($id, $columns = ['*'])
    {}

    /**
     * Find a sole model by its primary key.
     *
     * @param  mixed  $id
     * @param  array|string  $columns
     * @return TModel
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TModel>
     * @throws \Illuminate\Database\MultipleRecordsFoundException
     */
    public function findSole($id, $columns = ['*'])
    {}

    /**
     * Find multiple models by their primary keys.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $ids
     * @param  array|string  $columns
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>
     */
    public function findMany($ids, $columns = ['*'])
    {}

    /**
     * Find a model by its primary key or throw an exception.
     *
     * @param  mixed  $id
     * @param  array|string  $columns
     * @return ($id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>) ? \Illuminate\Database\Eloquent\Collection<int, TModel> : TModel)
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TModel>
     */
    public function findOrFail($id, $columns = ['*'])
    {}

    /**
     * Find a model by its primary key or return fresh model instance.
     *
     * @param  mixed  $id
     * @param  array|string  $columns
     * @return ($id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>) ? \Illuminate\Database\Eloquent\Collection<int, TModel> : TModel)
     */
    public function findOrNew($id, $columns = ['*'])
    {}

    /**
     * Find a model by its primary key or call a callback.
     *
     * @template TValue
     *
     * @param  mixed  $id
     * @param  (\Closure(): TValue)|list<string>|string  $columns
     * @param  (\Closure(): TValue)|null  $callback
     * @return (
     *     $id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>)
     *     ? \Illuminate\Database\Eloquent\Collection<int, TModel>
     *     : TModel|TValue
     * )
     */
    public function findOr($id, $columns = ['*'], ?Closure $callback = null)
    {}

    /**
     * Get the first record matching the attributes or instantiate it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TModel
     */
    public function firstOrNew(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Get the first record matching the attributes. If the record is not found, create it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TModel
     */
    public function firstOrCreate(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Attempt to create the record. If a unique constraint violation occurs, attempt to find the matching record.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TModel
     *
     * @throws \Illuminate\Database\UniqueConstraintViolationException
     */
    public function createOrFirst(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Create or update a record matching the attributes, and fill it with values.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TModel
     */
    public function updateOrCreate(array $attributes, Closure|array $values = [])
    {}

    /**
     * Create a record matching the attributes, or increment the existing record.
     *
     * @param  array  $attributes
     * @param  string  $column
     * @param  int|float  $default
     * @param  int|float  $step
     * @param  array  $extra
     * @return TModel
     */
    public function incrementOrCreate(array $attributes, string $column = 'count', $default = 1, $step = 1, array $extra = [])
    {}

    /**
     * Execute the query and get the first result or throw an exception.
     *
     * @param  array|string  $columns
     * @return TModel
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TModel>
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
     * @return TModel|TValue
     */
    public function firstOr($columns = ['*'], ?Closure $callback = null)
    {}

    /**
     * Execute the query and get the first result if it's the sole matching record.
     *
     * @param  array|string  $columns
     * @return TModel
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TModel>
     * @throws \Illuminate\Database\MultipleRecordsFoundException
     */
    public function sole($columns = ['*'])
    {}

    /**
     * Get a single column's value from the first result of a query.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return mixed
     */
    public function value($column)
    {}

    /**
     * Get a single column's value from the first result of a query if it's the sole matching record.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return mixed
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TModel>
     * @throws \Illuminate\Database\MultipleRecordsFoundException
     */
    public function soleValue($column)
    {}

    /**
     * Get a single column's value from the first result of the query or throw an exception.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return mixed
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException<TModel>
     */
    public function valueOrFail($column)
    {}

    /**
     * Execute the query as a "select" statement.
     *
     * @param  array|string  $columns
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>
     */
    public function get($columns = ['*'])
    {}

    /**
     * Get the hydrated models without eager loading.
     *
     * @param  array|string  $columns
     * @return array<int, TModel>
     */
    public function getModels($columns = ['*'])
    {}

    /**
     * Eager load the relationships for the models.
     *
     * @param  array<int, TModel>  $models
     * @return array<int, TModel>
     */
    public function eagerLoadRelations(array $models)
    {}

    /**
     * Eagerly load the relationship on a set of models.
     *
     * @param  array  $models
     * @param  string  $name
     * @param  \Closure  $constraints
     * @return array
     */
    protected function eagerLoadRelation(array $models, $name, Closure $constraints)
    {}

    /**
     * Get the relation instance for the given relation name.
     *
     * @param  string  $name
     * @return \Illuminate\Database\Eloquent\Relations\Relation<\Illuminate\Database\Eloquent\Model, TModel, *>
     */
    public function getRelation($name)
    {}

    /**
     * Get the deeply nested relations for a given top-level relation.
     *
     * @param  string  $relation
     * @return array
     */
    protected function relationsNestedUnder($relation)
    {}

    /**
     * Determine if the relationship is nested.
     *
     * @param  string  $relation
     * @param  string  $name
     * @return bool
     */
    protected function isNestedUnder($relation, $name)
    {}

    /**
     * Register a closure to be invoked after the query is executed.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function afterQuery(Closure $callback)
    {}

    /**
     * Invoke the "after query" modification callbacks.
     *
     * @param  mixed  $result
     * @return mixed
     */
    public function applyAfterQueryCallbacks($result)
    {}

    /**
     * Get a lazy collection for the given query.
     *
     * @return \Illuminate\Support\LazyCollection<int, TModel>
     */
    public function cursor()
    {}

    /**
     * Add a generic "order by" clause if the query doesn't already have one.
     *
     * @return void
     */
    protected function enforceOrderBy()
    {}

    /**
     * Get a collection with the values of a given column.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  string|null  $key
     * @return \Illuminate\Support\Collection<array-key, mixed>
     */
    public function pluck($column, $key = null)
    {}

    /**
     * Get an array of primary keys from the query result.
     *
     * @return array<int, array-key>
     */
    public function modelKeys()
    {}

    /**
     * Paginate the given query.
     *
     * @param  int|null|\Closure  $perPage
     * @param  array|string  $columns
     * @param  string  $pageName
     * @param  int|null  $page
     * @param  \Closure|int|null  $total
     * @return \Illuminate\Pagination\LengthAwarePaginator<int, TModel>
     *
     * @throws \InvalidArgumentException
     */
    public function paginate($perPage = null, $columns = ['*'], $pageName = 'page', $page = null, $total = null)
    {}

    /**
     * Paginate the given query into a simple paginator.
     *
     * @param  int|null  $perPage
     * @param  array|string  $columns
     * @param  string  $pageName
     * @param  int|null  $page
     * @return \Illuminate\Pagination\Paginator<int, TModel>
     */
    public function simplePaginate($perPage = null, $columns = ['*'], $pageName = 'page', $page = null)
    {}

    /**
     * Paginate the given query into a cursor paginator.
     *
     * @param  int|null  $perPage
     * @param  array|string  $columns
     * @param  string  $cursorName
     * @param  \Illuminate\Pagination\Cursor|string|null  $cursor
     * @return \Illuminate\Pagination\CursorPaginator<int, TModel>
     */
    public function cursorPaginate($perPage = null, $columns = ['*'], $cursorName = 'cursor', $cursor = null)
    {}

    /**
     * Ensure the proper order by required for cursor pagination.
     *
     * @param  bool  $shouldReverse
     * @return \Illuminate\Support\Collection
     */
    protected function ensureOrderForCursorPagination($shouldReverse = false)
    {}

    /**
     * Save a new model and return the instance.
     *
     * @param  array  $attributes
     * @return TModel
     */
    public function create(array $attributes = [])
    {}

    /**
     * Save a new model and return the instance without raising model events.
     *
     * @param  array  $attributes
     * @return TModel
     */
    public function createQuietly(array $attributes = [])
    {}

    /**
     * Save a new model and return the instance. Allow mass-assignment.
     *
     * @param  array  $attributes
     * @return TModel
     */
    public function forceCreate(array $attributes)
    {}

    /**
     * Save a new model instance with mass assignment without raising model events.
     *
     * @param  array  $attributes
     * @return TModel
     */
    public function forceCreateQuietly(array $attributes = [])
    {}

    /**
     * Update records in the database.
     *
     * @param  array  $values
     * @return int
     */
    public function update(array $values)
    {}

    /**
     * Insert new records or update the existing ones.
     *
     * @param  array  $values
     * @param  array|string  $uniqueBy
     * @param  array|null  $update
     * @return int
     */
    public function upsert(array $values, $uniqueBy, $update = null)
    {}

    /**
     * Update the column's update timestamp.
     *
     * @param  array|string|null  $column
     * @return int|false
     */
    public function touch($column = null)
    {}

    /**
     * Increment a column's value by a given amount.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  float|int  $amount
     * @param  array  $extra
     * @return int
     */
    public function increment($column, $amount = 1, array $extra = [])
    {}

    /**
     * Decrement a column's value by a given amount.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  float|int  $amount
     * @param  array  $extra
     * @return int
     */
    public function decrement($column, $amount = 1, array $extra = [])
    {}

    /**
     * Increment the given column's values by the given amounts.
     *
     * @param  array<string, float|int|numeric-string>  $columns
     * @param  array<string, mixed>  $extra
     * @return int
     */
    public function incrementEach(array $columns, array $extra = [])
    {}

    /**
     * Decrement the given column's values by the given amounts.
     *
     * @param  array<string, float|int|numeric-string>  $columns
     * @param  array<string, mixed>  $extra
     * @return int
     */
    public function decrementEach(array $columns, array $extra = [])
    {}

    /**
     * Add the "updated at" column to an array of values.
     *
     * @param  array  $values
     * @return array
     */
    protected function addUpdatedAtColumn(array $values)
    {}

    /**
     * Add unique IDs to the inserted values.
     *
     * @param  array  $values
     * @return array
     */
    protected function addUniqueIdsToUpsertValues(array $values)
    {}

    /**
     * Add timestamps to the inserted values.
     *
     * @param  array  $values
     * @return array
     */
    protected function addTimestampsToUpsertValues(array $values)
    {}

    /**
     * Add the "updated at" column to the updated columns.
     *
     * @param  array  $update
     * @return array
     */
    protected function addUpdatedAtToUpsertColumns(array $update)
    {}

    /**
     * Delete records from the database.
     *
     * @return mixed
     */
    public function delete()
    {}

    /**
     * Run the default delete function on the builder.
     *
     * Since we do not apply scopes here, the row will actually be deleted.
     *
     * @return mixed
     */
    public function forceDelete()
    {}

    /**
     * Register a replacement for the default delete function.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function onDelete(Closure $callback)
    {}

    /**
     * Determine if the given model has a scope.
     *
     * @param  string  $scope
     * @return bool
     */
    public function hasNamedScope($scope)
    {}

    /**
     * Call the given local model scopes.
     *
     * @param  array|string  $scopes
     * @return static|mixed
     */
    public function scopes($scopes)
    {}

    /**
     * Apply the scopes to the Eloquent builder instance and return it.
     *
     * @return static
     */
    public function applyScopes()
    {}

    /**
     * Apply the given scope on the current builder instance.
     *
     * @param  callable  $scope
     * @param  array  $parameters
     * @return mixed
     */
    protected function callScope(callable $scope, array $parameters = [])
    {}

    /**
     * Apply the given named scope on the current builder instance.
     *
     * @param  string  $scope
     * @param  array  $parameters
     * @return mixed
     */
    protected function callNamedScope($scope, array $parameters = [])
    {}

    /**
     * Nest where conditions by slicing them at the given where count.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  int  $originalWhereCount
     * @return void
     */
    protected function addNewWheresWithinGroup(QueryBuilder $query, $originalWhereCount)
    {}

    /**
     * Slice where conditions at the given offset and add them to the query as a nested condition.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  array  $whereSlice
     * @return void
     */
    protected function groupWhereSliceForScope(QueryBuilder $query, $whereSlice)
    {}

    /**
     * Create a where array with nested where conditions.
     *
     * @param  array  $whereSlice
     * @param  string  $boolean
     * @return array
     */
    protected function createNestedWhere($whereSlice, $boolean = 'and')
    {}

    /**
     * Specify relationships that should be eager loaded.
     *
     * @param  array<array-key, array|(\Closure(\Illuminate\Database\Eloquent\Relations\Relation<*,*,*>): mixed)|string>|string  $relations
     * @param  (\Closure(\Illuminate\Database\Eloquent\Relations\Relation<*,*,*>): mixed)|string|null  $callback
     * @return $this
     */
    public function with($relations, $callback = null)
    { func_get_args(); }

    /**
     * Prevent the specified relations from being eager loaded.
     *
     * @param  mixed  $relations
     * @return $this
     */
    public function without($relations)
    { func_get_args(); }

    /**
     * Set the relationships that should be eager loaded while removing any previously added eager loading specifications.
     *
     * @param  array<array-key, array|(\Closure(\Illuminate\Database\Eloquent\Relations\Relation<*,*,*>): mixed)|string>|string  $relations
     * @return $this
     */
    public function withOnly($relations)
    {}

    /**
     * Create a new instance of the model being queried.
     *
     * @param  array  $attributes
     * @return TModel
     */
    public function newModelInstance($attributes = [])
    {}

    /**
     * Parse a list of relations into individuals.
     *
     * @param  array  $relations
     * @return array
     */
    protected function parseWithRelations(array $relations)
    {}

    /**
     * Prepare nested with relationships.
     *
     * @param  array  $relations
     * @param  string  $prefix
     * @return array
     */
    protected function prepareNestedWithRelationships($relations, $prefix = '')
    {}

    /**
     * Combine an array of constraints into a single constraint.
     *
     * @param  array  $constraints
     * @return \Closure
     */
    protected function combineConstraints(array $constraints)
    {}

    /**
     * Parse the attribute select constraints from the name.
     *
     * @param  string  $name
     * @return array
     */
    protected function parseNameAndAttributeSelectionConstraint($name)
    {}

    /**
     * Create a constraint to select the given columns for the relation.
     *
     * @param  string  $name
     * @return array
     */
    protected function createSelectWithConstraint($name)
    {}

    /**
     * Parse the nested relationships in a relation.
     *
     * @param  string  $name
     * @param  array  $results
     * @return array
     */
    protected function addNestedWiths($name, $results)
    {}

    /**
     * Specify attributes that should be added to any new models created by this builder.
     *
     * The given key / value pairs will also be added as where conditions to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|array|string  $attributes
     * @param  mixed  $value
     * @param  bool  $asConditions
     * @return $this
     */
    public function withAttributes(Expression|array|string $attributes, $value = null, $asConditions = true)
    {}

    /**
     * Apply query-time casts to the model instance.
     *
     * @param  array  $casts
     * @return $this
     */
    public function withCasts($casts)
    {}

    /**
     * Execute the given Closure within a transaction savepoint if needed.
     *
     * @template TModelValue
     *
     * @param  \Closure(): TModelValue  $scope
     * @return TModelValue
     */
    public function withSavepointIfNeeded(Closure $scope): mixed
    {}

    /**
     * Get the Eloquent builder instances that are used in the union of the query.
     *
     * @return \Illuminate\Support\Collection
     */
    protected function getUnionBuilders()
    {}

    /**
     * Get the underlying query builder instance.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function getQuery()
    {}

    /**
     * Set the underlying query builder instance.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @return $this
     */
    public function setQuery($query)
    {}

    /**
     * Get a base query builder instance.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function toBase()
    {}

    /**
     * Get the relationships being eagerly loaded.
     *
     * @return array
     */
    public function getEagerLoads()
    {}

    /**
     * Set the relationships being eagerly loaded.
     *
     * @param  array  $eagerLoad
     * @return $this
     */
    public function setEagerLoads(array $eagerLoad)
    {}

    /**
     * Indicate that the given relationships should not be eagerly loaded.
     *
     * @param  array  $relations
     * @return $this
     */
    public function withoutEagerLoad(array $relations)
    {}

    /**
     * Flush the relationships being eagerly loaded.
     *
     * @return $this
     */
    public function withoutEagerLoads()
    {}

    /**
     * Get the "limit" value from the query or null if it's not set.
     *
     * @return mixed
     */
    public function getLimit()
    {}

    /**
     * Get the "offset" value from the query or null if it's not set.
     *
     * @return mixed
     */
    public function getOffset()
    {}

    /**
     * Get the default key name of the table.
     *
     * @return string
     */
    protected function defaultKeyName()
    {}

    /**
     * Get the model instance being queried.
     *
     * @return TModel
     */
    public function getModel()
    {}

    /**
     * Set a model instance for the model being queried.
     *
     * @template TModelNew of \Illuminate\Database\Eloquent\Model
     *
     * @param  TModelNew  $model
     * @return static<TModelNew>
     */
    public function setModel(Model $model)
    {}

    /**
     * Qualify the given column name by the model's table.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return string
     */
    public function qualifyColumn($column)
    {}

    /**
     * Qualify the given columns with the model's table.
     *
     * @param  array|\Illuminate\Contracts\Database\Query\Expression  $columns
     * @return array
     */
    public function qualifyColumns($columns)
    {}

    /**
     * Get the alias given to the query's table, if any.
     *
     * @return string|null
     */
    protected function getTableAlias()
    {}

    /**
     * Get the given macro by name.
     *
     * @param  string  $name
     * @return \Closure
     */
    public function getMacro($name)
    {}

    /**
     * Checks if a macro is registered.
     *
     * @param  string  $name
     * @return bool
     */
    public function hasMacro($name)
    {}

    /**
     * Get the given global macro by name.
     *
     * @param  string  $name
     * @return \Closure
     */
    public static function getGlobalMacro($name)
    {}

    /**
     * Checks if a global macro is registered.
     *
     * @param  string  $name
     * @return bool
     */
    public static function hasGlobalMacro($name)
    {}

    /**
     * Dynamically access builder proxies.
     *
     * @param  string  $key
     * @return mixed
     *
     * @throws \Exception
     */
    public function __get($key)
    {}

    /**
     * Dynamically handle calls into the query instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}

    /**
     * Dynamically handle calls into the query instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \BadMethodCallException
     */
    public static function __callStatic($method, $parameters)
    {}

    /**
     * Register the given mixin with the builder.
     *
     * @param  string  $mixin
     * @param  bool  $replace
     * @return void
     *
     * @throws \ReflectionException
     */
    protected static function registerMixin($mixin, $replace)
    {}

    /**
     * Clone the Eloquent query builder.
     *
     * @return static
     */
    public function clone()
    {}

    /**
     * Register a closure to be invoked on a clone.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function onClone(Closure $callback)
    {}

    /**
     * Force a clone of the underlying query builder when cloning.
     *
     * @return void
     */
    public function __clone()
    {}
}
