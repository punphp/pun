<?php

namespace Illuminate\Support;

use ArrayIterator;
use Closure;
use DateInterval;
use DateTimeImmutable;
use DateTimeInterface;
use Generator;
use Illuminate\Contracts\Support\CanBeEscapedWhenCastToString;
use Illuminate\Support\Traits\EnumeratesValues;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use IteratorAggregate;
use stdClass;
use Traversable;
use ValueError;

/**
 * @template TKey of array-key
 *
 * @template-covariant TValue
 *
 * @implements \Illuminate\Support\Enumerable<TKey, TValue>
 */
class LazyCollection implements CanBeEscapedWhenCastToString, Enumerable
{
    /**
     * @use \Illuminate\Support\Traits\EnumeratesValues<TKey, TValue>
     */
    use EnumeratesValues, Macroable;

    /**
     * The source from which to generate items.
     *
     * @var (Closure(): \Generator<TKey, TValue, mixed, void>)|static|array<TKey, TValue>
     */
    public $source;

    /**
     * Create a new lazy collection instance.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>|(Closure(): \Generator<TKey, TValue, mixed, void>)|self<TKey, TValue>|array<TKey, TValue>|null  $source
     *
     * @throws \InvalidArgumentException
     */
    public function __construct($source = null)
    {}

    /**
     * Create a new instance of the collection.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>|(Closure(): \Generator<TKey, TValue, mixed, void>)|self<TKey, TValue>|array<TKey, TValue>|null  $items
     * @return static
     */
    protected function newInstance($items = [])
    {}

    /**
     * Create a new collection instance if the value isn't one already.
     *
     * @template TMakeKey of array-key
     * @template TMakeValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TMakeKey, TMakeValue>|iterable<TMakeKey, TMakeValue>|(Closure(): \Generator<TMakeKey, TMakeValue, mixed, void>)|self<TMakeKey, TMakeValue>|array<TMakeKey, TMakeValue>|null  $items
     * @return static<TMakeKey, TMakeValue>
     */
    public static function make($items = [], ...$args)
    {}

    /**
     * Create a collection with the given range.
     *
     * @param  int  $from
     * @param  int  $to
     * @param  int  $step
     * @return ($step is 0 ? never : static<int, int>)
     *
     * @throws \InvalidArgumentException
     */
    public static function range($from, $to, $step = 1, ...$args)
    {}

    /**
     * Get all items in the enumerable.
     *
     * @return array<TKey, TValue>
     */
    public function all()
    {}

    /**
     * Eager load all items into a new lazy collection backed by an array.
     *
     * @return static<TKey, TValue>
     */
    public function eager()
    {}

    /**
     * Cache values as they're enumerated.
     *
     * @return static<TKey, TValue>
     */
    public function remember()
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
     * @param  string|array<string>|null  $key
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
     * Determine if an item exists in the enumerable.
     *
     * @param  (callable(TValue, TKey): bool)|TValue|string  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function contains($key, $operator = null, $value = null)
    { func_get_args(); }

    /**
     * Determine if an item exists, using strict comparison.
     *
     * @param  (callable(TValue): bool)|TValue|array-key  $key
     * @param  TValue|null  $value
     * @return bool
     */
    public function containsStrict($key, $value = null)
    { func_get_args(); }

    /**
     * Determine if an item is not contained in the enumerable.
     *
     * @param  mixed  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function doesntContain($key, $operator = null, $value = null)
    { func_get_args(); }

    /**
     * Determine if an item is not contained in the enumerable, using strict comparison.
     *
     * @param  mixed  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function doesntContainStrict($key, $operator = null, $value = null)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function crossJoin(...$arrays)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function countBy($countBy = null)
    {}

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function diff($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function diffUsing($items, callable $callback)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function diffAssoc($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function diffAssocUsing($items, callable $callback)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function diffKeys($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function diffKeysUsing($items, callable $callback)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function duplicates($callback = null, $strict = false)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function duplicatesStrict($callback = null)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function except($keys)
    { func_get_args(); }

    /**
     * Run a filter over each of the items.
     *
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @return static
     */
    public function filter(?callable $callback = null)
    {}

    /**
     * Get the first item from the enumerable passing the given truth test.
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
     * Get a flattened list of the items in the collection.
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
     * Get an item by key.
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
     * {@inheritDoc}
     */
    #[\Override]
    public function groupBy($groupBy, $preserveKeys = false)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function keyBy($keyBy)
    {}

    /**
     * Determine if an item exists in the collection by key.
     *
     * @param  mixed  $key
     * @return bool
     */
    public function has($key)
    { func_get_args(); }

    /**
     * Determine if any of the keys exist in the collection.
     *
     * @param  mixed  $key
     * @return bool
     */
    public function hasAny($key)
    { func_get_args(); }

    /**
     * Concatenate values of a given key as a string.
     *
     * @param  (callable(TValue, TKey): mixed)|string  $value
     * @param  string|null  $glue
     * @return string
     */
    public function implode($value, $glue = null)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function intersect($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function intersectUsing($items, callable $callback)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function intersectAssoc($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function intersectAssocUsing($items, callable $callback)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function intersectByKeys($items)
    { func_get_args(); }

    /**
     * Determine if the items are empty or not.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the collection contains a single item.
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
     * @return bool
     *
     * @deprecated 12.50.0 Use the `hasMany()` method instead.
     */
    public function containsManyItems(): bool
    {}

    /**
     * Join all items from the collection using a string. The final items can use a separate glue string.
     *
     * @param  string  $glue
     * @param  string  $finalGlue
     * @return string
     */
    public function join($glue, $finalGlue = '')
    { func_get_args(); }

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
     * {@inheritDoc}
     */
    #[\Override]
    public function mapToDictionary(callable $callback)
    { func_get_args(); }

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
     * {@inheritDoc}
     */
    #[\Override]
    public function merge($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function mergeRecursive($items)
    { func_get_args(); }

    /**
     * Multiply the items in the collection by the multiplier.
     *
     * @param  int  $multiplier
     * @return static
     */
    public function multiply(int $multiplier)
    { func_get_args(); }

    /**
     * Create a collection by using this collection for keys and another for its values.
     *
     * @template TCombineValue
     *
     * @param  \IteratorAggregate<array-key, TCombineValue>|array<array-key, TCombineValue>|(callable(): \Generator<array-key, TCombineValue>)  $values
     * @return static<TValue, TCombineValue>
     */
    public function combine($values)
    {}

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function union($items)
    { func_get_args(); }

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
    { func_get_args(); }

    /**
     * Select specific values from the items within the collection.
     *
     * @param  \Illuminate\Support\Enumerable<array-key, TKey>|array<array-key, TKey>|string|null  $keys
     * @return static
     */
    public function select($keys)
    { func_get_args(); }

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
     * Get one or a specified number of items randomly from the collection.
     *
     * @param  int|null  $number
     * @param  bool  $preserveKeys
     * @return ($number is null ? TValue : static<int, TValue>)
     *
     * @throws \InvalidArgumentException
     */
    public function random($number = null, $preserveKeys = false)
    { func_get_args(); }

    /**
     * Replace the collection items with the given items.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TKey, TValue>|iterable<TKey, TValue>  $items
     * @return static
     */
    public function replace($items)
    {}

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function replaceRecursive($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function reverse()
    { func_get_args(); }

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
     * {@inheritDoc}
     */
    #[\Override]
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
     * {@inheritDoc}
     */
    #[\Override]
    public function slice($offset, $length = null)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     *
     * @throws \InvalidArgumentException
     */
    #[\Override]
    public function split($numberOfGroups)
    { func_get_args(); }

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
    { func_get_args(); }

    /**
     * Determine if the collection contains a single item or a single item matching the given criteria.
     *
     * @param  (callable(TValue, TKey): bool)|string|null  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function hasSole($key = null, $operator = null, $value = null): bool
    { func_get_args(); }

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
    { func_get_args(); }

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
     * Chunk the collection into chunks with a callback.
     *
     * @param  callable(TValue, TKey, Collection<TKey, TValue>): bool  $callback
     * @return static<int, static<TKey, TValue>>
     */
    public function chunkWhile(callable $callback)
    {}

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function sort($callback = null)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function sortDesc($options = SORT_REGULAR)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function sortBy($callback, $options = SORT_REGULAR, $descending = false)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function sortByDesc($callback, $options = SORT_REGULAR)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function sortKeys($options = SORT_REGULAR, $descending = false)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function sortKeysDesc($options = SORT_REGULAR)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function sortKeysUsing(callable $callback)
    { func_get_args(); }

    /**
     * Take the first or last {$limit} items.
     *
     * @param  int  $limit
     * @return static<TKey, TValue>
     */
    public function take($limit)
    {}

    /**
     * Take items in the collection until the given condition is met.
     *
     * @param  TValue|callable(TValue,TKey): bool  $value
     * @return static<TKey, TValue>
     */
    public function takeUntil($value)
    {}

    /**
     * Take items in the collection until a given point in time, with an optional callback on timeout.
     *
     * @param  \DateTimeInterface  $timeout
     * @param  callable(TValue|null, TKey|null): mixed|null  $callback
     * @return static<TKey, TValue>
     */
    public function takeUntilTimeout(DateTimeInterface $timeout, ?callable $callback = null)
    {}

    /**
     * Take items in the collection while the given condition is met.
     *
     * @param  TValue|callable(TValue,TKey): bool  $value
     * @return static<TKey, TValue>
     */
    public function takeWhile($value)
    {}

    /**
     * Pass each item in the collection to the given callback, lazily.
     *
     * @param  callable(TValue, TKey): mixed  $callback
     * @return static<TKey, TValue>
     */
    public function tapEach(callable $callback)
    {}

    /**
     * Throttle the values, releasing them at most once per the given seconds.
     *
     * @return static<TKey, TValue>
     */
    public function throttle(float $seconds)
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
     * {@inheritDoc}
     */
    #[\Override]
    public function undot()
    {}

    /**
     * Return only unique items from the collection array.
     *
     * @param  (callable(TValue, TKey): mixed)|string|null  $key
     * @param  bool  $strict
     * @return static<TKey, TValue>
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
     * Run the given callback every time the interval has passed.
     *
     * @return static<TKey, TValue>
     */
    public function withHeartbeat(DateInterval|int $interval, callable $callback)
    {}

    /**
     * Get the total seconds from the given interval.
     */
    protected function intervalSeconds(DateInterval $interval): int
    {}

    /**
     * Zip the collection together with one or more arrays.
     *
     * e.g. new LazyCollection([1, 2, 3])->zip([4, 5, 6]);
     *      => [[1, 4], [2, 5], [3, 6]]
     *
     * @template TZipValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<array-key, TZipValue>|iterable<array-key, TZipValue>  ...$items
     * @return static<int, static<int, TValue|TZipValue>>
     */
    public function zip($items)
    { func_get_args(); }

    /**
     * {@inheritDoc}
     */
    #[\Override]
    public function pad($size, $value)
    { func_get_args(); }

    /**
     * Get the values iterator.
     *
     * @return \Traversable<TKey, TValue>
     */
    public function getIterator(): Traversable
    {}

    /**
     * Count the number of items in the collection.
     *
     * @return int
     */
    public function count(): int
    {}

    /**
     * Make an iterator from the given source.
     *
     * @template TIteratorKey of array-key
     * @template TIteratorValue
     *
     * @param  \IteratorAggregate<TIteratorKey, TIteratorValue>|array<TIteratorKey, TIteratorValue>|(callable(): \Generator<TIteratorKey, TIteratorValue>)  $source
     * @return \Traversable<TIteratorKey, TIteratorValue>
     */
    protected function makeIterator($source)
    {}

    /**
     * Explode the "value" and "key" arguments passed to "pluck".
     *
     * @param  \Closure|string|int|string[]|null  $value
     * @param  \Closure|string|string[]|null  $key
     * @return array{\Closure|string[]|int|null, \Closure|string[]|null}
     */
    protected function explodePluckParameters($value, $key)
    {}

    /**
     * Pass this lazy collection through a method on the collection class.
     *
     * @param  string  $method
     * @param  array<mixed>  $params
     * @return static
     */
    protected function passthru($method, array $params)
    {}

    /**
     * Get the current time.
     *
     * @return int
     */
    protected function now()
    {}

    /**
     * Get the precise current time.
     *
     * @return float
     */
    protected function preciseNow()
    {}

    /**
     * Sleep for the given amount of microseconds.
     *
     * @return void
     */
    protected function usleep(int $microseconds)
    {}
}
