<?php

namespace Illuminate\Database\Eloquent\Factories;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Collection;

class BelongsToManyRelationship
{
    /**
     * The related factory instance.
     *
     * @var \Illuminate\Database\Eloquent\Factories\Factory|\Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array
     */
    protected $factory;

    /**
     * The pivot attributes / attribute resolver.
     *
     * @var callable|array
     */
    protected $pivot;

    /**
     * The relationship name.
     *
     * @var string
     */
    protected $relationship;

    /**
     * Create a new attached relationship definition.
     *
     * @param  \Illuminate\Database\Eloquent\Factories\Factory|\Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array  $factory
     * @param  callable|array  $pivot
     * @param  string  $relationship
     */
    public function __construct($factory, $pivot, $relationship)
    {}

    /**
     * Create the attached relationship for the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return void
     */
    public function createFor(Model $model)
    {}

    /**
     * Specify the model instances to always use when creating relationships.
     *
     * @param  \Illuminate\Support\Collection  $recycle
     * @return static
     */
    public function recycle($recycle)
    {}
}
