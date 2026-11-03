<?php

namespace Illuminate\Database\Eloquent\Relations;

use Closure;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;
use Illuminate\Database\Eloquent\Relations\Concerns\SupportsInverseRelations;
use Illuminate\Database\UniqueConstraintViolationException;
use Illuminate\Support\Arr;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 * @template TResult
 *
 * @extends \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, TDeclaringModel, TResult>
 */
abstract class HasOneOrMany extends Relation
{
    use InteractsWithDictionary, SupportsInverseRelations;

    /**
     * The foreign key of the parent model.
     *
     * @var string
     */
    protected $foreignKey;

    /**
     * The local key of the parent model.
     *
     * @var string
     */
    protected $localKey;

    /**
     * Create a new has one or many relationship instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $foreignKey
     * @param  string  $localKey
     */
    public function __construct(Builder $query, Model $parent, $foreignKey, $localKey)
    {}

    /**
     * Create and return an un-saved instance of the related model.
     *
     * @param  array  $attributes
     * @return TRelatedModel
     */
    public function make(array $attributes = [])
    {}

    /**
     * Create and return an un-saved instance of the related models.
     *
     * @param  iterable  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>
     */
    public function makeMany($records)
    {}

    /**
     * Set the base constraints on the relation query.
     *
     * @return void
     */
    public function addConstraints()
    {}

    /** @inheritDoc */
    public function addEagerConstraints(array $models)
    {}

    /**
     * Match the eagerly loaded results to their single parents.
     *
     * @param  array<int, TDeclaringModel>  $models
     * @param  \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>  $results
     * @param  string  $relation
     * @return array<int, TDeclaringModel>
     */
    public function matchOne(array $models, EloquentCollection $results, $relation)
    {}

    /**
     * Match the eagerly loaded results to their many parents.
     *
     * @param  array<int, TDeclaringModel>  $models
     * @param  \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>  $results
     * @param  string  $relation
     * @return array<int, TDeclaringModel>
     */
    public function matchMany(array $models, EloquentCollection $results, $relation)
    {}

    /**
     * Match the eagerly loaded results to their many parents.
     *
     * @param  array<int, TDeclaringModel>  $models
     * @param  \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>  $results
     * @param  string  $relation
     * @param  string  $type
     * @return array<int, TDeclaringModel>
     */
    protected function matchOneOrMany(array $models, EloquentCollection $results, $relation, $type)
    {}

    /**
     * Get the value of a relationship by one or many type.
     *
     * @param  array  $dictionary
     * @param  string  $key
     * @param  string  $type
     * @return mixed
     */
    protected function getRelationValue(array $dictionary, $key, $type)
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
     * Find a model by its primary key or return a new instance of the related model.
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
     * @param  (\Closure(): array)|array  $values
     * @return TRelatedModel
     */
    public function firstOrNew(array $attributes = [], Closure|array $values = [])
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
     * @param  (\Closure(): array)|array  $values
     * @return TRelatedModel
     */
    public function updateOrCreate(array $attributes, Closure|array $values = [])
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
     * Attach a model instance to the parent model.
     *
     * @param  TRelatedModel  $model
     * @return TRelatedModel|false
     */
    public function save(Model $model)
    {}

    /**
     * Attach a model instance without raising any events to the parent model.
     *
     * @param  TRelatedModel  $model
     * @return TRelatedModel|false
     */
    public function saveQuietly(Model $model)
    {}

    /**
     * Attach a collection of models to the parent instance.
     *
     * @param  iterable<TRelatedModel>  $models
     * @return iterable<TRelatedModel>
     */
    public function saveMany($models)
    {}

    /**
     * Attach a collection of models to the parent instance without raising any events to the parent model.
     *
     * @param  iterable<TRelatedModel>  $models
     * @return iterable<TRelatedModel>
     */
    public function saveManyQuietly($models)
    {}

    /**
     * Create a new instance of the related model.
     *
     * @param  array  $attributes
     * @return TRelatedModel
     */
    public function create(array $attributes = [])
    {}

    /**
     * Create a new instance of the related model without raising any events to the parent model.
     *
     * @param  array  $attributes
     * @return TRelatedModel
     */
    public function createQuietly(array $attributes = [])
    {}

    /**
     * Create a new instance of the related model. Allow mass-assignment.
     *
     * @param  array  $attributes
     * @return TRelatedModel
     */
    public function forceCreate(array $attributes = [])
    {}

    /**
     * Create a new instance of the related model with mass assignment without raising model events.
     *
     * @param  array  $attributes
     * @return TRelatedModel
     */
    public function forceCreateQuietly(array $attributes = [])
    {}

    /**
     * Create a Collection of new instances of the related model.
     *
     * @param  iterable  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>
     */
    public function createMany(iterable $records)
    {}

    /**
     * Create a Collection of new instances of the related model without raising any events to the parent model.
     *
     * @param  iterable  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>
     */
    public function createManyQuietly(iterable $records)
    {}

    /**
     * Create a Collection of new instances of the related model, allowing mass-assignment.
     *
     * @param  iterable  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>
     */
    public function forceCreateMany(iterable $records)
    {}

    /**
     * Create a Collection of new instances of the related model, allowing mass-assignment and without raising any events to the parent model.
     *
     * @param  iterable  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>
     */
    public function forceCreateManyQuietly(iterable $records)
    {}

    /**
     * Set the foreign ID for creating a related model.
     *
     * @param  TRelatedModel  $model
     * @return void
     */
    protected function setForeignAttributesForCreate(Model $model)
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
     * Get the key value of the parent's local key.
     *
     * @return mixed
     */
    public function getParentKey()
    {}

    /**
     * Get the fully-qualified parent key name.
     *
     * @return string
     */
    public function getQualifiedParentKeyName()
    {}

    /**
     * Get the plain foreign key.
     *
     * @return string
     */
    public function getForeignKeyName()
    {}

    /**
     * Get the foreign key for the relationship.
     *
     * @return string
     */
    public function getQualifiedForeignKeyName()
    {}

    /**
     * Get the local key for the relationship.
     *
     * @return string
     */
    public function getLocalKeyName()
    {}
}
