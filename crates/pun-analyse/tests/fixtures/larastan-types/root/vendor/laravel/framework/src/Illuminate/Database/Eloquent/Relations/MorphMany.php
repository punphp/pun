<?php

namespace Illuminate\Database\Eloquent\Relations;

use Illuminate\Database\Eloquent\Collection as EloquentCollection;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 *
 * @extends \Illuminate\Database\Eloquent\Relations\MorphOneOrMany<TRelatedModel, TDeclaringModel, \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>>
 */
class MorphMany extends MorphOneOrMany
{
    /**
     * Convert the relationship to a "morph one" relationship.
     *
     * @return \Illuminate\Database\Eloquent\Relations\MorphOne<TRelatedModel, TDeclaringModel>
     */
    public function one()
    {}

    /** @inheritDoc */
    public function getResults()
    {}

    /** @inheritDoc */
    public function initRelation(array $models, $relation)
    {}

    /** @inheritDoc */
    public function match(array $models, EloquentCollection $results, $relation)
    {}

    /** @inheritDoc */
    public function forceCreate(array $attributes = [])
    {}
}
