<?php

namespace Illuminate\Database\Eloquent\Relations;

use BadMethodCallException;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;
use Illuminate\Support\Arr;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 *
 * @extends \Illuminate\Database\Eloquent\Relations\BelongsTo<TRelatedModel, TDeclaringModel>
 */
class MorphTo extends BelongsTo
{
    use InteractsWithDictionary;

    /**
     * The type of the polymorphic relation.
     *
     * @var string
     */
    protected $morphType;

    /**
     * The associated key on the parent model.
     *
     * @var string|null
     */
    protected $ownerKey;

    /**
     * The models whose relations are being eager loaded.
     *
     * @var \Illuminate\Database\Eloquent\Collection<int, TDeclaringModel>
     */
    protected $models;

    /**
     * All of the models keyed by ID.
     *
     * @var array
     */
    protected $dictionary = [];

    /**
     * A buffer of dynamic calls to query macros.
     *
     * @var array
     */
    protected $macroBuffer = [];

    /**
     * A map of relations to load for each individual morph type.
     *
     * @var array
     */
    protected $morphableEagerLoads = [];

    /**
     * A map of relationship counts to load for each individual morph type.
     *
     * @var array
     */
    protected $morphableEagerLoadCounts = [];

    /**
     * A map of constraints to apply for each individual morph type.
     *
     * @var array
     */
    protected $morphableConstraints = [];

    /**
     * Create a new morph to relationship instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $foreignKey
     * @param  string|null  $ownerKey
     * @param  string  $type
     * @param  string  $relation
     */
    public function __construct(Builder $query, Model $parent, $foreignKey, $ownerKey, $type, $relation)
    {}

    /** @inheritDoc */
    #[\Override]
    public function addEagerConstraints(array $models)
    {}

    /**
     * Build a dictionary with the models.
     *
     * @param  \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>  $models
     * @return void
     */
    protected function buildDictionary(EloquentCollection $models)
    {}

    /**
     * Get the results of the relationship.
     *
     * Called via eager load method of Eloquent query builder.
     *
     * @return \Illuminate\Database\Eloquent\Collection<int, TDeclaringModel>
     */
    public function getEager()
    {}

    /**
     * Get all of the relation results for a type.
     *
     * @param  string  $type
     * @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>
     */
    protected function getResultsByType($type)
    {}

    /**
     * Gather all of the foreign keys for a given type.
     *
     * @param  string  $type
     * @param  string  $keyType
     * @return array
     */
    protected function gatherKeysByType($type, $keyType)
    {}

    /**
     * Create a new model instance by type.
     *
     * @param  string  $type
     * @return TRelatedModel
     */
    public function createModelByType($type)
    {}

    /** @inheritDoc */
    #[\Override]
    public function match(array $models, EloquentCollection $results, $relation)
    {}

    /**
     * Match the results for a given type to their parents.
     *
     * @param  string  $type
     * @param  \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>  $results
     * @return void
     */
    protected function matchToMorphParents($type, EloquentCollection $results)
    {}

    /**
     * Associate the model instance to the given parent.
     *
     * @param  TRelatedModel|null  $model
     * @return TDeclaringModel
     */
    #[\Override]
    public function associate($model)
    {}

    /**
     * Dissociate previously associated model from the given parent.
     *
     * @return TDeclaringModel
     */
    #[\Override]
    public function dissociate()
    {}

    /** @inheritDoc */
    #[\Override]
    public function touch()
    {}

    /** @inheritDoc */
    #[\Override]
    protected function newRelatedInstanceFor(Model $parent)
    {}

    /**
     * Get the foreign key "type" name.
     *
     * @return string
     */
    public function getMorphType()
    {}

    /**
     * Get the dictionary used by the relationship.
     *
     * @return array
     */
    public function getDictionary()
    {}

    /**
     * Specify which relations to load for a given morph type.
     *
     * @param  array  $with
     * @return $this
     */
    public function morphWith(array $with)
    {}

    /**
     * Specify which relationship counts to load for a given morph type.
     *
     * @param  array  $withCount
     * @return $this
     */
    public function morphWithCount(array $withCount)
    {}

    /**
     * Specify constraints on the query for a given morph type.
     *
     * @param  array  $callbacks
     * @return $this
     */
    public function constrain(array $callbacks)
    {}

    /**
     * Resolve morph map aliases used as keys to their class names.
     *
     * @param  array  $values
     * @return array
     */
    protected function resolveMorphTypes(array $values)
    {}

    /**
     * Indicate that soft deleted models should be included in the results.
     *
     * @return $this
     */
    public function withTrashed()
    {}

    /**
     * Indicate that soft deleted models should not be included in the results.
     *
     * @return $this
     */
    public function withoutTrashed()
    {}

    /**
     * Indicate that only soft deleted models should be included in the results.
     *
     * @return $this
     */
    public function onlyTrashed()
    {}

    /**
     * Replay stored macro calls on the actual related instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @return \Illuminate\Database\Eloquent\Builder<TRelatedModel>
     */
    public function replayMacros(Builder $query)
    {}

    /** @inheritDoc */
    #[\Override]
    public function getQualifiedOwnerKeyName()
    {}

    /**
     * Handle dynamic method calls to the relationship.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
