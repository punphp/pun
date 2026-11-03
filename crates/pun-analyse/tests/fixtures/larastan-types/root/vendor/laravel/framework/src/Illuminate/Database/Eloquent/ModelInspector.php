<?php

namespace Illuminate\Database\Eloquent;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Support\Collection as BaseCollection;
use Illuminate\Support\Facades\Gate;
use Illuminate\Support\Str;
use ReflectionClass;
use ReflectionMethod;
use ReflectionNamedType;
use SplFileObject;

use function Illuminate\Support\enum_value;

class ModelInspector
{
    /**
     * The methods that can be called in a model to indicate a relation.
     *
     * @var list<string>
     */
    protected $relationMethods = [
        'hasMany',
        'hasManyThrough',
        'hasOneThrough',
        'belongsToMany',
        'hasOne',
        'belongsTo',
        'morphOne',
        'morphTo',
        'morphMany',
        'morphToMany',
        'morphedByMany',
    ];

    /**
     * Create a new model inspector instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app  The Laravel application instance.
     */
    public function __construct(protected Application $app)
    {}

    /**
     * Extract model details for the given model.
     *
     * @param  class-string<\Illuminate\Database\Eloquent\Model>|string  $model
     * @param  string|null  $connection
     * @return \Illuminate\Database\Eloquent\ModelInfo
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    public function inspect($model, $connection = null)
    {}

    /**
     * Get the column attributes for the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return \Illuminate\Support\Collection<int, array<string, mixed>>
     */
    protected function getAttributes($model)
    {}

    /**
     * Get the virtual (non-column) attributes for the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @param  array  $columns
     * @return \Illuminate\Support\Collection
     */
    protected function getVirtualAttributes($model, $columns)
    {}

    /**
     * Get the relations from the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return \Illuminate\Support\Collection
     */
    protected function getRelations($model)
    {}

    /**
     * Get the first policy associated with this model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return string|null
     */
    protected function getPolicy($model)
    {}

    /**
     * Get the events that the model dispatches.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return \Illuminate\Support\Collection
     */
    protected function getEvents($model)
    {}

    /**
     * Get the observers watching this model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return \Illuminate\Support\Collection
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function getObservers($model)
    {}

    /**
     * Get the collection class being used by the model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return class-string<\Illuminate\Database\Eloquent\Collection>
     */
    protected function getCollectedBy($model)
    {}

    /**
     * Get the builder class being used by the model.
     *
     * @template TModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  TModel  $model
     * @return class-string<\Illuminate\Database\Eloquent\Builder<TModel>>
     */
    protected function getBuilder($model)
    {}

    /**
     * Get the class used for JSON response transforming.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return class-string<\Illuminate\Http\Resources\Json\JsonResource>|null
     */
    protected function getResource($model)
    {}

    /**
     * Qualify the given model class base name.
     *
     * @param  string  $model
     * @return class-string<\Illuminate\Database\Eloquent\Model>
     *
     * @see \Illuminate\Console\GeneratorCommand
     */
    protected function qualifyModel(string $model)
    {}

    /**
     * Get the cast type for the given column.
     *
     * @param  string  $column
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return string|null
     */
    protected function getCastType($column, $model)
    {}

    /**
     * Get the model casts, including any date casts.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return \Illuminate\Support\Collection
     */
    protected function getCastsWithDates($model)
    {}

    /**
     * Determine if the given attribute is hidden.
     *
     * @param  string  $attribute
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return bool
     */
    protected function attributeIsHidden($attribute, $model)
    {}

    /**
     * Get the default value for the given column.
     *
     * @param  array<string, mixed>  $column
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return mixed
     */
    protected function getColumnDefault($column, $model)
    {}

    /**
     * Determine if the given attribute is unique.
     *
     * @param  string  $column
     * @param  array  $indexes
     * @return bool
     */
    protected function columnIsUnique($column, $indexes)
    {}
}
