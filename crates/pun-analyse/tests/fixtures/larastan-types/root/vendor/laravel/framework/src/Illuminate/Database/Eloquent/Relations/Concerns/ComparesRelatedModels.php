<?php

namespace Illuminate\Database\Eloquent\Relations\Concerns;

use Illuminate\Contracts\Database\Eloquent\SupportsPartialRelations;
use Illuminate\Database\Eloquent\Model;

trait ComparesRelatedModels
{
    /**
     * Determine if the model is the related instance of the relationship.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $model
     * @return bool
     */
    public function is($model)
    {}

    /**
     * Determine if the model is not the related instance of the relationship.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $model
     * @return bool
     */
    public function isNot($model)
    {}

    /**
     * Get the value of the parent model's key.
     *
     * @return mixed
     */
    abstract public function getParentKey();

    /**
     * Get the value of the model's related key.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return mixed
     */
    abstract protected function getRelatedKeyFrom(Model $model);

    /**
     * Compare the parent key with the related key.
     *
     * @param  mixed  $parentKey
     * @param  mixed  $relatedKey
     * @return bool
     */
    protected function compareKeys($parentKey, $relatedKey)
    {}
}
