<?php

namespace Illuminate\Database\Eloquent\Factories;

use Closure;
use Faker\Generator;
use Illuminate\Container\Container;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Factories\Attributes\UseModel;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\Macroable;
use ReflectionClass;
use Throwable;
use UnitEnum;

use function Illuminate\Support\enum_value;

/**
 * @template TModel of \Illuminate\Database\Eloquent\Model
 *
 * @method $this trashed()
 */
abstract class Factory
{
    use Conditionable, ForwardsCalls, Macroable {
        __call as macroCall;
    }

    /**
     * The name of the factory's corresponding model.
     *
     * @var class-string<TModel>
     */
    protected $model;

    /**
     * The number of models that should be generated.
     *
     * @var int|null
     */
    protected $count;

    /**
     * The state transformations that will be applied to the model.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $states;

    /**
     * The parent relationships that will be applied to the model.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $has;

    /**
     * The child relationships that will be applied to the model.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $for;

    /**
     * The model instances to always use when creating relationships.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $recycle;

    /**
     * The "after making" callbacks that will be applied to the model.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $afterMaking;

    /**
     * The "after creating" callbacks that will be applied to the model.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $afterCreating;

    /**
     * Whether relationships should not be automatically created.
     *
     * @var bool
     */
    protected $expandRelationships = true;

    /**
     * The relationships that should not be automatically created.
     *
     * @var array
     */
    protected $excludeRelationships = [];

    /**
     * The name of the database connection that will be used to create the models.
     *
     * @var \UnitEnum|string|null
     */
    protected $connection;

    /**
     * The current Faker instance.
     *
     * @var \Faker\Generator
     */
    protected $faker;

    /**
     * The default namespace where factories reside.
     *
     * @var string
     */
    public static $namespace = 'Database\\Factories\\';

    /**
     * @deprecated use $modelNameResolvers
     *
     * @var callable(self): class-string<TModel>
     */
    protected static $modelNameResolver;

    /**
     * The default model name resolvers.
     *
     * @var array<class-string, callable(self): class-string<TModel>>
     */
    protected static $modelNameResolvers = [];

    /**
     * The factory name resolver.
     *
     * @var callable
     */
    protected static $factoryNameResolver;

    /**
     * Whether to expand relationships by default.
     *
     * @var bool
     */
    protected static $expandRelationshipsByDefault = true;

    /**
     * The cached model class names resolved from attributes.
     *
     * @var array<class-string, class-string<TModel>|false>
     */
    protected static $cachedModelAttributes = [];

    /**
     * Create a new factory instance.
     *
     * @param  int|null  $count
     * @param  \Illuminate\Support\Collection|null  $states
     * @param  \Illuminate\Support\Collection|null  $has
     * @param  \Illuminate\Support\Collection|null  $for
     * @param  \Illuminate\Support\Collection|null  $afterMaking
     * @param  \Illuminate\Support\Collection|null  $afterCreating
     * @param  \UnitEnum|string|null  $connection
     * @param  \Illuminate\Support\Collection|null  $recycle
     * @param  bool|null  $expandRelationships
     * @param  array  $excludeRelationships
     */
    public function __construct(
        $count = null,
        ?Collection $states = null,
        ?Collection $has = null,
        ?Collection $for = null,
        ?Collection $afterMaking = null,
        ?Collection $afterCreating = null,
        $connection = null,
        ?Collection $recycle = null,
        ?bool $expandRelationships = null,
        array $excludeRelationships = [],
    ) {}

    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    abstract public function definition();

    /**
     * Get a new factory instance for the given attributes.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @return static
     */
    public static function new($attributes = [])
    {}

    /**
     * Get a new factory instance for the given number of models.
     *
     * @param  int  $count
     * @return static
     */
    public static function times(int $count)
    {}

    /**
     * Configure the factory.
     *
     * @return static
     */
    public function configure()
    {}

    /**
     * Get the raw attributes generated by the factory.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return array<int|string, mixed>
     */
    public function raw($attributes = [], ?Model $parent = null)
    {}

    /**
     * Create a single model and persist it to the database.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @return TModel
     */
    public function createOne($attributes = [])
    {}

    /**
     * Create a single model and persist it to the database without dispatching any model events.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @return TModel
     */
    public function createOneQuietly($attributes = [])
    {}

    /**
     * Create a collection of models and persist them to the database.
     *
     * @param  int|null|iterable<int, array<string, mixed>>  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>
     */
    public function createMany(int|iterable|null $records = null)
    {}

    /**
     * Create a collection of models and persist them to the database without dispatching any model events.
     *
     * @param  int|null|iterable<int, array<string, mixed>>  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>
     */
    public function createManyQuietly(int|iterable|null $records = null)
    {}

    /**
     * Create a collection of models and persist them to the database.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>|TModel
     */
    public function create($attributes = [], ?Model $parent = null)
    {}

    /**
     * Create a collection of models and persist them to the database without dispatching any model events.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>|TModel
     */
    public function createQuietly($attributes = [], ?Model $parent = null)
    {}

    /**
     * Create a callback that persists a model in the database when invoked.
     *
     * @param  array<string, mixed>  $attributes
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return \Closure(): (\Illuminate\Database\Eloquent\Collection<int, TModel>|TModel)
     */
    public function lazy(array $attributes = [], ?Model $parent = null)
    {}

    /**
     * Set the connection name on the results and store them.
     *
     * @param  \Illuminate\Support\Collection<int, TModel>  $results
     * @return void
     */
    protected function store(Collection $results)
    {}

    /**
     * Create the children for the given model.
     *
     * @param  TModel  $model
     * @return void
     */
    protected function createChildren(Model $model)
    {}

    /**
     * Make a single instance of the model.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @return TModel
     */
    public function makeOne($attributes = [])
    {}

    /**
     * Create a collection of models.
     *
     * @param  (callable(array<string, mixed>): array<string, mixed>)|array<string, mixed>  $attributes
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>|TModel
     */
    public function make($attributes = [], ?Model $parent = null)
    {}

    /**
     * Create a collection of models.
     *
     * @param  iterable<int, array<string, mixed>>|int|null  $records
     * @return \Illuminate\Database\Eloquent\Collection<int, TModel>
     */
    public function makeMany(iterable|int|null $records = null)
    {}

    /**
     * Insert the model records in bulk. No model events are emitted.
     *
     * @param  array<string, mixed>  $attributes
     * @param  Model|null  $parent
     * @return void
     */
    public function insert(array $attributes = [], ?Model $parent = null): void
    {}

    /**
     * Make an instance of the model with the given attributes.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return TModel
     */
    protected function makeInstance(?Model $parent)
    {}

    /**
     * Get a raw attributes array for the model.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return mixed
     */
    protected function getExpandedAttributes(?Model $parent)
    {}

    /**
     * Get the raw attributes for the model as an array.
     *
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return array
     */
    protected function getRawAttributes(?Model $parent)
    {}

    /**
     * Create the parent relationship resolvers (as deferred Closures).
     *
     * @return array
     */
    protected function parentResolvers()
    {}

    /**
     * Expand all attributes to their underlying values.
     *
     * @param  array  $definition
     * @return array
     */
    protected function expandAttributes(array $definition)
    {}

    /**
     * Add a new state transformation to the model definition.
     *
     * @param  (callable(array<string, mixed>, Model|null): array<string, mixed>)|array<string, mixed>  $state
     * @return static
     */
    public function state($state)
    {}

    /**
     * Prepend a new state transformation to the model definition.
     *
     * @param  (callable(array<string, mixed>, Model|null): array<string, mixed>)|array<string, mixed>  $state
     * @return static
     */
    public function prependState($state)
    {}

    /**
     * Set a single model attribute.
     *
     * @param  string|int  $key
     * @param  mixed  $value
     * @return static
     */
    public function set($key, $value)
    {}

    /**
     * Add a new sequenced state transformation to the model definition.
     *
     * @param  mixed  ...$sequence
     * @return static
     */
    public function sequence(...$sequence)
    {}

    /**
     * Add a new sequenced state transformation to the model definition and update the pending creation count to the size of the sequence.
     *
     * @param  array  ...$sequence
     * @return static
     */
    public function forEachSequence(...$sequence)
    {}

    /**
     * Add a new cross joined sequenced state transformation to the model definition.
     *
     * @param  array  ...$sequence
     * @return static
     */
    public function crossJoinSequence(...$sequence)
    {}

    /**
     * Define a child relationship for the model.
     *
     * @param  \Illuminate\Database\Eloquent\Factories\Factory  $factory
     * @param  string|null  $relationship
     * @return static
     */
    public function has(self $factory, $relationship = null)
    {}

    /**
     * Attempt to guess the relationship name for a "has" relationship.
     *
     * @param  string  $related
     * @return string
     */
    protected function guessRelationship(string $related)
    {}

    /**
     * Define an attached relationship for the model.
     *
     * @param  \Illuminate\Database\Eloquent\Factories\Factory|\Illuminate\Support\Collection|\Illuminate\Database\Eloquent\Model|array  $factory
     * @param  (callable(): array<string, mixed>)|array<string, mixed>  $pivot
     * @param  string|null  $relationship
     * @return static
     */
    public function hasAttached($factory, $pivot = [], $relationship = null)
    {}

    /**
     * Define a parent relationship for the model.
     *
     * @param  \Illuminate\Database\Eloquent\Factories\Factory|\Illuminate\Database\Eloquent\Model  $factory
     * @param  string|null  $relationship
     * @return static
     */
    public function for($factory, $relationship = null)
    {}

    /**
     * Provide model instances to use instead of any nested factory calls when creating relationships.
     *
     * @param  \Illuminate\Database\Eloquent\Model|\Illuminate\Support\Collection|array  $model
     * @return static
     */
    public function recycle($model)
    { func_get_args(); }

    /**
     * Retrieve a random model of a given type from previously provided models to recycle.
     *
     * @template TClass of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TClass>  $modelClassName
     * @return TClass|null
     */
    public function getRandomRecycledModel($modelClassName)
    {}

    /**
     * Add a new "after making" callback to the model definition.
     *
     * @param  \Closure(TModel): mixed  $callback
     * @return static
     */
    public function afterMaking(Closure $callback)
    {}

    /**
     * Add a new "after creating" callback to the model definition.
     *
     * @param  \Closure(TModel, \Illuminate\Database\Eloquent\Model|null): mixed  $callback
     * @return static
     */
    public function afterCreating(Closure $callback)
    {}

    /**
     * Remove the "after making" callbacks from the factory.
     *
     * @return static
     */
    public function withoutAfterMaking()
    {}

    /**
     * Remove the "after creating" callbacks from the factory.
     *
     * @return static
     */
    public function withoutAfterCreating()
    {}

    /**
     * Call the "after making" callbacks for the given model instances.
     *
     * @param  \Illuminate\Support\Collection  $instances
     * @return void
     */
    protected function callAfterMaking(Collection $instances)
    {}

    /**
     * Call the "after creating" callbacks for the given model instances.
     *
     * @param  \Illuminate\Support\Collection  $instances
     * @param  \Illuminate\Database\Eloquent\Model|null  $parent
     * @return void
     */
    protected function callAfterCreating(Collection $instances, ?Model $parent = null)
    {}

    /**
     * Specify how many models should be generated.
     *
     * @param  int|null  $count
     * @return static
     */
    public function count(?int $count)
    {}

    /**
     * Indicate that related parent models should not be created.
     *
     * @param  array<string|class-string<Model>>  $parents
     * @return static
     */
    public function withoutParents($parents = [])
    {}

    /**
     * Get the name of the database connection that is used to generate models.
     *
     * @return string
     */
    public function getConnectionName()
    {}

    /**
     * Specify the database connection that should be used to generate models.
     *
     * @param  \UnitEnum|string|null  $connection
     * @return static
     */
    public function connection(UnitEnum|string|null $connection)
    {}

    /**
     * Create a new instance of the factory builder with the given mutated properties.
     *
     * @param  array  $arguments
     * @return static
     */
    protected function newInstance(array $arguments = [])
    {}

    /**
     * Get a new model instance.
     *
     * @param  array<string, mixed>  $attributes
     * @return TModel
     */
    public function newModel(array $attributes = [])
    {}

    /**
     * Get the name of the model that is generated by the factory.
     *
     * @return class-string<TModel>
     */
    public function modelName()
    {}

    /**
     * Specify the callback that should be invoked to guess model names based on factory names.
     *
     * @param  callable(self): class-string<TModel>  $callback
     * @return void
     */
    public static function guessModelNamesUsing(callable $callback)
    {}

    /**
     * Specify the default namespace that contains the application's model factories.
     *
     * @param  string  $namespace
     * @return void
     */
    public static function useNamespace(string $namespace)
    {}

    /**
     * Get a new factory instance for the given model name.
     *
     * @template TClass of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TClass>  $modelName
     * @return \Illuminate\Database\Eloquent\Factories\Factory<TClass>
     */
    public static function factoryForModel(string $modelName)
    {}

    /**
     * Specify the callback that should be invoked to guess factory names based on dynamic relationship names.
     *
     * @param  callable(class-string<\Illuminate\Database\Eloquent\Model>): class-string<\Illuminate\Database\Eloquent\Factories\Factory>  $callback
     * @return void
     */
    public static function guessFactoryNamesUsing(callable $callback)
    {}

    /**
     * Specify that relationships should create parent relationships by default.
     *
     * @return void
     */
    public static function expandRelationshipsByDefault()
    {}

    /**
     * Specify that relationships should not create parent relationships by default.
     *
     * @return void
     */
    public static function dontExpandRelationshipsByDefault()
    {}

    /**
     * Get a new Faker instance.
     *
     * @return \Faker\Generator|null
     */
    protected function withFaker()
    {}

    /**
     * Get the factory name for the given model name.
     *
     * @template TClass of \Illuminate\Database\Eloquent\Model
     *
     * @param  class-string<TClass>  $modelName
     * @return class-string<\Illuminate\Database\Eloquent\Factories\Factory<TClass>>
     */
    public static function resolveFactoryName(string $modelName)
    {}

    /**
     * Get the application namespace for the application.
     *
     * @return string
     */
    protected static function appNamespace()
    {}

    /**
     * Flush the factory's global state.
     *
     * @return void
     */
    public static function flushState()
    {}

    /**
     * Proxy dynamic factory methods onto their proper methods.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
