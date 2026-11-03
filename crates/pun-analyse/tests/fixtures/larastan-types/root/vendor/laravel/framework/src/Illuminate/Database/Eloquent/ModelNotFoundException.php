<?php

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\RecordsNotFoundException;
use Illuminate\Support\Arr;

use function Illuminate\Support\enum_value;

/**
 * @template TModel of \Illuminate\Database\Eloquent\Model
 */
class ModelNotFoundException extends RecordsNotFoundException
{
    /**
     * Name of the affected Eloquent model.
     *
     * @var class-string<TModel>
     */
    protected $model;

    /**
     * The affected model IDs.
     *
     * @var array<int, int|string>
     */
    protected $ids;

    /**
     * Set the affected Eloquent model and instance ids.
     *
     * @param  class-string<TModel>  $model
     * @param  array<int, int|string>|int|string  $ids
     * @return $this
     */
    public function setModel($model, $ids = [])
    {}

    /**
     * Get the affected Eloquent model.
     *
     * @return class-string<TModel>
     */
    public function getModel()
    {}

    /**
     * Get the affected Eloquent model IDs.
     *
     * @return array<int, int|string>
     */
    public function getIds()
    {}
}
