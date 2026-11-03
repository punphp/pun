<?php

namespace Illuminate\Database\Eloquent\Relations\Concerns;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\RelationNotFoundException;
use Illuminate\Support\Arr;
use Illuminate\Support\Str;

trait SupportsInverseRelations
{
    /**
     * The name of the inverse relationship.
     *
     * @var string|null
     */
    protected ?string $inverseRelationship = null;

    /**
     * Instruct Eloquent to link the related models back to the parent after the relationship query has run.
     *
     * Alias of "chaperone".
     *
     * @param  string|null  $relation
     * @return $this
     */
    public function inverse(?string $relation = null)
    {}

    /**
     * Instruct Eloquent to link the related models back to the parent after the relationship query has run.
     *
     * @param  string|null  $relation
     * @return $this
     */
    public function chaperone(?string $relation = null)
    {}

    /**
     * Guess the name of the inverse relationship.
     *
     * @return string|null
     */
    protected function guessInverseRelation(): ?string
    {}

    /**
     * Get the possible inverse relations for the parent model.
     *
     * @return array<non-empty-string>
     */
    protected function getPossibleInverseRelations(): array
    {}

    /**
     * Set the inverse relation on all models in a collection.
     *
     * @param  \Illuminate\Database\Eloquent\Collection  $models
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return \Illuminate\Database\Eloquent\Collection
     */
    protected function applyInverseRelationToCollection($models, ?Model $parent = null)
    {}

    /**
     * Set the inverse relation on a model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return \Illuminate\Database\Eloquent\Model
     */
    protected function applyInverseRelationToModel(Model $model, ?Model $parent = null)
    {}

    /**
     * Get the name of the inverse relationship.
     *
     * @return string|null
     */
    public function getInverseRelationship()
    {}

    /**
     * Remove the chaperone / inverse relationship for this query.
     *
     * Alias of "withoutChaperone".
     *
     * @return $this
     */
    public function withoutInverse()
    {}

    /**
     * Remove the chaperone / inverse relationship for this query.
     *
     * @return $this
     */
    public function withoutChaperone()
    {}
}
