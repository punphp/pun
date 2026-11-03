<?php

namespace Illuminate\Database\Eloquent\Relations;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Concerns\ComparesRelatedModels;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;
use Illuminate\Database\Eloquent\Relations\Concerns\SupportsDefaultModels;

use function Illuminate\Support\enum_value;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 *
 * @extends \Illuminate\Database\Eloquent\Relations\Relation<TRelatedModel, TDeclaringModel, ?TRelatedModel>
 */
class BelongsTo extends Relation
{
    use ComparesRelatedModels,
        InteractsWithDictionary,
        SupportsDefaultModels;

    /**
     * The child model instance of the relation.
     *
     * @var TDeclaringModel
     */
    protected $child;

    /**
     * The foreign key of the parent model.
     *
     * @var string
     */
    protected $foreignKey;

    /**
     * The associated key on the parent model.
     *
     * @var string
     */
    protected $ownerKey;

    /**
     * The name of the relationship.
     *
     * @var string
     */
    protected $relationName;

    /**
     * Create a new belongs to relationship instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $child
     * @param  string  $foreignKey
     * @param  string  $ownerKey
     * @param  string  $relationName
     */
    public function __construct(Builder $query, Model $child, $foreignKey, $ownerKey, $relationName)
    {}

    /** @inheritDoc */
    public function getResults()
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
     * Gather the keys from an array of related models.
     *
     * @param  array<int, TDeclaringModel>  $models
     * @return array
     */
    protected function getEagerModelKeys(array $models)
    {}

    /** @inheritDoc */
    public function initRelation(array $models, $relation)
    {}

    /** @inheritDoc */
    public function match(array $models, EloquentCollection $results, $relation)
    {}

    /**
     * Associate the model instance to the given parent.
     *
     * @param  TRelatedModel|int|string|null  $model
     * @return TDeclaringModel
     */
    public function associate($model)
    {}

    /**
     * Dissociate previously associated model from the given parent.
     *
     * @return TDeclaringModel
     */
    public function dissociate()
    {}

    /**
     * Alias of "dissociate" method.
     *
     * @return TDeclaringModel
     */
    public function disassociate()
    {}

    /**
     * Touch all of the related models for the relationship.
     *
     * @return void
     */
    public function touch()
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
     * Determine if the related model has an auto-incrementing ID.
     *
     * @return bool
     */
    protected function relationHasIncrementingId()
    {}

    /**
     * Make a new related instance for the given model.
     *
     * @param  TDeclaringModel  $parent
     * @return TRelatedModel
     */
    protected function newRelatedInstanceFor(Model $parent)
    {}

    /**
     * Get the child of the relationship.
     *
     * @return TDeclaringModel
     */
    public function getChild()
    {}

    /**
     * Get the foreign key of the relationship.
     *
     * @return string
     */
    public function getForeignKeyName()
    {}

    /**
     * Get the fully-qualified foreign key of the relationship.
     *
     * @return string
     */
    public function getQualifiedForeignKeyName()
    {}

    /**
     * Get the key value of the child's foreign key.
     *
     * @return mixed
     */
    public function getParentKey()
    {}

    /**
     * Get the associated key of the relationship.
     *
     * @return string
     */
    public function getOwnerKeyName()
    {}

    /**
     * Get the fully-qualified associated key of the relationship.
     *
     * @return string
     */
    public function getQualifiedOwnerKeyName()
    {}

    /**
     * Get the value of the model's foreign key.
     *
     * @param  TRelatedModel  $model
     * @return int|string
     */
    protected function getRelatedKeyFrom(Model $model)
    {}

    /**
     * Get the value of the model's foreign key.
     *
     * @param  TDeclaringModel  $model
     * @return mixed
     */
    protected function getForeignKeyFrom(Model $model)
    {}

    /**
     * Get the name of the relationship.
     *
     * @return string
     */
    public function getRelationName()
    {}
}
