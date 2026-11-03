<?php

namespace Illuminate\Database\Eloquent;

use ArrayAccess;
use Closure;
use Exception;
use Illuminate\Contracts\Broadcasting\HasBroadcastChannel;
use Illuminate\Contracts\Queue\QueueableCollection;
use Illuminate\Contracts\Queue\QueueableEntity;
use Illuminate\Contracts\Routing\UrlRoutable;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\CanBeEscapedWhenCastToString;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Database\ConnectionResolverInterface as Resolver;
use Illuminate\Database\Eloquent\Attributes\Boot;
use Illuminate\Database\Eloquent\Attributes\Connection;
use Illuminate\Database\Eloquent\Attributes\Initialize;
use Illuminate\Database\Eloquent\Attributes\Refreshes;
use Illuminate\Database\Eloquent\Attributes\RouteKey;
use Illuminate\Database\Eloquent\Attributes\Scope as LocalScope;
use Illuminate\Database\Eloquent\Attributes\Table;
use Illuminate\Database\Eloquent\Attributes\UseEloquentBuilder;
use Illuminate\Database\Eloquent\Attributes\WithoutIncrementing;
use Illuminate\Database\Eloquent\Attributes\WithoutTimestamps;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\Concerns\AsPivot;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\Pivot;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection as BaseCollection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable as SupportStringable;
use Illuminate\Support\Traits\ForwardsCalls;
use JsonException;
use JsonSerializable;
use LogicException;
use ReflectionClass;
use ReflectionMethod;
use Stringable;

use function Illuminate\Support\enum_value;

abstract class Model implements Arrayable, ArrayAccess, CanBeEscapedWhenCastToString, HasBroadcastChannel, Jsonable, JsonSerializable, QueueableEntity, Stringable, UrlRoutable
{
    use Concerns\HasAttributes,
        Concerns\HasEvents,
        Concerns\HasGlobalScopes,
        Concerns\HasRelationships,
        Concerns\HasTimestamps,
        Concerns\HasUniqueIds,
        Concerns\HidesAttributes,
        Concerns\GuardsAttributes,
        Concerns\PreventsCircularRecursion,
        Concerns\TransformsToResource,
        ForwardsCalls;
    /** @use HasCollection<\Illuminate\Database\Eloquent\Collection<array-key, static & self>> */
    use HasCollection;

    /**
     * The connection name for the model.
     *
     * @var \UnitEnum|string|null
     */
    protected $connection;

    /**
     * The table associated with the model.
     *
     * @var string|null
     */
    protected $table;

    /**
     * The primary key for the model.
     *
     * @var string
     */
    protected $primaryKey = 'id';

    /**
     * The "type" of the primary key ID.
     *
     * @var string
     */
    protected $keyType = 'int';

    /**
     * Indicates if the IDs are auto-incrementing.
     *
     * @var bool
     */
    public $incrementing = true;

    /**
     * The relations to eager load on every query.
     *
     * @var array
     */
    protected $with = [];

    /**
     * The relationship counts that should be eager loaded on every query.
     *
     * @var array
     */
    protected $withCount = [];

    /**
     * The attributes that should be refreshed after the model is written.
     *
     * @var list<string>
     */
    protected array $refreshes = [];

    /**
     * Indicates whether lazy loading will be prevented on this model.
     *
     * @var bool
     */
    public $preventsLazyLoading = false;

    /**
     * The number of models to return for pagination.
     *
     * @var int
     */
    protected $perPage = 15;

    /**
     * Indicates if the model exists.
     *
     * @var bool
     */
    public $exists = false;

    /**
     * Indicates if the model was inserted during the object's lifecycle.
     *
     * @var bool
     */
    public $wasRecentlyCreated = false;

    /**
     * Indicates that the object's string representation should be escaped when __toString is invoked.
     *
     * @var bool
     */
    protected $escapeWhenCastingToString = false;

    /**
     * The connection resolver instance.
     *
     * @var \Illuminate\Database\ConnectionResolverInterface
     */
    protected static $resolver;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected static $dispatcher;

    /**
     * The models that are currently being booted.
     *
     * @var array
     */
    protected static $booting = [];

    /**
     * The array of booted models.
     *
     * @var array
     */
    protected static $booted = [];

    /**
     * The callbacks that should be executed after the model has booted.
     *
     * @var array
     */
    protected static $bootedCallbacks = [];

    /**
     * The array of trait initializers that will be called on each new instance.
     *
     * @var array
     */
    protected static $traitInitializers = [];

    /**
     * The array of global scopes on the model.
     *
     * @var array
     */
    protected static $globalScopes = [];

    /**
     * The list of models classes that should not be affected with touch.
     *
     * @var array
     */
    protected static $ignoreOnTouch = [];

    /**
     * Indicates whether lazy loading should be restricted on all models.
     *
     * @var bool
     */
    protected static $modelsShouldPreventLazyLoading = false;

    /**
     * Indicates whether relations should be automatically loaded on all models when they are accessed.
     *
     * @var bool
     */
    protected static $modelsShouldAutomaticallyEagerLoadRelationships = false;

    /**
     * The callback that is responsible for handling lazy loading violations.
     *
     * @var (callable(self, string): mixed)|null
     */
    protected static $lazyLoadingViolationCallback;

    /**
     * Indicates if an exception should be thrown instead of silently discarding non-fillable attributes.
     *
     * @var bool
     */
    protected static $modelsShouldPreventSilentlyDiscardingAttributes = false;

    /**
     * The callback that is responsible for handling discarded attribute violations.
     *
     * @var (callable(self, array): mixed)|null
     */
    protected static $discardedAttributeViolationCallback;

    /**
     * Indicates if an exception should be thrown when trying to access a missing attribute on a retrieved model.
     *
     * @var bool
     */
    protected static $modelsShouldPreventAccessingMissingAttributes = false;

    /**
     * The callback that is responsible for handling missing attribute violations.
     *
     * @var (callable(self, string): mixed)|null
     */
    protected static $missingAttributeViolationCallback;

    /**
     * Indicates if broadcasting is currently enabled.
     *
     * @var bool
     */
    protected static $isBroadcasting = true;

    /**
     * The Eloquent query builder class to use for the model.
     *
     * @var class-string<\Illuminate\Database\Eloquent\Builder<*>>
     */
    protected static string $builder = Builder::class;

    /**
     * The Eloquent collection class to use for the model.
     *
     * @var class-string<\Illuminate\Database\Eloquent\Collection<*, *>>
     */
    protected static string $collectionClass = Collection::class;

    /**
     * Cache of soft deletable models.
     *
     * @var array<class-string<self>, bool>
     */
    protected static array $isSoftDeletable;

    /**
     * Cache of prunable models.
     *
     * @var array<class-string<self>, bool>
     */
    protected static array $isPrunable;

    /**
     * Cache of mass prunable models.
     *
     * @var array<class-string<self>, bool>
     */
    protected static array $isMassPrunable;

    /**
     * Cache of resolved class attributes.
     *
     * @var array<class-string<self>, array<class-string, mixed>>
     */
    protected static array $classAttributes = [];

    /**
     * The name of the "created at" column.
     *
     * @var string|null
     */
    const CREATED_AT = 'created_at';

    /**
     * The name of the "updated at" column.
     *
     * @var string|null
     */
    const UPDATED_AT = 'updated_at';

    /**
     * Create a new Eloquent model instance.
     *
     * @param  array<string, mixed>  $attributes
     */
    public function __construct(array $attributes = [])
    {}

    /**
     * Check if the model needs to be booted and if so, do it.
     *
     * @return void
     *
     * @throws \LogicException
     */
    protected function bootIfNotBooted()
    {}

    /**
     * Perform any actions required before the model boots.
     *
     * @return void
     */
    protected static function booting()
    {}

    /**
     * Bootstrap the model and its traits.
     *
     * @return void
     */
    protected static function boot()
    {}

    /**
     * Boot all of the bootable traits on the model.
     *
     * @return void
     */
    protected static function bootTraits()
    {}

    /**
     * Initialize any initializable traits on the model.
     *
     * @return void
     */
    protected function initializeTraits()
    {}

    /**
     * Initialize the model attributes from class attributes.
     *
     * @return void
     */
    public function initializeModelAttributes()
    {}

    /**
     * Perform any actions required after the model boots.
     *
     * @return void
     */
    protected static function booted()
    {}

    /**
     * Register a closure to be executed after the model has booted.
     *
     * @param  \Closure  $callback
     * @return void
     */
    protected static function whenBooted(Closure $callback)
    {}

    /**
     * Clear the list of booted models so they will be re-booted.
     *
     * @return void
     */
    public static function clearBootedModels()
    {}

    /**
     * Disables relationship model touching for the current class during given callback scope.
     *
     * @param  callable  $callback
     * @return void
     */
    public static function withoutTouching(callable $callback)
    {}

    /**
     * Disables relationship model touching for the given model classes during given callback scope.
     *
     * @param  array  $models
     * @param  callable  $callback
     * @return void
     */
    public static function withoutTouchingOn(array $models, callable $callback)
    {}

    /**
     * Determine if the given model is ignoring touches.
     *
     * @param  string|null  $class
     * @return bool
     */
    public static function isIgnoringTouch($class = null)
    {}

    /**
     * Indicate that models should prevent lazy loading, silently discarding attributes, and accessing missing attributes.
     *
     * @param  bool  $shouldBeStrict
     * @return void
     */
    public static function shouldBeStrict(bool $shouldBeStrict = true)
    {}

    /**
     * Prevent model relationships from being lazy loaded.
     *
     * @param  bool  $value
     * @return void
     */
    public static function preventLazyLoading($value = true)
    {}

    /**
     * Determine if model relationships should be automatically eager loaded when accessed.
     *
     * @param  bool  $value
     * @return void
     */
    public static function automaticallyEagerLoadRelationships($value = true)
    {}

    /**
     * Register a callback that is responsible for handling lazy loading violations.
     *
     * @param  (callable(self, string, \Illuminate\Database\LazyLoadingViolationException): mixed)|null  $callback
     * @return void
     */
    public static function handleLazyLoadingViolationUsing(?callable $callback)
    {}

    /**
     * Prevent non-fillable attributes from being silently discarded.
     *
     * @param  bool  $value
     * @return void
     */
    public static function preventSilentlyDiscardingAttributes($value = true)
    {}

    /**
     * Register a callback that is responsible for handling discarded attribute violations.
     *
     * @param  (callable(self, array, \Illuminate\Database\Eloquent\MassAssignmentException): mixed)|null  $callback
     * @return void
     */
    public static function handleDiscardedAttributeViolationUsing(?callable $callback)
    {}

    /**
     * Prevent accessing missing attributes on retrieved models.
     *
     * @param  bool  $value
     * @return void
     */
    public static function preventAccessingMissingAttributes($value = true)
    {}

    /**
     * Register a callback that is responsible for handling missing attribute violations.
     *
     * @param  (callable(self, string, \Illuminate\Database\Eloquent\MissingAttributeException): mixed)|null  $callback
     * @return void
     */
    public static function handleMissingAttributeViolationUsing(?callable $callback)
    {}

    /**
     * Execute a callback without broadcasting any model events for all model types.
     *
     * @template TReturn
     *
     * @param  callable(): TReturn  $callback
     * @return TReturn
     */
    public static function withoutBroadcasting(callable $callback)
    {}

    /**
     * Fill the model with an array of attributes.
     *
     * @param  array<string, mixed>  $attributes
     * @return $this
     *
     * @throws \Illuminate\Database\Eloquent\MassAssignmentException
     */
    public function fill(array $attributes)
    {}

    /**
     * Fill the model with an array of attributes. Force mass assignment.
     *
     * @param  array<string, mixed>  $attributes
     * @return $this
     */
    public function forceFill(array $attributes)
    {}

    /**
     * Qualify the given column name by the model's table.
     *
     * @param  string  $column
     * @return string
     */
    public function qualifyColumn($column)
    {}

    /**
     * Qualify the given columns with the model's table.
     *
     * @param  array  $columns
     * @return array
     */
    public function qualifyColumns($columns)
    {}

    /**
     * Create a new instance of the given model.
     *
     * @param  array<string, mixed>  $attributes
     * @param  bool  $exists
     * @return static
     */
    public function newInstance($attributes = [], $exists = false)
    {}

    /**
     * Create a new model instance that is existing.
     *
     * @param  array<string, mixed>  $attributes
     * @param  \UnitEnum|string|null  $connection
     * @return static
     */
    public function newFromBuilder($attributes = [], $connection = null)
    {}

    /**
     * Begin querying the model on a given connection.
     *
     * @param  \UnitEnum|string|null  $connection
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public static function on($connection = null)
    {}

    /**
     * Begin querying the model on the write connection.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public static function onWriteConnection()
    {}

    /**
     * Get all of the models from the database.
     *
     * @param  array|string  $columns
     * @return \Illuminate\Database\Eloquent\Collection<int, static>
     */
    public static function all($columns = ['*'])
    {}

    /**
     * Begin querying a model with eager loading.
     *
     * @param  array|string  $relations
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public static function with($relations)
    {}

    /**
     * Eager load relations on the model.
     *
     * @param  array|string  $relations
     * @return $this
     */
    public function load($relations)
    {}

    /**
     * Eager load relationships on the polymorphic relation of a model.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @return $this
     */
    public function loadMorph($relation, $relations)
    {}

    /**
     * Eager load relations on the model if they are not already eager loaded.
     *
     * @param  array|string  $relations
     * @return $this
     */
    public function loadMissing($relations)
    {}

    /**
     * Eager load relation's column aggregations on the model.
     *
     * @param  array|string  $relations
     * @param  string  $column
     * @param  string|null  $function
     * @return $this
     */
    public function loadAggregate($relations, $column, $function = null)
    {}

    /**
     * Eager load relation counts on the model.
     *
     * @param  array|string  $relations
     * @return $this
     */
    public function loadCount($relations)
    {}

    /**
     * Eager load relation max column values on the model.
     *
     * @param  array|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMax($relations, $column)
    {}

    /**
     * Eager load relation min column values on the model.
     *
     * @param  array|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMin($relations, $column)
    {}

    /**
     * Eager load relation's column summations on the model.
     *
     * @param  array|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadSum($relations, $column)
    {}

    /**
     * Eager load relation average column values on the model.
     *
     * @param  array|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadAvg($relations, $column)
    {}

    /**
     * Eager load related model existence values on the model.
     *
     * @param  array|string  $relations
     * @return $this
     */
    public function loadExists($relations)
    {}

    /**
     * Eager load relationship column aggregation on the polymorphic relation of a model.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @param  string  $column
     * @param  string|null  $function
     * @return $this
     */
    public function loadMorphAggregate($relation, $relations, $column, $function = null)
    {}

    /**
     * Eager load relationship counts on the polymorphic relation of a model.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @return $this
     */
    public function loadMorphCount($relation, $relations)
    {}

    /**
     * Eager load relationship max column values on the polymorphic relation of a model.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMorphMax($relation, $relations, $column)
    {}

    /**
     * Eager load relationship min column values on the polymorphic relation of a model.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMorphMin($relation, $relations, $column)
    {}

    /**
     * Eager load relationship column summations on the polymorphic relation of a model.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMorphSum($relation, $relations, $column)
    {}

    /**
     * Eager load relationship average column values on the polymorphic relation of a model.
     *
     * @param  string  $relation
     * @param  array  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMorphAvg($relation, $relations, $column)
    {}

    /**
     * Increment a column's value by a given amount.
     *
     * @param  string  $column
     * @param  float|int  $amount
     * @param  array  $extra
     * @return int|false
     */
    protected function increment($column, $amount = 1, array $extra = [])
    {}

    /**
     * Decrement a column's value by a given amount.
     *
     * @param  string  $column
     * @param  float|int  $amount
     * @param  array  $extra
     * @return int|false
     */
    protected function decrement($column, $amount = 1, array $extra = [])
    {}

    /**
     * Run the increment or decrement method on the model.
     *
     * @param  string  $column
     * @param  float|int  $amount
     * @param  array  $extra
     * @param  string  $method
     * @return int|false
     */
    protected function incrementOrDecrement($column, $amount, $extra, $method)
    {}

    /**
     * Update the model in the database.
     *
     * @param  array<string, mixed>  $attributes
     * @param  array<string, mixed>  $options
     * @return bool
     */
    public function update(array $attributes = [], array $options = [])
    {}

    /**
     * Update the model in the database within a transaction.
     *
     * @param  array<string, mixed>  $attributes
     * @param  array<string, mixed>  $options
     * @return bool
     *
     * @throws \Throwable
     */
    public function updateOrFail(array $attributes = [], array $options = [])
    {}

    /**
     * Update the model in the database without raising any events.
     *
     * @param  array<string, mixed>  $attributes
     * @param  array<string, mixed>  $options
     * @return bool
     */
    public function updateQuietly(array $attributes = [], array $options = [])
    {}

    /**
     * Increment a column's value by a given amount without raising any events.
     *
     * @param  string  $column
     * @param  float|int  $amount
     * @param  array  $extra
     * @return int
     */
    protected function incrementQuietly($column, $amount = 1, array $extra = [])
    {}

    /**
     * Decrement a column's value by a given amount without raising any events.
     *
     * @param  string  $column
     * @param  float|int  $amount
     * @param  array  $extra
     * @return int
     */
    protected function decrementQuietly($column, $amount = 1, array $extra = [])
    {}

    /**
     * Increment each given column's value by the given amounts.
     *
     * @param  array<string, float|int>  $columns
     * @param  array<string, mixed>  $extra
     * @return int|false
     */
    protected function incrementEach(array $columns, array $extra = [])
    {}

    /**
     * Decrement each given column's value by the given amounts.
     *
     * @param  array<string, float|int>  $columns
     * @param  array<string, mixed>  $extra
     * @return int|false
     */
    protected function decrementEach(array $columns, array $extra = [])
    {}

    /**
     * Increment each given column's value by the given amounts without raising any events.
     *
     * @param  array<string, float|int>  $columns
     * @param  array<string, mixed>  $extra
     * @return int
     */
    protected function incrementEachQuietly(array $columns, array $extra = [])
    {}

    /**
     * Decrement each given column's value by the given amounts without raising any events.
     *
     * @param  array<string, float|int>  $columns
     * @param  array<string, mixed>  $extra
     * @return int
     */
    protected function decrementEachQuietly(array $columns, array $extra = [])
    {}

    /**
     * Run the incrementEach or decrementEach method on the model.
     *
     * @param  array<string, float|int>  $columns
     * @param  array<string, mixed>  $extra
     * @param  string  $method
     * @return int|false
     */
    protected function incrementOrDecrementEach(array $columns, array $extra, string $method)
    {}

    /**
     * Save the model and all of its relationships.
     *
     * @return bool
     */
    public function push()
    {}

    /**
     * Save the model and all of its relationships without raising any events to the parent model.
     *
     * @return bool
     */
    public function pushQuietly()
    {}

    /**
     * Save the model to the database without raising any events.
     *
     * @param  array  $options
     * @return bool
     */
    public function saveQuietly(array $options = [])
    {}

    /**
     * Save the model to the database.
     *
     * @param  array  $options
     * @return bool
     */
    public function save(array $options = [])
    {}

    /**
     * Save the model to the database, ignoring specific unique constraint conflicts.
     *
     * @param  array  $options
     * @param  array|string|null  $uniqueBy
     * @return bool
     */
    public function saveOrIgnore(array $options = [], array|string|null $uniqueBy = null)
    {}

    /**
     * Save the model to the database within a transaction.
     *
     * @param  array  $options
     * @return bool
     *
     * @throws \Throwable
     */
    public function saveOrFail(array $options = [])
    {}

    /**
     * Perform any actions that are necessary after the model is saved.
     *
     * @param  array  $options
     * @return void
     */
    protected function finishSave(array $options)
    {}

    /**
     * Perform a model update operation.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return bool
     */
    protected function performUpdate(Builder $query)
    {}

    /**
     * Set the keys for a select query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function setKeysForSelectQuery($query)
    {}

    /**
     * Get the primary key value for a select query.
     *
     * @return mixed
     */
    protected function getKeyForSelectQuery()
    {}

    /**
     * Set the keys for a save update query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    protected function setKeysForSaveQuery($query)
    {}

    /**
     * Get the primary key value for a save query.
     *
     * @return mixed
     */
    protected function getKeyForSaveQuery()
    {}

    /**
     * Perform a model insert operation.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return bool
     */
    protected function performInsert(Builder $query)
    {}

    /**
     * Perform a model insert operation, ignoring specific unique constraint conflicts.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @param  array|string|null  $uniqueBy
     * @return bool
     */
    protected function performInsertOrIgnore(Builder $query, array|string|null $uniqueBy)
    {}

    /**
     * Insert the given attributes and set the ID on the model.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @param  array<string, mixed>  $attributes
     * @return void
     */
    protected function insertAndSetId(Builder $query, $attributes)
    {}

    /**
     * Refresh the configured attributes after the model is saved.
     *
     * @return void
     */
    protected function refreshSavedAttributes()
    {}

    /**
     * Destroy the models for the given IDs.
     *
     * @param  \Illuminate\Support\Collection|array|int|string  $ids
     * @return int
     */
    public static function destroy($ids)
    {}

    /**
     * Delete the model from the database.
     *
     * @return bool|null
     *
     * @throws \LogicException
     */
    public function delete()
    {}

    /**
     * Delete the model from the database without raising any events.
     *
     * @return bool
     */
    public function deleteQuietly()
    {}

    /**
     * Delete the model from the database within a transaction.
     *
     * @return bool|null
     *
     * @throws \Throwable
     */
    public function deleteOrFail()
    {}

    /**
     * Force a hard delete on a soft deleted model.
     *
     * This method protects developers from running forceDelete when the trait is missing.
     *
     * @return bool|null
     */
    public function forceDelete()
    {}

    /**
     * Force a hard destroy on a soft deleted model.
     *
     * This method protects developers from running forceDestroy when the trait is missing.
     *
     * @param  \Illuminate\Support\Collection|array|int|string  $ids
     * @return int
     */
    public static function forceDestroy($ids)
    {}

    /**
     * Perform the actual delete query on this model instance.
     *
     * @return void
     */
    protected function performDeleteOnModel()
    {}

    /**
     * Begin querying the model.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public static function query()
    {}

    /**
     * Get a new query builder for the model's table.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newQuery()
    {}

    /**
     * Get a new query builder that doesn't have any global scopes or eager loading.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newModelQuery()
    {}

    /**
     * Get a new query builder with no relationships loaded.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newQueryWithoutRelationships()
    {}

    /**
     * Register the global scopes for this builder instance.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $builder
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function registerGlobalScopes($builder)
    {}

    /**
     * Get a new query builder that doesn't have any global scopes.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newQueryWithoutScopes()
    {}

    /**
     * Get a new query instance without a given scope.
     *
     * @param  \Illuminate\Database\Eloquent\Scope|string  $scope
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newQueryWithoutScope($scope)
    {}

    /**
     * Get a new query to restore one or more models by their queueable IDs.
     *
     * @param  array|int  $ids
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function newQueryForRestoration($ids)
    {}

    /**
     * Create a new Eloquent query builder for the model.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @return \Illuminate\Database\Eloquent\Builder<*>
     */
    public function newEloquentBuilder($query)
    {}

    /**
     * Resolve the custom Eloquent builder class from the model attributes.
     *
     * @return class-string<\Illuminate\Database\Eloquent\Builder>|false
     */
    protected function resolveCustomBuilderClass()
    {}

    /**
     * Get a new query builder instance for the connection.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    protected function newBaseQueryBuilder()
    {}

    /**
     * Create a new pivot model instance.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $parent
     * @param  array<string, mixed>  $attributes
     * @param  string  $table
     * @param  bool  $exists
     * @param  string|null  $using
     * @return \Illuminate\Database\Eloquent\Relations\Pivot
     */
    public function newPivot(self $parent, array $attributes, $table, $exists, $using = null)
    {}

    /**
     * Determine if the model has a given scope.
     *
     * @param  string  $scope
     * @return bool
     */
    public function hasNamedScope($scope)
    {}

    /**
     * Apply the given named scope if possible.
     *
     * @param  string  $scope
     * @param  array  $parameters
     * @return mixed
     */
    public function callNamedScope($scope, array $parameters = [])
    {}

    /**
     * Determine if the given method has a scope attribute.
     *
     * @param  string  $method
     * @return bool
     */
    protected static function isScopeMethodWithAttribute(string $method)
    {}

    /**
     * Convert the model instance to an array.
     *
     * @return array
     */
    public function toArray()
    {}

    /**
     * Convert the model instance to JSON.
     *
     * @param  int  $options
     * @return string
     *
     * @throws \Illuminate\Database\Eloquent\JsonEncodingException
     */
    public function toJson($options = 0)
    {}

    /**
     * Convert the model instance to pretty print formatted JSON.
     *
     * @param  int  $options
     * @return string
     *
     * @throws \Illuminate\Database\Eloquent\JsonEncodingException
     */
    public function toPrettyJson(int $options = 0)
    {}

    /**
     * Convert the object into something JSON serializable.
     *
     * @return mixed
     */
    public function jsonSerialize(): mixed
    {}

    /**
     * Reload a fresh model instance from the database.
     *
     * @param  array|string  $with
     * @return static|null
     */
    public function fresh($with = [])
    {}

    /**
     * Reload the current model instance with fresh attributes from the database.
     *
     * @return $this
     */
    public function refresh()
    {}

    /**
     * Reload the current model instance with fresh attributes from the database while locking it for updating.
     *
     * @return $this
     */
    public function refreshForUpdate()
    {}

    /**
     * Reload the current model instance using the given query.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return $this
     */
    protected function refreshUsingQuery(Builder $query)
    {}

    /**
     * Clone the model into a new, non-existing instance.
     *
     * @param  array|null  $except
     * @return static
     */
    public function replicate(?array $except = null)
    {}

    /**
     * Clone the model into a new, non-existing instance without raising any events.
     *
     * @param  array|null  $except
     * @return static
     */
    public function replicateQuietly(?array $except = null)
    {}

    /**
     * Determine if two models have the same ID and belong to the same table.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $model
     * @return bool
     */
    public function is($model)
    {}

    /**
     * Determine if two models are not the same.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $model
     * @return bool
     */
    public function isNot($model)
    {}

    /**
     * Get the database connection for the model.
     *
     * @return \Illuminate\Database\Connection
     */
    public function getConnection()
    {}

    /**
     * Get the current connection name for the model.
     *
     * @return string|null
     */
    public function getConnectionName()
    {}

    /**
     * Set the connection associated with the model.
     *
     * @param  \UnitEnum|string|null  $name
     * @return $this
     */
    public function setConnection($name)
    {}

    /**
     * Resolve a connection instance.
     *
     * @param  \UnitEnum|string|null  $connection
     * @return \Illuminate\Database\Connection
     */
    public static function resolveConnection($connection = null)
    {}

    /**
     * Get the connection resolver instance.
     *
     * @return \Illuminate\Database\ConnectionResolverInterface|null
     */
    public static function getConnectionResolver()
    {}

    /**
     * Set the connection resolver instance.
     *
     * @param  \Illuminate\Database\ConnectionResolverInterface  $resolver
     * @return void
     */
    public static function setConnectionResolver(Resolver $resolver)
    {}

    /**
     * Unset the connection resolver for models.
     *
     * @return void
     */
    public static function unsetConnectionResolver()
    {}

    /**
     * Get the table associated with the model.
     *
     * @return string
     */
    public function getTable()
    {}

    /**
     * Set the table associated with the model.
     *
     * @param  string  $table
     * @return $this
     */
    public function setTable($table)
    {}

    /**
     * Get the primary key for the model.
     *
     * @return string
     */
    public function getKeyName()
    {}

    /**
     * Set the primary key for the model.
     *
     * @param  string  $key
     * @return $this
     */
    public function setKeyName($key)
    {}

    /**
     * Get the table qualified key name.
     *
     * @return string
     */
    public function getQualifiedKeyName()
    {}

    /**
     * Get the auto-incrementing key type.
     *
     * @return string
     */
    public function getKeyType()
    {}

    /**
     * Set the data type for the primary key.
     *
     * @param  string  $type
     * @return $this
     */
    public function setKeyType($type)
    {}

    /**
     * Get the value indicating whether the IDs are incrementing.
     *
     * @return bool
     */
    public function getIncrementing()
    {}

    /**
     * Set whether IDs are incrementing.
     *
     * @param  bool  $value
     * @return $this
     */
    public function setIncrementing($value)
    {}

    /**
     * Get the value of the model's primary key.
     *
     * @return mixed
     */
    public function getKey()
    {}

    /**
     * Get the queueable identity for the entity.
     *
     * @return mixed
     */
    public function getQueueableId()
    {}

    /**
     * Get the queueable relationships for the entity.
     *
     * @return array
     */
    public function getQueueableRelations()
    {}

    /**
     * Get the queueable connection for the entity.
     *
     * @return string|null
     */
    public function getQueueableConnection()
    {}

    /**
     * Get the value of the model's route key.
     *
     * @return mixed
     */
    public function getRouteKey()
    {}

    /**
     * Get the route key for the model.
     *
     * @return string
     */
    public function getRouteKeyName()
    {}

    /**
     * Retrieve the model for a bound value.
     *
     * @param  mixed  $value
     * @param  string|null  $field
     * @return \Illuminate\Database\Eloquent\Model|null
     */
    public function resolveRouteBinding($value, $field = null)
    {}

    /**
     * Retrieve the model for a bound value.
     *
     * @param  mixed  $value
     * @param  string|null  $field
     * @return \Illuminate\Database\Eloquent\Model|null
     */
    public function resolveSoftDeletableRouteBinding($value, $field = null)
    {}

    /**
     * Retrieve the child model for a bound value.
     *
     * @param  string  $childType
     * @param  mixed  $value
     * @param  string|null  $field
     * @return \Illuminate\Database\Eloquent\Model|null
     */
    public function resolveChildRouteBinding($childType, $value, $field)
    {}

    /**
     * Retrieve the child model for a bound value.
     *
     * @param  string  $childType
     * @param  mixed  $value
     * @param  string|null  $field
     * @return \Illuminate\Database\Eloquent\Model|null
     */
    public function resolveSoftDeletableChildRouteBinding($childType, $value, $field)
    {}

    /**
     * Retrieve the child model query for a bound value.
     *
     * @param  string  $childType
     * @param  mixed  $value
     * @param  string|null  $field
     * @return \Illuminate\Database\Eloquent\Relations\Relation<\Illuminate\Database\Eloquent\Model, $this, *>
     */
    protected function resolveChildRouteBindingQuery($childType, $value, $field)
    {}

    /**
     * Retrieve the child route model binding relationship name for the given child type.
     *
     * @param  string  $childType
     * @return string
     */
    protected function childRouteBindingRelationshipName($childType)
    {}

    /**
     * Retrieve the model for a bound value.
     *
     * @param  \Illuminate\Database\Eloquent\Model|\Illuminate\Contracts\Database\Eloquent\Builder|\Illuminate\Database\Eloquent\Relations\Relation  $query
     * @param  mixed  $value
     * @param  string|null  $field
     * @return \Illuminate\Contracts\Database\Eloquent\Builder
     */
    public function resolveRouteBindingQuery($query, $value, $field = null)
    {}

    /**
     * Get the default foreign key name for the model.
     *
     * @return string
     */
    public function getForeignKey()
    {}

    /**
     * Get the number of models to return per page.
     *
     * @return int
     */
    public function getPerPage()
    {}

    /**
     * Set the number of models to return per page.
     *
     * @param  int  $perPage
     * @return $this
     */
    public function setPerPage($perPage)
    {}

    /**
     * Determine if the model is soft deletable.
     */
    public static function isSoftDeletable(): bool
    {}

    /**
     * Determine if the model is prunable.
     */
    protected function isPrunable(): bool
    {}

    /**
     * Determine if the model is mass prunable.
     */
    protected function isMassPrunable(): bool
    {}

    /**
     * Determine if lazy loading is disabled.
     *
     * @return bool
     */
    public static function preventsLazyLoading()
    {}

    /**
     * Determine if relationships are being automatically eager loaded when accessed.
     *
     * @return bool
     */
    public static function isAutomaticallyEagerLoadingRelationships()
    {}

    /**
     * Determine if discarding guarded attribute fills is disabled.
     *
     * @return bool
     */
    public static function preventsSilentlyDiscardingAttributes()
    {}

    /**
     * Determine if accessing missing attributes is disabled.
     *
     * @return bool
     */
    public static function preventsAccessingMissingAttributes()
    {}

    /**
     * Get the broadcast channel route definition that is associated with the given entity.
     *
     * @return string
     */
    public function broadcastChannelRoute()
    {}

    /**
     * Get the broadcast channel name that is associated with the given entity.
     *
     * @return string
     */
    public function broadcastChannel()
    {}

    /**
     * Resolve a class attribute value from the model.
     *
     * @template TAttribute of object
     *
     * @param  class-string<TAttribute>  $attributeClass
     * @param  string|null  $property
     * @param  string|null  $class
     * @return mixed
     */
    protected static function resolveClassAttribute(string $attributeClass, ?string $property = null, ?string $class = null)
    {}

    /**
     * Dynamically retrieve attributes on the model.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}

    /**
     * Dynamically set attributes on the model.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return void
     */
    public function __set($key, $value)
    {}

    /**
     * Determine if the given attribute exists.
     *
     * @param  mixed  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value for a given offset.
     *
     * @param  mixed  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value for a given offset.
     *
     * @param  mixed  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the value for a given offset.
     *
     * @param  mixed  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Determine if an attribute or relation exists on the model.
     *
     * @param  string  $key
     * @return bool
     */
    public function __isset($key)
    {}

    /**
     * Unset an attribute on the model.
     *
     * @param  string  $key
     * @return void
     */
    public function __unset($key)
    {}

    /**
     * Handle dynamic method calls into the model.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}

    /**
     * Handle dynamic static method calls into the model.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public static function __callStatic($method, $parameters)
    {}

    /**
     * Convert the model to its string representation.
     *
     * @return string
     */
    public function __toString()
    {}

    /**
     * Indicate that the object's string representation should be escaped when __toString is invoked.
     *
     * @param  bool  $escape
     * @return $this
     */
    public function escapeWhenCastingToString($escape = true)
    {}

    /**
     * Prepare the object for serialization.
     *
     * @return array
     */
    public function __sleep()
    {}

    /**
     * When a model is being unserialized, check if it needs to be booted.
     *
     * @return void
     */
    public function __wakeup()
    {}
}
