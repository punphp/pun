<?php

namespace Illuminate\Support;

use ArrayAccess;
use ArrayIterator;
use Illuminate\Contracts\Support\CanBeEscapedWhenCastToString;
use Illuminate\Support\Traits\EnumeratesValues;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\TransformsToResourceCollection;
use InvalidArgumentException;
use SortDirection;
use stdClass;
use Traversable;

/**
 * @template TKey of array-key
 *
 * @template-covariant TValue
 *
 * @implements \ArrayAccess<TKey, TValue>
 * @implements \Illuminate\Support\Enumerable<TKey, TValue>
 */
class Collection implements ArrayAccess, CanBeEscapedWhenCastToString, Enumerable
{
    /**
     * @use \Illuminate\Support\Traits\EnumeratesValues<TKey, TValue>
     */
    use EnumeratesValues, Macroable, TransformsToResourceCollection;

    /**
     * The items contained in the collection.
     *
     * @var array<TKey, TValue>
     */
    protected $items = [];

    /**
     * Create a new collection.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>|null  $items
     */
    public function __construct($items = [])
    {}

    /**
     * Create a new instance of the collection.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>|null  $items
     * @return static
     */
    protected function newInstance($items = [])
    {}

    /**
     * Create a collection with the given range.
     *
     * @param  int  $from
     * @param  int  $to
     * @param  int  $step
     * @return static<int, int>
     */
    public static function range($from, $to, $step = 1, ...$args)
    {}

    /**
     * Get all of the items in the collection.
     *
     * @return array<TKey, TValue>
     */
    public function all()
    {}

    /**
     * Get a lazy collection for the items in this collection.
     *
     * @return \Illuminate\Support\LazyCollection<TKey, TValue>
     */
    public function lazy()
    {}

    /**
     * Get the median of a given key.
     *
     * @param  string|array<array-key, string>|null  $key
     * @return float|int|null
     */
    public function median($key = null)
    {}

    /**
     * Get the mode of a given key.
     *
     * @param  string|array<array-key, string>|null  $key
     * @return array<int, float|int>|null
     */
    public function mode($key = null)
    {}

    /**
     * Collapse the collection of items into a single array.
     *
     * @return static<int, mixed>
     */
    public function collapse()
    {}

    /**
     * Collapse the collection of items into a single array while preserving its keys.
     *
     * @return static<mixed, mixed>
     */
    public function collapseWithKeys()
    {}

    /**
     * Determine if an item exists in the collection.
     *
     * @param  (callable(TValue, TKey): bool)|TValue|string  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function contains($key, $operator = null, $value = null)
    {}

    /**
     * Determine if an item exists, using strict comparison.
     *
     * @param  (callable(TValue): bool)|TValue|array-key  $key
     * @param  TValue|null  $value
     * @return bool
     */
    public function containsStrict($key, $value = null)
    {}

    /**
     * Determine if an item is not contained in the collection.
     *
     * @param  mixed  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function doesntContain($key, $operator = null, $value = null)
    {}

    /**
     * Determine if an item is not contained in the enumerable, using strict comparison.
     *
     * @param  mixed  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function doesntContainStrict($key, $operator = null, $value = null)
    {}

    /**
     * Cross join with the given lists, returning all possible permutations.
     *
     * @template TCrossJoinKey
     * @template TCrossJoinValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TCrossJoinKey, TCrossJoinValue>|iterable<TCrossJoinKey, TCrossJoinValue>  ...$lists
     * @return static<int, array<int, TValue|TCrossJoinValue>>
     */
    public function crossJoin(...$lists)
    {}

    /**
     * Get the items in the collection that are not present in the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TValue>|iterable<array-key, TValue>  $items
     * @return static
     */
    public function diff($items)
    {}

    /**
     * Get the items in the collection that are not present in the given items, using the callback.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TValue>|iterable<array-key, TValue>  $items
     * @param  callable(TValue, TValue): int  $callback
     * @return static
     */
    public function diffUsing($items, callable $callback)
    {}

    /**
     * Get the items in the collection whose keys and values are not present in the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @return static
     */
    public function diffAssoc($items)
    {}

    /**
     * Get the items in the collection whose keys and values are not present in the given items, using the callback.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @param  callable(TKey, TKey): int  $callback
     * @return static
     */
    public function diffAssocUsing($items, callable $callback)
    {}

    /**
     * Get the items in the collection whose keys are not present in the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, mixed>|iterable<TKey, mixed>  $items
     * @return static
     */
    public function diffKeys($items)
    {}

    /**
     * Get the items in the collection whose keys are not present in the given items, using the callback.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, mixed>|iterable<TKey, mixed>  $items
     * @param  callable(TKey, TKey): int  $callback
     * @return static
     */
    public function diffKeysUsing($items, callable $callback)
    {}

    /**
     * Retrieve duplicate items from the collection.
     *
     * @template TMapValue
     *
     * @param  (callable(TValue): TMapValue)|string|null  $callback
     * @param  bool  $strict
     * @return static
     */
    public function duplicates($callback = null, $strict = false)
    {}

    /**
     * Retrieve duplicate items from the collection using strict comparison.
     *
     * @template TMapValue
     *
     * @param  (callable(TValue): TMapValue)|string|null  $callback
     * @return static
     */
    public function duplicatesStrict($callback = null)
    {}

    /**
     * Get the comparison function to detect duplicates.
     *
     * @param  bool  $strict
     * @return callable(TValue, TValue): bool
     */
    protected function duplicateComparator($strict)
    {}

    /**
     * Get all items except for those with the specified keys.
     *
     * @param  \Illuminate\Support\Enumerable<array-key, TKey>|array<array-key, TKey>|string  $keys
     * @return static
     */
    public function except($keys)
    {}

    /**
     * Run a filter over each of the items.
     *
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @return static
     */
    public function filter(?callable $callback = null)
    {}

    /**
     * Get the first item from the collection passing the given truth test.
     *
     * @template TFirstDefault
     *
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @param  TFirstDefault|(\Closure(): TFirstDefault)  $default
     * @return TValue|TFirstDefault
     */
    public function first(?callable $callback = null, $default = null)
    {}

    /**
     * Get a flattened array of the items in the collection.
     *
     * @param  int  $depth
     * @return static<int, mixed>
     */
    public function flatten($depth = INF)
    {}

    /**
     * Flip the items in the collection.
     *
     * @return static<TValue, TKey>
     */
    public function flip()
    {}

    /**
     * Remove an item from the collection by key.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TValue>|iterable<array-key, TKey>|TKey  $keys
     * @return $this
     */
    public function forget($keys)
    {}

    /**
     * Get an item from the collection by key.
     *
     * @template TGetDefault
     *
     * @param  TKey|null  $key
     * @param  TGetDefault|(\Closure(): TGetDefault)  $default
     * @return TValue|TGetDefault
     */
    public function get($key, $default = null)
    {}

    /**
     * Get an item from the collection by key or add it to collection if it does not exist.
     *
     * @template TGetOrPutValue
     *
     * @param  mixed  $key
     * @param  TGetOrPutValue|(\Closure(): TGetOrPutValue)  $value
     * @return TValue|TGetOrPutValue
     *
     * @phpstan-this-out static<TKey, TValue|TGetOrPutValue>
     */
    public function getOrPut($key, $value)
    {}

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function groupBy($groupBy, $preserveKeys = false)
    {}

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function keyBy($keyBy)
    {}

    /**
     * Determine if an item exists in the collection by key.
     *
     * @param  TKey|array<array-key, TKey>  $key
     * @return bool
     */
    public function has($key)
    {}

    /**
     * Determine if any of the keys exist in the collection.
     *
     * @param  TKey|array<array-key, TKey>  $key
     * @return bool
     */
    public function hasAny($key)
    {}

    /**
     * Concatenate values of a given key as a string.
     *
     * @param  (callable(TValue, TKey): mixed)|string|null  $value
     * @param  string|null  $glue
     * @return string
     */
    public function implode($value, $glue = null)
    {}

    /**
     * Intersect the collection with the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @return static
     */
    public function intersect($items)
    {}

    /**
     * Intersect the collection with the given items, using the callback.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TValue>|iterable<array-key, TValue>  $items
     * @param  callable(TValue, TValue): int  $callback
     * @return static
     */
    public function intersectUsing($items, callable $callback)
    {}

    /**
     * Intersect the collection with the given items with additional index check.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @return static
     */
    public function intersectAssoc($items)
    {}

    /**
     * Intersect the collection with the given items with additional index check, using the callback.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TValue>|iterable<array-key, TValue>  $items
     * @param  callable(TValue, TValue): int  $callback
     * @return static
     */
    public function intersectAssocUsing($items, callable $callback)
    {}

    /**
     * Intersect the collection with the given items by key.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, mixed>|iterable<TKey, mixed>  $items
     * @return static
     */
    public function intersectByKeys($items)
    {}

    /**
     * Determine if the collection is empty or not.
     *
     * @phpstan-assert-if-true null $this->first()
     * @phpstan-assert-if-true null $this->last()
     *
     * @phpstan-assert-if-false TValue $this->first()
     * @phpstan-assert-if-false TValue $this->last()
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the collection contains exactly one item. If a callback is provided, determine if exactly one item matches the condition.
     *
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @return bool
     *
     * @deprecated 12.49.0 Use the `hasSole()` method instead.
     */
    public function containsOneItem(?callable $callback = null): bool
    {}

    /**
     * Determine if the collection contains multiple items.
     *
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @return bool
     *
     * @deprecated 12.50.0 Use the `hasMany()` method instead.
     */
    public function containsManyItems(?callable $callback = null): bool
    {}

    /**
     * Join all items from the collection using a string. The final items can use a separate glue string.
     *
     * @param  string  $glue
     * @param  string  $finalGlue
     * @return TValue|string
     */
    public function join($glue, $finalGlue = '')
    {}

    /**
     * Get the keys of the collection items.
     *
     * @return static<int, TKey>
     */
    public function keys()
    {}

    /**
     * Get the last item from the collection.
     *
     * @template TLastDefault
     *
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @param  TLastDefault|(\Closure(): TLastDefault)  $default
     * @return TValue|TLastDefault
     */
    public function last(?callable $callback = null, $default = null)
    {}

    /**
     * Get the values of a given key.
     *
     * @param  \Closure|string|int|array<array-key, string>|null  $value
     * @param  \Closure|string|null  $key
     * @return static<array-key, mixed>
     */
    public function pluck($value, $key = null)
    {}

    /**
     * Run a map over each of the items.
     *
     * @template TMapValue
     *
     * @param  callable(TValue, TKey): TMapValue  $callback
     * @return static<TKey, TMapValue>
     */
    public function map(callable $callback)
    {}

    /**
     * Run a dictionary map over the items.
     *
     * The callback should return an associative array with a single key/value pair.
     *
     * @template TMapToDictionaryKey of array-key
     * @template TMapToDictionaryValue
     *
     * @param  callable(TValue, TKey): array<TMapToDictionaryKey, TMapToDictionaryValue>  $callback
     * @return static<TMapToDictionaryKey, array<int, TMapToDictionaryValue>>
     */
    public function mapToDictionary(callable $callback)
    {}

    /**
     * Run an associative map over each of the items.
     *
     * The callback should return an associative array with a single key/value pair.
     *
     * @template TMapWithKeysKey of array-key
     * @template TMapWithKeysValue
     *
     * @param  callable(TValue, TKey): array<TMapWithKeysKey, TMapWithKeysValue>  $callback
     * @return static<TMapWithKeysKey, TMapWithKeysValue>
     */
    public function mapWithKeys(callable $callback)
    {}

    /**
     * Merge the collection with the given items.
     *
     * @template TMergeValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TMergeValue>|iterable<TKey, TMergeValue>  $items
     * @return static<TKey, TValue|TMergeValue>
     */
    public function merge($items)
    {}

    /**
     * Recursively merge the collection with the given items.
     *
     * @template TMergeRecursiveValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TMergeRecursiveValue>|iterable<TKey, TMergeRecursiveValue>  $items
     * @return static<TKey, TValue|TMergeRecursiveValue>
     */
    public function mergeRecursive($items)
    {}

    /**
     * Multiply the items in the collection by the multiplier.
     *
     * @param  int  $multiplier
     * @return static
     */
    public function multiply(int $multiplier)
    {}

    /**
     * Create a collection by using this collection for keys and another for its values.
     *
     * @template TCombineValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TCombineValue>|iterable<array-key, TCombineValue>  $values
     * @return static<TValue, TCombineValue>
     */
    public function combine($values)
    {}

    /**
     * Union the collection with the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @return static
     */
    public function union($items)
    {}

    /**
     * Create a new collection consisting of every n-th element.
     *
     * @param  int  $step
     * @param  int  $offset
     * @return ($step is positive-int ? static : never)
     *
     * @throws \InvalidArgumentException
     */
    public function nth($step, $offset = 0)
    {}

    /**
     * Get the items with the specified keys.
     *
     * @param  \Illuminate\Support\Enumerable<array-key, TKey>|array<array-key, TKey>|string|null  $keys
     * @return static
     */
    public function only($keys)
    {}

    /**
     * Select specific values from the items within the collection.
     *
     * @param  \Illuminate\Support\Enumerable<int, string>|list<string>|string|null  $keys
     * @return static
     */
    public function select($keys)
    {}

    /**
     * Get and remove the last N items from the collection.
     *
     * @param  int  $count
     * @return ($count is 1 ? TValue|null : static<int, TValue>)
     */
    public function pop($count = 1)
    {}

    /**
     * Push an item onto the beginning of the collection.
     *
     * @param  TValue  $value
     * @param  TKey|null  $key
     * @return $this
     *
     * @phpstan-this-out ($key is null ? static<TKey|int, TValue> : $this)
     */
    public function prepend($value, $key = null)
    {}

    /**
     * Push one or more items onto the end of the collection.
     *
     * @param  TValue  ...$values
     * @return $this
     *
     * @phpstan-this-out static<TKey|int, TValue>
     */
    public function push(...$values)
    {}

    /**
     * Prepend one or more items to the beginning of the collection.
     *
     * @param  TValue  ...$values
     * @return $this
     *
     * @phpstan-this-out static<TKey|int, TValue>
     */
    public function unshift(...$values)
    {}

    /**
     * Push all of the given items onto the collection.
     *
     * @template TConcatKey of array-key
     * @template TConcatValue
     *
     * @param  iterable<TConcatKey, TConcatValue>  $source
     * @return static<TKey|TConcatKey, TValue|TConcatValue>
     */
    public function concat($source)
    {}

    /**
     * Get and remove an item from the collection.
     *
     * @template TPullDefault
     *
     * @param  TKey  $key
     * @param  TPullDefault|(\Closure(): TPullDefault)  $default
     * @return TValue|TPullDefault
     */
    public function pull($key, $default = null)
    {}

    /**
     * Put an item in the collection by key.
     *
     * @template TPutValue
     *
     * @param  TKey  $key
     * @param  TPutValue  $value
     * @return $this
     *
     * @phpstan-this-out static<TKey, TValue|TPutValue>
     */
    public function put($key, $value)
    {}

    /**
     * Get one or a specified number of items randomly from the collection.
     *
     * @param  (callable(self<TKey, TValue>): int)|int|null  $number
     * @param  bool  $preserveKeys
     * @return ($number is null ? TValue : static<int, TValue>)
     *
     * @throws \InvalidArgumentException
     */
    public function random($number = null, $preserveKeys = false)
    {}

    /**
     * Replace the collection items with the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @return static
     */
    public function replace($items)
    {}

    /**
     * Recursively replace the collection items with the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @return static
     */
    public function replaceRecursive($items)
    {}

    /**
     * Reverse items order.
     *
     * @return static
     */
    public function reverse()
    {}

    /**
     * Search the collection for a given value and return the corresponding key if successful.
     *
     * @param  TValue|(callable(TValue,TKey): bool)  $value
     * @param  bool  $strict
     * @return TKey|false
     */
    public function search($value, $strict = false)
    {}

    /**
     * Get the item before the given item.
     *
     * @param  TValue|(callable(TValue,TKey): bool)  $value
     * @param  bool  $strict
     * @return TValue|null
     */
    public function before($value, $strict = false)
    {}

    /**
     * Get the item after the given item.
     *
     * @param  TValue|(callable(TValue,TKey): bool)  $value
     * @param  bool  $strict
     * @return TValue|null
     */
    public function after($value, $strict = false)
    {}

    /**
     * Get and remove the first N items from the collection.
     *
     * @param  int<0, max>  $count
     * @return ($count is 1 ? TValue|null : static<int, TValue>)
     *
     * @throws \InvalidArgumentException
     */
    public function shift($count = 1)
    {}

    /**
     * Shuffle the items in the collection.
     *
     * @return static
     */
    public function shuffle()
    {}

    /**
     * Create chunks representing a "sliding window" view of the items in the collection.
     *
     * @param  positive-int  $size
     * @param  positive-int  $step
     * @return static<int, static>
     *
     * @throws \InvalidArgumentException
     */
    public function sliding($size = 2, $step = 1)
    {}

    /**
     * Skip the first {$count} items.
     *
     * @param  int  $count
     * @return static
     */
    public function skip($count)
    {}

    /**
     * Skip items in the collection until the given condition is met.
     *
     * @param  TValue|callable(TValue,TKey): bool  $value
     * @return static
     */
    public function skipUntil($value)
    {}

    /**
     * Skip items in the collection while the given condition is met.
     *
     * @param  TValue|callable(TValue,TKey): bool  $value
     * @return static
     */
    public function skipWhile($value)
    {}

    /**
     * Slice the underlying collection array.
     *
     * @param  int  $offset
     * @param  int|null  $length
     * @return static
     */
    public function slice($offset, $length = null)
    {}

    /**
     * Split a collection into a certain number of groups.
     *
     * @param  int  $numberOfGroups
     * @return ($numberOfGroups is positive-int ? static<int, static> : never)
     *
     * @throws \InvalidArgumentException
     */
    public function split($numberOfGroups)
    {}

    /**
     * Split a collection into a certain number of groups, and fill the first groups completely.
     *
     * @param  int  $numberOfGroups
     * @return ($numberOfGroups is positive-int ? static<int, static> : never)
     *
     * @throws \InvalidArgumentException
     */
    public function splitIn($numberOfGroups)
    {}

    /**
     * Get the first item in the collection, but only if exactly one item exists. Otherwise, throw an exception.
     *
     * @param  (callable(TValue, TKey): bool)|string|null  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return TValue
     *
     * @throws \Illuminate\Support\ItemNotFoundException
     * @throws \Illuminate\Support\MultipleItemsFoundException
     */
    public function sole($key = null, $operator = null, $value = null)
    {}

    /**
     * Determine if the collection contains a single item, optionally matching the given criteria.
     *
     * @param  (callable(TValue, TKey): bool)|string|null  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function hasSole($key = null, $operator = null, $value = null): bool
    {}

    /**
     * Get the first item in the collection but throw an exception if no matching items exist.
     *
     * @param  (callable(TValue, TKey): bool)|string|null  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return TValue
     *
     * @throws \Illuminate\Support\ItemNotFoundException
     */
    public function firstOrFail($key = null, $operator = null, $value = null)
    {}

    /**
     * Chunk the collection into chunks of the given size.
     *
     * @param  int  $size
     * @param  bool  $preserveKeys
     * @return ($preserveKeys is true ? static<int, static> : static<int, static<int, TValue>>)
     */
    public function chunk($size, $preserveKeys = true)
    {}

    /**
     * Chunk the collection into chunks with a callback.
     *
     * @param  callable(TValue, TKey, static<TKey, TValue>): bool  $callback
     * @return static<int, static<TKey, TValue>>
     */
    public function chunkWhile(callable $callback)
    {}

    /**
     * Sort through each item with a callback.
     *
     * @param  (callable(TValue, TValue): int)|null|int  $callback
     * @return static
     */
    public function sort($callback = null)
    {}

    /**
     * Sort items in descending order.
     *
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @return static
     */
    public function sortDesc($options = SORT_REGULAR)
    {}

    /**
     * Sort the collection using the given callback.
     *
     * @param  array<array-key, (callable(TValue, TValue): mixed)|(callable(TValue, TKey): mixed)|string|array{string, SortDirection|'asc'|'desc'}>|(callable(TValue, TKey): mixed)|string|int  $callback
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @param  SortDirection|bool  $descending
     * @return static
     */
    public function sortBy($callback, $options = SORT_REGULAR, $descending = false)
    {}

    /**
     * Sort the collection using multiple comparisons.
     *
     * @param  array<array-key, (callable(TValue, TValue): mixed)|(callable(TValue, TKey): mixed)|string|array{string, SortDirection|'asc'|'desc'}>  $comparisons
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @return static
     */
    protected function sortByMany(array $comparisons = [], int $options = SORT_REGULAR)
    {}

    /**
     * Sort the collection in descending order using the given callback.
     *
     * @param  array<array-key, (callable(TValue, TValue): mixed)|(callable(TValue, TKey): mixed)|string|array{string, SortDirection|'asc'|'desc'}>|(callable(TValue, TKey): mixed)|string|int  $callback
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @return static
     */
    public function sortByDesc($callback, $options = SORT_REGULAR)
    {}

    /**
     * Sort the collection keys.
     *
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @param  SortDirection|bool  $descending
     * @return static
     */
    public function sortKeys($options = SORT_REGULAR, $descending = false)
    {}

    /**
     * Sort the collection keys in descending order.
     *
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @return static
     */
    public function sortKeysDesc($options = SORT_REGULAR)
    {}

    /**
     * Sort the collection keys using a callback.
     *
     * @param  callable(TKey, TKey): int  $callback
     * @return static
     */
    public function sortKeysUsing(callable $callback)
    {}

    /**
     * Splice a portion of the underlying collection array.
     *
     * @param  int  $offset
     * @param  int|null  $length
     * @param  array<array-key, TValue>  $replacement
     * @return static
     */
    public function splice($offset, $length = null, $replacement = [])
    {}

    /**
     * Take the first or last {$limit} items.
     *
     * @param  int  $limit
     * @return static
     */
    public function take($limit)
    {}

    /**
     * Take items in the collection until the given condition is met.
     *
     * @param  TValue|callable(TValue,TKey): bool  $value
     * @return static
     */
    public function takeUntil($value)
    {}

    /**
     * Take items in the collection while the given condition is met.
     *
     * @param  TValue|callable(TValue,TKey): bool  $value
     * @return static
     */
    public function takeWhile($value)
    {}

    /**
     * Transform each item in the collection using a callback.
     *
     * @template TMapValue
     *
     * @param  callable(TValue, TKey): TMapValue  $callback
     * @return $this
     *
     * @phpstan-this-out static<TKey, TMapValue>
     */
    public function transform(callable $callback)
    {}

    /**
     * Flatten a multi-dimensional associative array with dots.
     *
     * @param  int  $depth
     * @return static
     */
    public function dot($depth = INF)
    {}

    /**
     * Convert a flatten "dot" notation array into an expanded array.
     *
     * @return static
     */
    public function undot()
    {}

    /**
     * Return only unique items from the collection array.
     *
     * @param  (callable(TValue, TKey): mixed)|string|null  $key
     * @param  bool  $strict
     * @return static
     */
    public function unique($key = null, $strict = false)
    {}

    /**
     * Reset the keys on the underlying array.
     *
     * @return static<int, TValue>
     */
    public function values()
    {}

    /**
     * Zip the collection together with one or more arrays.
     *
     * e.g. new Collection([1, 2, 3])->zip([4, 5, 6]);
     *      => [[1, 4], [2, 5], [3, 6]]
     *
     * @template TZipValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TZipValue>|iterable<array-key, TZipValue>  ...$items
     * @return static<int, static<int, TValue|TZipValue>>
     */
    public function zip($items)
    {}

    /**
     * Pad collection to the specified length with a value.
     *
     * @template TPadValue
     *
     * @param  int  $size
     * @param  TPadValue  $value
     * @return static<int, TValue|TPadValue>
     */
    public function pad($size, $value)
    {}

    /**
     * Get an iterator for the items.
     *
     * @return \ArrayIterator<TKey, TValue>
     */
    public function getIterator(): Traversable
    {}

    /**
     * Count the number of items in the collection.
     *
     * @return int<0, max>
     */
    public function count(): int
    {}

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function countBy($countBy = null)
    {}

    /**
     * Add an item to the collection.
     *
     * @param  TValue  $item
     * @return $this
     *
     * @phpstan-this-out static<TKey|int, TValue>
     */
    public function add($item)
    {}

    /**
     * Get a base Support collection instance from this collection.
     *
     * @return \Illuminate\Support\Collection<TKey, TValue>
     */
    public function toBase()
    {}

    /**
     * Determine if an item exists at an offset.
     *
     * @param  TKey  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get an item at a given offset.
     *
     * @param  TKey  $offset
     * @return TValue
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the item at a given offset.
     *
     * @param  TKey|null  $offset
     * @param  TValue  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the item at a given offset.
     *
     * @param  TKey  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}
}
