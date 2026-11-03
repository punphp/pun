<?php

namespace Illuminate\Database\Eloquent\Relations;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;

/**
 * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
 * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
 * @template TPivotModel of \Illuminate\Database\Eloquent\Relations\Pivot = \Illuminate\Database\Eloquent\Relations\MorphPivot
 * @template TAccessor of string = 'pivot'
 *
 * @extends \Illuminate\Database\Eloquent\Relations\BelongsToMany<TRelatedModel, TDeclaringModel, TPivotModel, TAccessor>
 */
class MorphToMany extends BelongsToMany
{
    /**
     * The type of the polymorphic relation.
     *
     * @var string
     */
    protected $morphType;

    /**
     * The morph class of the morph type constraint.
     *
     * @var class-string|string
     */
    protected $morphClass;

    /**
     * Indicates if we are connecting the inverse of the relation.
     *
     * This primarily affects the morphClass constraint.
     *
     * @var bool
     */
    protected $inverse;

    /**
     * Create a new morph to many relationship instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $name
     * @param  string  $table
     * @param  string  $foreignPivotKey
     * @param  string  $relatedPivotKey
     * @param  string  $parentKey
     * @param  string  $relatedKey
     * @param  string|null  $relationName
     * @param  bool  $inverse
     */
    public function __construct(
        Builder $query,
        Model $parent,
        $name,
        $table,
        $foreignPivotKey,
        $relatedPivotKey,
        $parentKey,
        $relatedKey,
        $relationName = null,
        $inverse = false,
    ) {}

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

    /**
     * Create a new pivot attachment record.
     *
     * @param  int  $id
     * @param  bool  $timed
     * @return array
     */
    protected function baseAttachRecord($id, $timed)
    {}

    /** @inheritDoc */
    public function getRelationExistenceQuery(Builder $query, Builder $parentQuery, $columns = ['*'])
    {}

    /**
     * Get the pivot models that are currently attached, filtered by related model keys.
     *
     * @param  mixed  $ids
     * @return \Illuminate\Support\Collection<int, TPivotModel>
     */
    protected function getCurrentlyAttachedPivotsForIds($ids = null)
    {}

    /**
     * Create a new query builder for the pivot table.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function newPivotQuery()
    {}

    /**
     * Create a new pivot model instance.
     *
     * @param  array  $attributes
     * @param  bool  $exists
     * @return TPivotModel
     */
    public function newPivot(array $attributes = [], $exists = false)
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
     * Get the foreign key "type" name.
     *
     * @return string
     */
    public function getMorphType()
    {}

    /**
     * Get the fully-qualified morph type for the relation.
     *
     * @return string
     */
    public function getQualifiedMorphTypeName()
    {}

    /**
     * Get the class name of the parent model.
     *
     * @return class-string|string
     */
    public function getMorphClass()
    {}

    /**
     * Get the indicator for a reverse relationship.
     *
     * @return bool
     */
    public function getInverse()
    {}
}
