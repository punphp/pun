<?php

namespace Illuminate\Database\Eloquent\Relations;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 *
 * @extends \Illuminate\Database\Eloquent\Relations\HasOneOrManyThrough<TRelatedModel, TIntermediateModel, TDeclaringModel, \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>>
 */
class HasManyThrough extends HasOneOrManyThrough
{
    use InteractsWithDictionary;

    /**
     * Convert the relationship to a "has one through" relationship.
     *
     * @return \Illuminate\Database\Eloquent\Relations\HasOneThrough<TRelatedModel, TIntermediateModel, TDeclaringModel>
     */
    public function one()
    {}

    /** @inheritDoc */
    public function initRelation(array $models, $relation)
    {}

    /** @inheritDoc */
    public function match(array $models, EloquentCollection $results, $relation)
    {}

    /** @inheritDoc */
    public function getResults()
    {}
}
