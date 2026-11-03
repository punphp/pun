<?php

namespace Illuminate\Database\Eloquent\Relations;

use Closure;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;
use Illuminate\Database\Query\Grammars\MySqlGrammar;
use Illuminate\Database\UniqueConstraintViolationException;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection as BaseCollection;

use function Illuminate\Support\enum_value;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 * @template TResult
 *
 * @extends \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, TIntermediateModel, TResult>
 */
abstract class HasOneOrManyThrough extends Relation
{
    use InteractsWithDictionary;

    /**
     * The "through" parent model instance.
     *
     * @var TIntermediateModel
     */
    protected $throughParent;

    /**
     * The far parent model instance.
     *
     * @var TDeclaringModel
     */
    protected $farParent;

    /**
     * The near key on the relationship.
     *
     * @var string
     */
    protected $firstKey;

    /**
     * The far key on the relationship.
     *
     * @var string
     */
    protected $secondKey;

    /**
     * The local key on the relationship.
     *
     * @var string
     */
    protected $localKey;

    /**
     * The local key on the intermediary model.
     *
     * @var string
     */
    protected $secondLocalKey;

    /**
     * Create a new has many through relationship instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $farParent
     * @param  TIntermediateModel  $throughParent
     * @param  string  $firstKey
     * @param  string  $secondKey
     * @param  string  $localKey
     * @param  string  $secondLocalKey
     */
    public function __construct(Builder $query, Model $farParent, Model $throughParent, $firstKey, $secondKey, $localKey, $secondLocalKey)
    {}

    /**
     * Set the base constraints on the relation query.
     *
     * @return void
     */
    public function addConstraints()
    {}

    /**
     * Set the join clause on the query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>|null  $query
     * @return void
     */
    protected function performJoin(?Builder $query = null)
    {}

    /**
     * Get the fully-qualified parent key name.
     *
     * @return string
     */
    public function getQualifiedParentKeyName()
    {}

    /**
     * Determine whether "through" parent of the relation uses Soft Deletes.
     *
     * @return bool
     */
    public function throughParentSoftDeletes()
    {}

    /**
     * Indicate that trashed "through" parents should be included in the query.
     *
     * @return $this
     */
    public function withTrashedParents()
    {}

    /** @inheritDoc */
    public function addEagerConstraints(array $models)
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
     * Find a related model by its primary key or return a new instance of the related model.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return ($id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>) ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel> : TRelatedModel)
     */
    public function findOrNew($id, $columns = ['*'])
    {}

    /**
     * Get the first related model record matching the attributes or instantiate it.
     *
     * @param  array  $attributes
     * @param  array  $values
     * @return TRelatedModel
     */
    public function firstOrNew(array $attributes = [], array $values = [])
    {}

    /**
     * Get the first record matching the attributes. If the record is not found, create it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TRelatedModel
     */
    public function firstOrCreate(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Attempt to create the record. If a unique constraint violation occurs, attempt to find the matching record.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TRelatedModel
     *
     * @throws \Illuminate\Database\UniqueConstraintViolationException
     */
    public function createOrFirst(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Get the first record matching the attributes, including trashed records, and restore it. If the record is not found, create it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TRelatedModel
     */
    public function restoreOrCreate(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Attempt to create the record. If a unique constraint violation occurs, find the matching record, including trashed records, and restore it.
     *
     * @param  array  $attributes
     * @param  (\Closure(): array)|array  $values
     * @return TRelatedModel
     *
     * @throws \Illuminate\Database\UniqueConstraintViolationException
     */
    public function createOrRestore(array $attributes = [], Closure|array $values = [])
    {}

    /**
     * Create or update a related record matching the attributes, and fill it with values.
     *
     * @param  array  $attributes
     * @param  array  $values
     * @return TRelatedModel
     */
    public function updateOrCreate(array $attributes, array $values = [])
    {}

    /**
     * Qualify the given attribute keys with the related model's table.
     *
     * @param  array  $attributes
     * @return array
     */
    protected function qualifyRelatedAttributes(array $attributes)
    {}

    /**
     * Create a related record matching the attributes, or increment the existing record.
     *
     * @param  array  $attributes
     * @param  string  $column
     * @param  int|float  $default
     * @param  int|float  $step
     * @param  array  $extra
     * @return TRelatedModel
     */
    public function incrementOrCreate(array $attributes, string $column = 'count', $default = 1, $step = 1, array $extra = [])
    {}

    /**
     * Add a basic where clause to the query, and return the first result.
     *
     * @param  \Closure|string|array  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return TRelatedModel|null
     */
    public function firstWhere($column, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Execute the query and get the first related model.
     *
     * @param  array  $columns
     * @return TRelatedModel|null
     */
    public function first($columns = ['*'])
    {}

    /**
     * Execute the query and get the first result or throw an exception.
     *
     * @param  array  $columns
     * @return TRelatedModel
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
     * @return TRelatedModel|TValue
     */
    public function firstOr($columns = ['*'], ?Closure $callback = null)
    {}

    /**
     * Find a related model by its primary key.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return ($id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>) ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel> : TRelatedModel|null)
     */
    public function find($id, $columns = ['*'])
    {}

    /**
     * Find a sole related model by its primary key.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return TRelatedModel
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
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>
     */
    public function findMany($ids, $columns = ['*'])
    {}

    /**
     * Find a related model by its primary key or throw an exception.
     *
     * @param  mixed  $id
     * @param  array  $columns
     * @return ($id is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>) ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel> : TRelatedModel)
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
     *     ? \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>|TValue
     *     : TRelatedModel|TValue
     * )
     */
    public function findOr($id, $columns = ['*'], ?Closure $callback = null)
    {}

    /** @inheritDoc */
    public function get($columns = ['*'])
    {}

    /**
     * Get a paginator for the "select" statement.
     *
     * @param  int|null  $perPage
     * @param  array  $columns
     * @param  string  $pageName
     * @param  int|null  $page
     * @return \Illuminate\Pagination\LengthAwarePaginator
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
     * @return \Illuminate\Contracts\Pagination\Paginator
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
     * @return \Illuminate\Contracts\Pagination\CursorPaginator
     */
    public function cursorPaginate($perPage = null, $columns = ['*'], $cursorName = 'cursor', $cursor = null)
    {}

    /**
     * Set the select clause for the relation query.
     *
     * @param  array  $columns
     * @return array
     */
    protected function shouldSelect(array $columns = ['*'])
    {}

    /**
     * Touch all of the related models for the relationship.
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
     * Get a generator for the given query.
     *
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel>
     */
    public function cursor()
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
     * @param  callable(TRelatedModel): TReturn  $callback
     * @param  int  $count
     * @return \Illuminate\Support\Collection<int, TReturn>
     */
    public function chunkMap(callable $callback, $count = 1000)
    {}

    /**
     * Query lazily, by chunks of the given size.
     *
     * @param  int  $chunkSize
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel>
     */
    public function lazy($chunkSize = 1000)
    {}

    /**
     * Query lazily, by chunking the results of a query by comparing IDs.
     *
     * @param  int  $chunkSize
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel>
     */
    public function lazyById($chunkSize = 1000, $column = null, $alias = null)
    {}

    /**
     * Query lazily, by chunking the results of a query by comparing IDs in descending order.
     *
     * @param  int  $chunkSize
     * @param  string|null  $column
     * @param  string|null  $alias
     * @return \Illuminate\Support\LazyCollection<int, TRelatedModel>
     */
    public function lazyByIdDesc($chunkSize = 1000, $column = null, $alias = null)
    {}

    /**
     * Prepare the query builder for query execution.
     *
     * @param  array  $columns
     * @return \Illuminate\Database\Eloquent\Builder<TRelatedModel>
     */
    protected function prepareQueryBuilder($columns = ['*'])
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
    public function getRelationExistenceQueryForSelfRelation(Builder $query, Builder $parentQuery, $columns = ['*'])
    {}

    /**
     * Add the constraints for a relationship query on the same table as the through parent.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  \Illuminate\Database\Eloquent\Builder<TDeclaringModel>  $parentQuery
     * @param  mixed  $columns
     * @return \Illuminate\Database\Eloquent\Builder<TRelatedModel>
     */
    public function getRelationExistenceQueryForThroughSelfRelation(Builder $query, Builder $parentQuery, $columns = ['*'])
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
     * Get the qualified foreign key on the related model.
     *
     * @return string
     */
    public function getQualifiedFarKeyName()
    {}

    /**
     * Get the foreign key on the "through" model.
     *
     * @return string
     */
    public function getFirstKeyName()
    {}

    /**
     * Get the qualified foreign key on the "through" model.
     *
     * @return string
     */
    public function getQualifiedFirstKeyName()
    {}

    /**
     * Get the foreign key on the related model.
     *
     * @return string
     */
    public function getForeignKeyName()
    {}

    /**
     * Get the qualified foreign key on the related model.
     *
     * @return string
     */
    public function getQualifiedForeignKeyName()
    {}

    /**
     * Get the local key on the far parent model.
     *
     * @return string
     */
    public function getLocalKeyName()
    {}

    /**
     * Get the qualified local key on the far parent model.
     *
     * @return string
     */
    public function getQualifiedLocalKeyName()
    {}

    /**
     * Get the local key on the intermediary model.
     *
     * @return string
     */
    public function getSecondLocalKeyName()
    {}
}
