<?php

namespace Illuminate\Database\Eloquent\Factories;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasOneOrMany;
use Illuminate\Database\Eloquent\Relations\MorphOneOrMany;

class Relationship
{
    /**
     * The related factory instance.
     *
     * @var \Illuminate\Database\Eloquent\Factories\Factory
     */
    protected $factory;

    /**
     * The relationship name.
     *
     * @var string
     */
    protected $relationship;

    /**
     * Create a new child relationship instance.
     *
     * @param  \Illuminate\Database\Eloquent\Factories\Factory  $factory
     * @param  string  $relationship
     */
    public function __construct(Factory $factory, $relationship)
    {}

    /**
     * Create the child relationship for the given parent model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $parent
     * @return void
     */
    public function createFor(Model $parent)
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
