<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Closure;
use Illuminate\Database\ClassMorphViolationException;
use Illuminate\Database\Eloquent\Attributes\Initialize;
use Illuminate\Database\Eloquent\Attributes\Touches;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\PendingHasThroughRelationship;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\HasOne;
use Illuminate\Database\Eloquent\Relations\HasOneThrough;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\Relations\MorphOne;
use Illuminate\Database\Eloquent\Relations\MorphTo;
use Illuminate\Database\Eloquent\Relations\MorphToMany;
use Illuminate\Database\Eloquent\Relations\Pivot;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Support\Arr;
use Illuminate\Support\Str;

trait HasRelationships
{
    /**
     * The loaded relationships for the model.
     *
     * @var array
     */
    protected $relations = [];

    /**
     * The relationships that should be touched on save.
     *
     * @var array
     */
    protected $touches = [];

    /**
     * The relationship autoloader callback.
     *
     * @var \Closure|null
     */
    protected $relationAutoloadCallback = null;

    /**
     * The relationship autoloader callback context.
     *
     * @var mixed
     */
    protected $relationAutoloadContext = null;

    /**
     * The many to many relationship methods.
     *
     * @var string[]
     */
    public static $manyMethods = [
        'belongsToMany', 'morphToMany', 'morphedByMany',
    ];

    /**
     * The relation resolver callbacks.
     *
     * @var array
     */
    protected static $relationResolvers = [];

    /**
     * Initialize the HasRelationships trait.
     *
     * @return void
     */
    #[Initialize]
    public function initializeHasRelationships()
    {}

    /**
     * Get the dynamic relation resolver if defined or inherited, or return null.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $class
     * @param  string  $key
     * @return Closure|null
     */
    public function relationResolver($class, $key)
    {}

    /**
     * Define a dynamic relation resolver.
     *
     * @param  string  $name
     * @param  \Closure  $callback
     * @return void
     */
    public static function resolveRelationUsing($name, Closure $callback)
    {}

    /**
     * Determine if a relationship autoloader callback has been defined.
     *
     * @return bool
     */
    public function hasRelationAutoloadCallback()
    {}

    /**
     * Define an automatic relationship autoloader callback for this model and its relations.
     *
     * @param  \Closure  $callback
     * @param  mixed  $context
     * @return $this
     */
    public function autoloadRelationsUsing(Closure $callback, $context = null)
    {}

    /**
     * Attempt to autoload the given relationship using the autoload callback.
     *
     * @param  string  $key
     * @return bool
     */
    protected function attemptToAutoloadRelation($key)
    {}

    /**
     * Invoke the relationship autoloader callback for the given relationships.
     *
     * @param  string  $key
     * @param  array  $tuples
     * @return void
     */
    protected function invokeRelationAutoloadCallbackFor($key, $tuples)
    {}

    /**
     * Propagate the relationship autoloader callback to the given related models.
     *
     * @param  string  $key
     * @param  mixed  $models
     * @return void
     */
    protected function propagateRelationAutoloadCallbackToRelation($key, $models)
    {}

    /**
     * Define a one-to-one relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  string|null  $foreignKey
     * @param  string|null  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\HasOne<TRelatedModel, $this>
     */
    public function hasOne($related, $foreignKey = null, $localKey = null)
    {}

    /**
     * Instantiate a new HasOne relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $foreignKey
     * @param  string  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\HasOne<TRelatedModel, TDeclaringModel>
     */
    protected function newHasOne(Builder $query, Model $parent, $foreignKey, $localKey)
    {}

    /**
     * Define a has-one-through relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  class-string<TIntermediateModel>  $through
     * @param  string|null  $firstKey
     * @param  string|null  $secondKey
     * @param  string|null  $localKey
     * @param  string|null  $secondLocalKey
     * @return \Illuminate\Database\Eloquent\Relations\HasOneThrough<TRelatedModel, TIntermediateModel, $this>
     */
    public function hasOneThrough($related, $through, $firstKey = null, $secondKey = null, $localKey = null, $secondLocalKey = null)
    {}

    /**
     * Instantiate a new HasOneThrough relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $farParent
     * @param  TIntermediateModel  $throughParent
     * @param  string  $firstKey
     * @param  string  $secondKey
     * @param  string  $localKey
     * @param  string  $secondLocalKey
     * @return \Illuminate\Database\Eloquent\Relations\HasOneThrough<TRelatedModel, TIntermediateModel, TDeclaringModel>
     */
    protected function newHasOneThrough(Builder $query, Model $farParent, Model $throughParent, $firstKey, $secondKey, $localKey, $secondLocalKey)
    {}

    /**
     * Define a polymorphic one-to-one relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  string  $name
     * @param  string|null  $type
     * @param  string|null  $id
     * @param  string|null  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\MorphOne<TRelatedModel, $this>
     */
    public function morphOne($related, $name, $type = null, $id = null, $localKey = null)
    {}

    /**
     * Instantiate a new MorphOne relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $type
     * @param  string  $id
     * @param  string  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\MorphOne<TRelatedModel, TDeclaringModel>
     */
    protected function newMorphOne(Builder $query, Model $parent, $type, $id, $localKey)
    {}

    /**
     * Define an inverse one-to-one or many relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  string|null  $foreignKey
     * @param  string|null  $ownerKey
     * @param  string|null  $relation
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<TRelatedModel, $this>
     */
    public function belongsTo($related, $foreignKey = null, $ownerKey = null, $relation = null)
    {}

    /**
     * Instantiate a new BelongsTo relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $child
     * @param  string  $foreignKey
     * @param  string  $ownerKey
     * @param  string  $relation
     * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<TRelatedModel, TDeclaringModel>
     */
    protected function newBelongsTo(Builder $query, Model $child, $foreignKey, $ownerKey, $relation)
    {}

    /**
     * Define a polymorphic, inverse one-to-one or many relationship.
     *
     * @param  string|null  $name
     * @param  string|null  $type
     * @param  string|null  $id
     * @param  string|null  $ownerKey
     * @return \Illuminate\Database\Eloquent\Relations\MorphTo<\Illuminate\Database\Eloquent\Model, $this>
     */
    public function morphTo($name = null, $type = null, $id = null, $ownerKey = null)
    {}

    /**
     * Define a polymorphic, inverse one-to-one or many relationship.
     *
     * @param  string  $name
     * @param  string  $type
     * @param  string  $id
     * @param  string|null  $ownerKey
     * @return \Illuminate\Database\Eloquent\Relations\MorphTo<\Illuminate\Database\Eloquent\Model, $this>
     */
    protected function morphEagerTo($name, $type, $id, $ownerKey)
    {}

    /**
     * Define a polymorphic, inverse one-to-one or many relationship.
     *
     * @param  string  $target
     * @param  string  $name
     * @param  string  $type
     * @param  string  $id
     * @param  string|null  $ownerKey
     * @return \Illuminate\Database\Eloquent\Relations\MorphTo<\Illuminate\Database\Eloquent\Model, $this>
     */
    protected function morphInstanceTo($target, $name, $type, $id, $ownerKey)
    {}

    /**
     * Instantiate a new MorphTo relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $foreignKey
     * @param  string|null  $ownerKey
     * @param  string  $type
     * @param  string  $relation
     * @return \Illuminate\Database\Eloquent\Relations\MorphTo<TRelatedModel, TDeclaringModel>
     */
    protected function newMorphTo(Builder $query, Model $parent, $foreignKey, $ownerKey, $type, $relation)
    {}

    /**
     * Retrieve the actual class name for a given morph class.
     *
     * @param  string  $class
     * @return string
     *
     * @throws \Illuminate\Database\ClassMorphViolationException
     */
    public static function getActualClassNameForMorph($class)
    {}

    /**
     * Guess the "belongs to" relationship name.
     *
     * @return string
     */
    protected function guessBelongsToRelation()
    {}

    /**
     * Create a pending has-many-through or has-one-through relationship.
     *
     * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  string|\Illuminate\Database\Eloquent\Relations\HasMany<TIntermediateModel, covariant $this>|\Illuminate\Database\Eloquent\Relations\HasOne<TIntermediateModel, covariant $this>  $relationship
     * @return (
     *     $relationship is string
     *     ? \Illuminate\Database\Eloquent\PendingHasThroughRelationship<\Illuminate\Database\Eloquent\Model, $this>
     *     : (
     *          $relationship is \Illuminate\Database\Eloquent\Relations\HasMany<TIntermediateModel, $this>
     *          ? \Illuminate\Database\Eloquent\PendingHasThroughRelationship<TIntermediateModel, $this, \Illuminate\Database\Eloquent\Relations\HasMany<TIntermediateModel, $this>>
     *          : \Illuminate\Database\Eloquent\PendingHasThroughRelationship<TIntermediateModel, $this, \Illuminate\Database\Eloquent\Relations\HasOne<TIntermediateModel, $this>>
     *     )
     * )
     */
    public function through($relationship)
    {}

    /**
     * Define a one-to-many relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  string|null  $foreignKey
     * @param  string|null  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<TRelatedModel, $this>
     */
    public function hasMany($related, $foreignKey = null, $localKey = null)
    {}

    /**
     * Instantiate a new HasMany relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $foreignKey
     * @param  string  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\HasMany<TRelatedModel, TDeclaringModel>
     */
    protected function newHasMany(Builder $query, Model $parent, $foreignKey, $localKey)
    {}

    /**
     * Define a has-many-through relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  class-string<TIntermediateModel>  $through
     * @param  string|null  $firstKey
     * @param  string|null  $secondKey
     * @param  string|null  $localKey
     * @param  string|null  $secondLocalKey
     * @return \Illuminate\Database\Eloquent\Relations\HasManyThrough<TRelatedModel, TIntermediateModel, $this>
     */
    public function hasManyThrough($related, $through, $firstKey = null, $secondKey = null, $localKey = null, $secondLocalKey = null)
    {}

    /**
     * Instantiate a new HasManyThrough relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $farParent
     * @param  TIntermediateModel  $throughParent
     * @param  string  $firstKey
     * @param  string  $secondKey
     * @param  string  $localKey
     * @param  string  $secondLocalKey
     * @return \Illuminate\Database\Eloquent\Relations\HasManyThrough<TRelatedModel, TIntermediateModel, TDeclaringModel>
     */
    protected function newHasManyThrough(Builder $query, Model $farParent, Model $throughParent, $firstKey, $secondKey, $localKey, $secondLocalKey)
    {}

    /**
     * Define a polymorphic one-to-many relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  string  $name
     * @param  string|null  $type
     * @param  string|null  $id
     * @param  string|null  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\MorphMany<TRelatedModel, $this>
     */
    public function morphMany($related, $name, $type = null, $id = null, $localKey = null)
    {}

    /**
     * Instantiate a new MorphMany relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string  $type
     * @param  string  $id
     * @param  string  $localKey
     * @return \Illuminate\Database\Eloquent\Relations\MorphMany<TRelatedModel, TDeclaringModel>
     */
    protected function newMorphMany(Builder $query, Model $parent, $type, $id, $localKey)
    {}

    /**
     * Define a many-to-many relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TPivotModel of \Illuminate\Database\Eloquent\Relations\Pivot = \Illuminate\Database\Eloquent\Relations\Pivot
     * @template TPivotTable of string|null
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  class-string<TPivotModel>|TPivotTable  $table
     * @param  string|null  $foreignPivotKey
     * @param  string|null  $relatedPivotKey
     * @param  string|null  $parentKey
     * @param  string|null  $relatedKey
     * @param  string|null  $relation
     * @return \Illuminate\Database\Eloquent\Relations\BelongsToMany<TRelatedModel, $this, TPivotModel>
     */
    public function belongsToMany(
        $related,
        $table = null,
        $foreignPivotKey = null,
        $relatedPivotKey = null,
        $parentKey = null,
        $relatedKey = null,
        $relation = null,
    ) {}

    /**
     * Instantiate a new BelongsToMany relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  \Illuminate\Database\Eloquent\Builder<TRelatedModel>  $query
     * @param  TDeclaringModel  $parent
     * @param  string|class-string<\Illuminate\Database\Eloquent\Model>  $table
     * @param  string  $foreignPivotKey
     * @param  string  $relatedPivotKey
     * @param  string  $parentKey
     * @param  string  $relatedKey
     * @param  string|null  $relationName
     * @return \Illuminate\Database\Eloquent\Relations\BelongsToMany<TRelatedModel, TDeclaringModel, \Illuminate\Database\Eloquent\Relations\Pivot>
     */
    protected function newBelongsToMany(
        Builder $query,
        Model $parent,
        $table,
        $foreignPivotKey,
        $relatedPivotKey,
        $parentKey,
        $relatedKey,
        $relationName = null,
    ) {}

    /**
     * Define a polymorphic many-to-many relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  string  $name
     * @param  string|null  $table
     * @param  string|null  $foreignPivotKey
     * @param  string|null  $relatedPivotKey
     * @param  string|null  $parentKey
     * @param  string|null  $relatedKey
     * @param  string|null  $relation
     * @param  bool  $inverse
     * @return \Illuminate\Database\Eloquent\Relations\MorphToMany<TRelatedModel, $this>
     */
    public function morphToMany(
        $related,
        $name,
        $table = null,
        $foreignPivotKey = null,
        $relatedPivotKey = null,
        $parentKey = null,
        $relatedKey = null,
        $relation = null,
        $inverse = false,
    ) {}

    /**
     * Instantiate a new MorphToMany relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
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
     * @return \Illuminate\Database\Eloquent\Relations\MorphToMany<TRelatedModel, TDeclaringModel>
     */
    protected function newMorphToMany(
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
     * Define a polymorphic, inverse many-to-many relationship.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $related
     * @param  string  $name
     * @param  string|null  $table
     * @param  string|null  $foreignPivotKey
     * @param  string|null  $relatedPivotKey
     * @param  string|null  $parentKey
     * @param  string|null  $relatedKey
     * @param  string|null  $relation
     * @return \Illuminate\Database\Eloquent\Relations\MorphToMany<TRelatedModel, $this>
     */
    public function morphedByMany(
        $related,
        $name,
        $table = null,
        $foreignPivotKey = null,
        $relatedPivotKey = null,
        $parentKey = null,
        $relatedKey = null,
        $relation = null,
    ) {}

    /**
     * Get the relationship name of the belongsToMany relationship.
     *
     * @return string|null
     */
    protected function guessBelongsToManyRelation()
    {}

    /**
     * Get the joining table name for a many-to-many relation.
     *
     * @param  string  $related
     * @param  \Illuminate\Database\Eloquent\Model|null  $instance
     * @return string
     */
    public function joiningTable($related, $instance = null)
    {}

    /**
     * Get this model's half of the intermediate table name for belongsToMany relationships.
     *
     * @return string
     */
    public function joiningTableSegment()
    {}

    /**
     * Determine if the model touches a given relation.
     *
     * @param  string  $relation
     * @return bool
     */
    public function touches($relation)
    {}

    /**
     * Touch the owning relations of the model.
     *
     * @return void
     */
    public function touchOwners()
    {}

    /**
     * Get the polymorphic relationship columns.
     *
     * @param  string  $name
     * @param  string  $type
     * @param  string  $id
     * @return array
     */
    protected function getMorphs($name, $type, $id)
    {}

    /**
     * Get the class name for polymorphic relations.
     *
     * @return string
     *
     * @throws \Illuminate\Database\ClassMorphViolationException
     */
    public function getMorphClass()
    {}

    /**
     * Create a new model instance for a related model.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $class
     * @return TRelatedModel
     */
    protected function newRelatedInstance($class)
    {}

    /**
     * Create a new model instance for a related "through" model.
     *
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TRelatedModel>  $class
     * @return TRelatedModel
     */
    protected function newRelatedThroughInstance($class)
    {}

    /**
     * Get all the loaded relations for the instance.
     *
     * @return array
     */
    public function getRelations()
    {}

    /**
     * Get a specified relationship.
     *
     * @param  string  $relation
     * @return mixed
     */
    public function getRelation($relation)
    {}

    /**
     * Determine if the given relation is loaded.
     *
     * @param  string  $key
     * @return bool
     */
    public function relationLoaded($key)
    {}

    /**
     * Set the given relationship on the model.
     *
     * @param  string  $relation
     * @param  mixed  $value
     * @return $this
     */
    public function setRelation($relation, $value)
    {}

    /**
     * Unset a loaded relationship.
     *
     * @param  string  $relation
     * @return $this
     */
    public function unsetRelation($relation)
    {}

    /**
     * Set the entire relations array on the model.
     *
     * @param  array  $relations
     * @return $this
     */
    public function setRelations(array $relations)
    {}

    /**
     * Enable relationship autoloading for this model.
     *
     * @return $this
     */
    public function withRelationshipAutoloading()
    {}

    /**
     * Duplicate the instance and unset all the loaded relations.
     *
     * @return $this
     */
    public function withoutRelations()
    {}

    /**
     * Duplicate the instance and unset the given loaded relations.
     *
     * @param  array|string  $relations
     * @return $this
     */
    public function withoutRelation($relations)
    {}

    /**
     * Unset all the loaded relations for the instance.
     *
     * @return $this
     */
    public function unsetRelations()
    {}

    /**
     * Get the relationships that are touched on save.
     *
     * @return array
     */
    public function getTouchedRelations()
    {}

    /**
     * Set the relationships that are touched on save.
     *
     * @param  array  $touches
     * @return $this
     */
    public function setTouchedRelations(array $touches)
    {}
}
