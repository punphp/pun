<?php

namespace Illuminate\Database\Eloquent;

use Illuminate\Contracts\Queue\QueueableCollection;
use Illuminate\Contracts\Queue\QueueableEntity;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Eloquent\Relations\Concerns\InteractsWithDictionary;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection as BaseCollection;
use LogicException;

/**
 * @template TKey of array-key
 * @template TModel of \Illuminate\Database\Eloquent\Model
 *
 * @extends \Illuminate\Support\Collection<TKey, TModel>
 */
class Collection extends BaseCollection implements QueueableCollection
{
    use InteractsWithDictionary;

    /**
     * Find a model in the collection by key.
     *
     * @template TFindDefault
     *
     * @param  mixed  $key
     * @param  TFindDefault  $default
     * @return ($key is (\Illuminate\Contracts\Support\Arrayable<array-key, mixed>|array<mixed>) ? static : TModel|TFindDefault)
     */
    public function find($key, $default = null)
    {}

    /**
     * Find a model in the collection by key or throw an exception.
     *
     * @param  mixed  $key
     * @return TModel
     *
     * @throws \Illuminate\Database\Eloquent\ModelNotFoundException
     */
    public function findOrFail($key)
    {}

    /**
     * Load a set of relationships onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @return $this
     */
    public function load($relations)
    { func_get_args(); }

    /**
     * Load a set of aggregations over relationship's column onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @param  string  $column
     * @param  string|null  $function
     * @return $this
     */
    public function loadAggregate($relations, $column, $function = null)
    {}

    /**
     * Load a set of relationship counts onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @return $this
     */
    public function loadCount($relations)
    {}

    /**
     * Load a set of relationship's max column values onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMax($relations, $column)
    {}

    /**
     * Load a set of relationship's min column values onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadMin($relations, $column)
    {}

    /**
     * Load a set of relationship's column summations onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadSum($relations, $column)
    {}

    /**
     * Load a set of relationship's average column values onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @param  string  $column
     * @return $this
     */
    public function loadAvg($relations, $column)
    {}

    /**
     * Load a set of related existences onto the collection.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @return $this
     */
    public function loadExists($relations)
    {}

    /**
     * Load a set of relationships onto the collection if they are not already eager loaded.
     *
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>|string  $relations
     * @return $this
     */
    public function loadMissing($relations)
    { func_get_args(); }

    /**
     * Load a relationship path for models of the given type if it is not already eager loaded.
     *
     * @param  array<int, array<string, class-string>>  $tuples
     * @return void
     */
    public function loadMissingRelationshipChain(array $tuples)
    {}

    /**
     * Load a relationship path if it is not already eager loaded.
     *
     * @param  \Illuminate\Database\Eloquent\Collection<int, TModel>  $models
     * @param  array  $path
     * @return void
     */
    protected function loadMissingRelation(self $models, array $path)
    {}

    /**
     * Load a set of relationships onto the mixed relationship collection.
     *
     * @param  string  $relation
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>  $relations
     * @return $this
     */
    public function loadMorph($relation, $relations)
    {}

    /**
     * Load a set of relationship counts onto the mixed relationship collection.
     *
     * @param  string  $relation
     * @param  array<array-key, array|(callable(\Illuminate\Database\Eloquent\Relations\Relation<*, *, *>): mixed)|string>  $relations
     * @return $this
     */
    public function loadMorphCount($relation, $relations)
    {}

    /**
     * Determine if a key exists in the collection.
     *
     * @param  (callable(TModel, TKey): bool)|TModel|string|int  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function contains($key, $operator = null, $value = null)
    { func_get_args(); }

    /**
     * Determine if a key does not exist in the collection.
     *
     * @param  (callable(TModel, TKey): bool)|TModel|string|int  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function doesntContain($key, $operator = null, $value = null)
    { func_get_args(); }

    /**
     * Get the array of primary keys.
     *
     * @return array<int, array-key>
     */
    public function modelKeys()
    {}

    /**
     * Merge the collection with the given items.
     *
     * @param  iterable<array-key, TModel>  $items
     * @return static
     */
    public function merge($items)
    {}

    /**
     * Run a map over each of the items.
     *
     * @template TMapValue
     *
     * @param  callable(TModel, TKey): TMapValue  $callback
     * @return \Illuminate\Support\Collection<TKey, TMapValue>|static<TKey, TMapValue>
     */
    public function map(callable $callback)
    {}

    /**
     * Run an associative map over each of the items.
     *
     * The callback should return an associative array with a single key / value pair.
     *
     * @template TMapWithKeysKey of array-key
     * @template TMapWithKeysValue
     *
     * @param  callable(TModel, TKey): array<TMapWithKeysKey, TMapWithKeysValue>  $callback
     * @return \Illuminate\Support\Collection<TMapWithKeysKey, TMapWithKeysValue>|static<TMapWithKeysKey, TMapWithKeysValue>
     */
    public function mapWithKeys(callable $callback)
    {}

    /**
     * Reload a fresh model instance from the database for all the entities.
     *
     * @param  array<array-key, string>|string  $with
     * @return static
     */
    public function fresh($with = [])
    { func_get_args(); }

    /**
     * Diff the collection with the given items.
     *
     * @param  iterable<array-key, TModel>  $items
     * @return static
     */
    public function diff($items)
    {}

    /**
     * Intersect the collection with the given items.
     *
     * @param  iterable<array-key, TModel>  $items
     * @return static
     */
    public function intersect($items)
    {}

    /**
     * Return only unique items from the collection.
     *
     * @param  (callable(TModel, TKey): mixed)|string|null  $key
     * @param  bool  $strict
     * @return static
     */
    public function unique($key = null, $strict = false)
    {}

    /**
     * Returns only the models from the collection with the specified keys.
     *
     * @param  array<array-key, mixed>|null  $keys
     * @return static
     */
    public function only($keys)
    {}

    /**
     * Returns all models in the collection except the models with specified keys.
     *
     * @param  array<array-key, mixed>|null  $keys
     * @return static
     */
    public function except($keys)
    {}

    /**
     * Make the given, typically visible, attributes hidden across the entire collection.
     *
     * @param  array<array-key, string>|string  $attributes
     * @return $this
     */
    public function makeHidden($attributes)
    {}

    /**
     * Merge the given, typically visible, attributes hidden across the entire collection.
     *
     * @param  array<array-key, string>|string  $attributes
     * @return $this
     */
    public function mergeHidden($attributes)
    {}

    /**
     * Set the hidden attributes across the entire collection.
     *
     * @param  array<int, string>  $hidden
     * @return $this
     */
    public function setHidden($hidden)
    {}

    /**
     * Make the given, typically hidden, attributes visible across the entire collection.
     *
     * @param  array<array-key, string>|string  $attributes
     * @return $this
     */
    public function makeVisible($attributes)
    {}

    /**
     * Merge the given, typically hidden, attributes visible across the entire collection.
     *
     * @param  array<array-key, string>|string  $attributes
     * @return $this
     */
    public function mergeVisible($attributes)
    {}

    /**
     * Set the visible attributes across the entire collection.
     *
     * @param  array<int, string>  $visible
     * @return $this
     */
    public function setVisible($visible)
    {}

    /**
     * Append an attribute across the entire collection.
     *
     * @param  array<array-key, string>|string  $attributes
     * @return $this
     */
    public function append($attributes)
    {}

    /**
     * Sets the appends on every element of the collection, overwriting the existing appends for each.
     *
     * @param  array<array-key, mixed>  $appends
     * @return $this
     */
    public function setAppends(array $appends)
    {}

    /**
     * Remove appended properties from every element in the collection.
     *
     * @return $this
     */
    public function withoutAppends()
    {}

    /**
     * Get a dictionary keyed by primary keys.
     *
     * @param  iterable<array-key, TModel>|null  $items
     * @return array<array-key, TModel>
     */
    public function getDictionary($items = null)
    {}

    /**
     * The following methods are intercepted to always return base collections.
     */

    /**
     * {@inheritDoc}
     *
     * @return \Illuminate\Support\Collection<array-key, int>
     */
    #[\Override]
    public function countBy($countBy = null)
    {}

    /**
     * {@inheritDoc}
     *
     * @return \Illuminate\Support\Collection<int, mixed>
     */
    #[\Override]
    public function collapse()
    {}

    /**
     * {@inheritDoc}
     *
     * @return \Illuminate\Support\Collection<int, mixed>
     */
    #[\Override]
    public function flatten($depth = INF)
    {}

    /**
     * {@inheritDoc}
     *
     * @return \Illuminate\Support\Collection<TModel, TKey>
     */
    #[\Override]
    public function flip()
    {}

    /**
     * {@inheritDoc}
     *
     * @return \Illuminate\Support\Collection<int, TKey>
     */
    #[\Override]
    public function keys()
    {}

    /**
     * {@inheritDoc}
     *
     * @template TPadValue
     *
     * @return \Illuminate\Support\Collection<int, TModel|TPadValue>
     */
    #[\Override]
    public function pad($size, $value)
    {}

    /**
     * {@inheritDoc}
     *
     * @return \Illuminate\Support\Collection<int<0, 1>, static<TKey, TModel>>
     */
    #[\Override]
    public function partition($key, $operator = null, $value = null)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     *
     * @return \Illuminate\Support\Collection<array-key, mixed>
     */
    #[\Override]
    public function pluck($value, $key = null)
    {}

    /**
     * {@inheritDoc}
     *
     * @template TZipValue
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Support\Collection<int, TModel|TZipValue>>
     */
    #[\Override]
    public function zip($items)
    { func_get_args(); }

    /**
     * Retrieve duplicate items from the collection.
     *
     * @param  (callable(TModel): mixed)|string|null  $callback
     * @param  bool  $strict
     * @return \Illuminate\Support\Collection<array-key, mixed>|static
     */
    #[\Override]
    public function duplicates($callback = null, $strict = false)
    {}

    /**
     * Retrieve duplicate items from the collection using strict comparison.
     *
     * @param  (callable(TModel): mixed)|string|null  $callback
     * @return \Illuminate\Support\Collection<array-key, mixed>|static
     */
    #[\Override]
    public function duplicatesStrict($callback = null)
    {}

    /**
     * Get the comparison function to detect duplicates.
     *
     * @return callable(TModel, TModel): bool
     */
    protected function duplicateComparator($strict)
    {}

    /**
     * Enable relationship autoloading for all models in this collection.
     *
     * @return $this
     */
    public function withRelationshipAutoloading()
    {}

    /**
     * Get the type of the entities being queued.
     *
     * @return string|null
     *
     * @throws \LogicException
     */
    public function getQueueableClass()
    {}

    /**
     * Get the queueable class name for the given model.
     *
     * @param  \Illuminate\Database\Eloquent\Model  $model
     * @return string
     */
    protected function getQueueableModelClass($model)
    {}

    /**
     * Get the identifiers for all of the entities.
     *
     * @return array<int, mixed>
     */
    public function getQueueableIds()
    {}

    /**
     * Get the relationships of the entities being queued.
     *
     * @return array<int, string>
     */
    public function getQueueableRelations()
    {}

    /**
     * Get the connection of the entities being queued.
     *
     * @return string|null
     *
     * @throws \LogicException
     */
    public function getQueueableConnection()
    {}

    /**
     * Get the Eloquent query builder from the collection.
     *
     * @return \Illuminate\Database\Eloquent\Builder<TModel>
     *
     * @throws \LogicException
     */
    public function toQuery()
    {}
}
