<?php

namespace Illuminate\Database;

use RuntimeException;

class ClassMorphViolationException extends RuntimeException
{
    /**
     * The name of the affected Eloquent model.
     *
     * @var string
     */
    public $model;

    /**
     * Create a new exception instance.
     *
     * @param  object|string  $model
     */
    public function __construct($model)
    {}
}
