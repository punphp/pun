<?php

namespace Illuminate\Database\Eloquent\Relations;

use Illuminate\Contracts\Database\Eloquent\SupportsPartialRelations;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\Concerns\CanBeOneOfMany;
use Illuminate\Database\Eloquent\Relations\Concerns\ComparesRelatedModels;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;
use Illuminate\Database\Eloquent\Relations\Concerns\SupportsDefaultModels;
use Illuminate\Database\Query\JoinClause;
use Illuminate\Support\Str;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 *
 * @extends \Illuminate\Database\Eloquent\Relations\HasOneOrManyThrough<TRelatedModel, TIntermediateModel, TDeclaringModel, ?TRelatedModel>
 */
class HasOneThrough extends HasOneOrManyThrough implements SupportsPartialRelations
{
    use ComparesRelatedModels, CanBeOneOfMany, InteractsWithDictionary, SupportsDefaultModels;

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
    public function getRelationExistenceQuery(Builder $query, Builder $parentQuery, $columns = ['*'])
    {}

    /**
     * Point the one-of-many join at the aliased intermediate table of a self-referencing existence query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @return void
     */
    protected function qualifyOneOfManyJoinWithThroughParentAlias(Builder $query)
    {}

    /** @inheritDoc */
    public function addOneOfManySubQueryConstraints(Builder $query, $column = null, $aggregate = null)
    {}

    /** @inheritDoc */
    public function getOneOfManySubQuerySelectColumns()
    {}

    /** @inheritDoc */
    public function addOneOfManyJoinSubQueryConstraints(JoinClause $join)
    {}

    /**
     * Make a new related instance for the given model.
     *
     * @param  TDeclaringModel  $parent
     * @return TRelatedModel
     */
    public function newRelatedInstanceFor(Model $parent)
    {}

    /** @inheritDoc */
    public function is($model)
    {}

    /** @inheritDoc */
    protected function getRelatedKeyFrom(Model $model)
    {}

    /** @inheritDoc */
    public function getParentKey()
    {}
}
