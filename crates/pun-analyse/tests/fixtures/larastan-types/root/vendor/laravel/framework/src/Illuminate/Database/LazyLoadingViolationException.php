<?php

namespace Illuminate\Database;

use RuntimeException;

class LazyLoadingViolationException extends RuntimeException
{
    /**
     * The name of the affected Eloquent model.
     *
     * @var string
     */
    public $model;

    /**
     * The name of the relation.
     *
     * @var string
     */
    public $relation;

    /**
     * Create a new exception instance.
     *
     * @param  object  $model
     * @param  string  $relation
     */
    public function __construct($model, $relation)
    {}
}
