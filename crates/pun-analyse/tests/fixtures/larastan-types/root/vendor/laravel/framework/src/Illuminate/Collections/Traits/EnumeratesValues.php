<?php

namespace Illuminate\Support\Traits;

use BackedEnum;
use CachingIterator;
use Closure;
use Exception;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;
use Illuminate\Support\HigherOrderCollectionProxy;
use JsonSerializable;
use UnexpectedValueException;
use UnitEnum;

use function Illuminate\Support\enum_value;

/**
 * @template TKey of array-key
 *
 * @template-covariant TValue
 *
 * @property-read HigherOrderCollectionProxy<'average', TValue, static> $average
 * @property-read HigherOrderCollectionProxy<'avg', TValue, static> $avg
 * @property-read HigherOrderCollectionProxy<'contains', TValue, static> $contains
 * @property-read HigherOrderCollectionProxy<'doesntContain', TValue, static> $doesntContain
 * @property-read HigherOrderCollectionProxy<'each', TValue, static> $each
 * @property-read HigherOrderCollectionProxy<'every', TValue, static> $every
 * @property-read HigherOrderCollectionProxy<'filter', TValue, static> $filter
 * @property-read HigherOrderCollectionProxy<'first', TValue, static> $first
 * @property-read HigherOrderCollectionProxy<'flatMap', TValue, static> $flatMap
 * @property-read HigherOrderCollectionProxy<'groupBy', TValue, static> $groupBy
 * @property-read HigherOrderCollectionProxy<'hasMany', TValue, static> $hasMany
 * @property-read HigherOrderCollectionProxy<'hasSole', TValue, static> $hasSole
 * @property-read HigherOrderCollectionProxy<'keyBy', TValue, static> $keyBy
 * @property-read HigherOrderCollectionProxy<'last', TValue, static> $last
 * @property-read HigherOrderCollectionProxy<'map', TValue, static> $map
 * @property-read HigherOrderCollectionProxy<'max', TValue, static> $max
 * @property-read HigherOrderCollectionProxy<'min', TValue, static> $min
 * @property-read HigherOrderCollectionProxy<'partition', TValue, static> $partition
 * @property-read HigherOrderCollectionProxy<'percentage', TValue, static> $percentage
 * @property-read HigherOrderCollectionProxy<'reject', TValue, static> $reject
 * @property-read HigherOrderCollectionProxy<'skipUntil', TValue, static> $skipUntil
 * @property-read HigherOrderCollectionProxy<'skipWhile', TValue, static> $skipWhile
 * @property-read HigherOrderCollectionProxy<'sole', TValue, static> $sole
 * @property-read HigherOrderCollectionProxy<'some', TValue, static> $some
 * @property-read HigherOrderCollectionProxy<'sortBy', TValue, static> $sortBy
 * @property-read HigherOrderCollectionProxy<'sortByDesc', TValue, static> $sortByDesc
 * @property-read HigherOrderCollectionProxy<'sum', TValue, static> $sum
 * @property-read HigherOrderCollectionProxy<'takeUntil', TValue, static> $takeUntil
 * @property-read HigherOrderCollectionProxy<'takeWhile', TValue, static> $takeWhile
 * @property-read HigherOrderCollectionProxy<'unique', TValue, static> $unique
 * @property-read HigherOrderCollectionProxy<'unless', TValue, static> $unless
 * @property-read HigherOrderCollectionProxy<'when', TValue, static> $when
 */
trait EnumeratesValues
{
    use Conditionable;

    /**
     * Indicates that the object's string representation should be escaped when __toString is invoked.
     *
     * @var bool
     */
    protected $escapeWhenCastingToString = false;

    /**
     * The methods that can be proxied.
     *
     * @var array<int, string>
     */
    protected static $proxies = [
        'average',
        'avg',
        'contains',
        'doesntContain',
        'each',
        'every',
        'filter',
        'first',
        'flatMap',
        'groupBy',
        'hasMany',
        'hasSole',
        'keyBy',
        'last',
        'map',
        'max',
        'min',
        'partition',
        'percentage',
        'reject',
        'skipUntil',
        'skipWhile',
        'sole',
        'some',
        'sortBy',
        'sortByDesc',
        'sum',
        'takeUntil',
        'takeWhile',
        'unique',
        'unless',
        'when',
    ];

    /**
     * Create a new collection instance if the value isn't one already.
     *
     * @template TMakeKey of array-key
     * @template TMakeValue
     *
     * @param  \Illuminate\Contracts\Support\Arrayable<TMakeKey, TMakeValue>|iterable<TMakeKey, TMakeValue>|null  $items
     * @return static<TMakeKey, TMakeValue>
     */
    public static function make($items = [], ...$args)
    {}

    /**
     * Wrap the given value in a collection if applicable.
     *
     * @template TWrapValue
     *
     * @param  iterable<array-key, TWrapValue>|TWrapValue  $value
     * @return static<array-key, TWrapValue>
     */
    public static function wrap($value, ...$args)
    {}

    /**
     * Get the underlying items from the given collection if applicable.
     *
     * @template TUnwrapKey of array-key
     * @template TUnwrapValue
     *
     * @param  array<TUnwrapKey, TUnwrapValue>|static<TUnwrapKey, TUnwrapValue>  $value
     * @return array<TUnwrapKey, TUnwrapValue>
     */
    public static function unwrap($value)
    {}

    /**
     * Create a new instance with no items.
     *
     * @return static
     */
    public static function empty(...$args)
    {}

    /**
     * Create a new collection by invoking the callback a given amount of times.
     *
     * @template TTimesValue
     *
     * @param  int  $number
     * @param  (callable(int): TTimesValue)|null  $callback
     * @return static<int, TTimesValue>
     */
    public static function times($number, ?callable $callback = null, ...$args)
    {}

    /**
     * Create a new collection by decoding a JSON string.
     *
     * @param  string  $json
     * @param  int  $depth
     * @param  int  $flags
     * @return static<TKey, TValue>
     */
    public static function fromJson($json, $depth = 512, $flags = 0, ...$args)
    {}

    /**
     * Get the average value of a given key.
     *
     * @param  (callable(TValue): float|int)|string|null  $callback
     * @return float|int|null
     */
    public function avg($callback = null)
    {}

    /**
     * Alias for the "avg" method.
     *
     * @param  (callable(TValue): float|int)|string|null  $callback
     * @return float|int|null
     */
    public function average($callback = null)
    {}

    /**
     * Alias for the "contains" method.
     *
     * @param  (callable(TValue, TKey): bool)|TValue|string  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function some($key, $operator = null, $value = null)
    {}

    /**
     * Dump the given arguments and terminate execution.
     *
     * @param  mixed  ...$args
     * @return never
     */
    public function dd(...$args)
    {}

    /**
     * Dump the items.
     *
     * @param  mixed  ...$args
     * @return $this
     */
    public function dump(...$args)
    {}

    /**
     * Execute a callback over each item.
     *
     * @param  callable(TValue, TKey): mixed  $callback
     * @return $this
     */
    public function each(callable $callback)
    {}

    /**
     * Execute a callback over each nested chunk of items.
     *
     * @param  callable(...mixed): mixed  $callback
     * @return static
     */
    public function eachSpread(callable $callback)
    {}

    /**
     * Determine if all items pass the given truth test.
     *
     * @param  (callable(TValue, TKey): bool)|TValue|string  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function every($key, $operator = null, $value = null)
    {}

    /**
     * Get the first item by the given key value pair.
     *
     * @param  callable|string  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return TValue|null
     */
    public function firstWhere($key, $operator = null, $value = null)
    {}

    /**
     * Determine if the collection contains multiple items, optionally matching the given criteria.
     *
     * @param  (callable(TValue, TKey): bool)|string|null  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return bool
     */
    public function hasMany($key = null, $operator = null, $value = null): bool
    {}

    /**
     * Get a single key's value from the first matching item in the collection.
     *
     * @template TValueDefault
     *
     * @param  string  $key
     * @param  TValueDefault|(\Closure(): TValueDefault)  $default
     * @return TValue|TValueDefault
     */
    public function value($key, $default = null)
    {}

    /**
     * Ensure that every item in the collection is of the expected type.
     *
     * @template TEnsureOfType
     *
     * @param  class-string<TEnsureOfType>|array<array-key, class-string<TEnsureOfType>>|'string'|'int'|'float'|'bool'|'array'|'null'  $type
     * @return static<TKey, TEnsureOfType>
     *
     * @throws \UnexpectedValueException
     */
    public function ensure($type)
    {}

    /**
     * Determine if the collection is not empty.
     *
     * @phpstan-assert-if-true TValue $this->first()
     * @phpstan-assert-if-true TValue $this->last()
     *
     * @phpstan-assert-if-false null $this->first()
     * @phpstan-assert-if-false null $this->last()
     *
     * @return bool
     */
    public function isNotEmpty()
    {}

    /**
     * Run a map over each nested chunk of items.
     *
     * @template TMapSpreadValue
     *
     * @param  callable(mixed...): TMapSpreadValue  $callback
     * @return static<TKey, TMapSpreadValue>
     */
    public function mapSpread(callable $callback)
    {}

    /**
     * Run a grouping map over the items.
     *
     * The callback should return an associative array with a single key/value pair.
     *
     * @template TMapToGroupsKey of array-key
     * @template TMapToGroupsValue
     *
     * @param  callable(TValue, TKey): array<TMapToGroupsKey, TMapToGroupsValue>  $callback
     * @return static<TMapToGroupsKey, static<int, TMapToGroupsValue>>
     */
    public function mapToGroups(callable $callback)
    {}

    /**
     * Map a collection and flatten the result by a single level.
     *
     * @template TFlatMapKey of array-key
     * @template TFlatMapValue
     *
     * @param  callable(TValue, TKey): (\Illuminate\Support\Collection<TFlatMapKey, TFlatMapValue>|array<TFlatMapKey, TFlatMapValue>)  $callback
     * @return static<TFlatMapKey, TFlatMapValue>
     */
    public function flatMap(callable $callback)
    {}

    /**
     * Map the values into a new class.
     *
     * @template TMapIntoValue
     *
     * @param  class-string<TMapIntoValue>  $class
     * @return static<TKey, TMapIntoValue>
     */
    public function mapInto($class)
    {}

    /**
     * Get the min value of a given key.
     *
     * @template TMinResult = mixed
     *
     * @param  (callable(TValue): TMinResult)|string|null  $callback
     * @return ($callback is callable ? ?TMinResult : ($callback is null ? ?TValue : mixed))
     */
    public function min($callback = null)
    {}

    /**
     * Get the max value of a given key.
     *
     * @template TMaxResult = mixed
     *
     * @param  (callable(TValue): TMaxResult)|string|null  $callback
     * @return ($callback is callable ? ?TMaxResult : ($callback is null ? ?TValue : mixed))
     */
    public function max($callback = null)
    {}

    /**
     * "Paginate" the collection by slicing it into a smaller collection.
     *
     * @param  int  $page
     * @param  int  $perPage
     * @return static
     */
    public function forPage($page, $perPage)
    {}

    /**
     * Partition the collection into two arrays using the given callback or key.
     *
     * @param  (callable(TValue, TKey): bool)|TValue|string  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return static<int<0, 1>, static<TKey, TValue>>
     */
    public function partition($key, $operator = null, $value = null)
    {}

    /**
     * Calculate the percentage of items that pass a given truth test.
     *
     * @param  (callable(TValue, TKey): bool)  $callback
     * @param  int  $precision
     * @return float|null
     */
    public function percentage(callable $callback, int $precision = 2)
    {}

    /**
     * Get the sum of the given values.
     *
     * @template TReturnType
     *
     * @param  (callable(TValue, TKey): TReturnType)|string|null  $callback
     * @return ($callback is string ? mixed : ($callback is callable ? TReturnType : mixed))
     */
    public function sum($callback = null)
    {}

    /**
     * Apply the callback if the collection is empty.
     *
     * @template TWhenEmptyReturnType
     *
     * @param  (callable($this): TWhenEmptyReturnType)  $callback
     * @param  (callable($this): TWhenEmptyReturnType)|null  $default
     * @return $this|TWhenEmptyReturnType
     */
    public function whenEmpty(callable $callback, ?callable $default = null)
    {}

    /**
     * Apply the callback if the collection is not empty.
     *
     * @template TWhenNotEmptyReturnType
     *
     * @param  callable($this): TWhenNotEmptyReturnType  $callback
     * @param  (callable($this): TWhenNotEmptyReturnType)|null  $default
     * @return $this|TWhenNotEmptyReturnType
     */
    public function whenNotEmpty(callable $callback, ?callable $default = null)
    {}

    /**
     * Apply the callback unless the collection is empty.
     *
     * @template TUnlessEmptyReturnType
     *
     * @param  callable($this): TUnlessEmptyReturnType  $callback
     * @param  (callable($this): TUnlessEmptyReturnType)|null  $default
     * @return $this|TUnlessEmptyReturnType
     */
    public function unlessEmpty(callable $callback, ?callable $default = null)
    {}

    /**
     * Apply the callback unless the collection is not empty.
     *
     * @template TUnlessNotEmptyReturnType
     *
     * @param  callable($this): TUnlessNotEmptyReturnType  $callback
     * @param  (callable($this): TUnlessNotEmptyReturnType)|null  $default
     * @return $this|TUnlessNotEmptyReturnType
     */
    public function unlessNotEmpty(callable $callback, ?callable $default = null)
    {}

    /**
     * Filter items by the given key value pair.
     *
     * @param  callable|string  $key
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return static
     */
    public function where($key, $operator = null, $value = null)
    {}

    /**
     * Filter items where the value for the given key is null.
     *
     * @param  string|null  $key
     * @return static
     */
    public function whereNull($key = null)
    {}

    /**
     * Filter items where the value for the given key is not null.
     *
     * @param  string|null  $key
     * @return static
     */
    public function whereNotNull($key = null)
    {}

    /**
     * Filter items by the given key value pair using strict comparison.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return static
     */
    public function whereStrict($key, $value)
    {}

    /**
     * Filter items by the given key value pair.
     *
     * @param  string  $key
     * @param  \Illuminate\Contracts\Support\Arrayable|iterable  $values
     * @param  bool  $strict
     * @return static
     */
    public function whereIn($key, $values, $strict = false)
    {}

    /**
     * Filter items by the given key value pair using strict comparison.
     *
     * @param  string  $key
     * @param  \Illuminate\Contracts\Support\Arrayable|iterable  $values
     * @return static
     */
    public function whereInStrict($key, $values)
    {}

    /**
     * Filter items such that the value of the given key is between the given values.
     *
     * @param  string  $key
     * @param  \Illuminate\Contracts\Support\Arrayable|iterable  $values
     * @return static
     */
    public function whereBetween($key, $values)
    {}

    /**
     * Filter items such that the value of the given key is not between the given values.
     *
     * @param  string  $key
     * @param  \Illuminate\Contracts\Support\Arrayable|iterable  $values
     * @return static
     */
    public function whereNotBetween($key, $values)
    {}

    /**
     * Filter items by the given key value pair.
     *
     * @param  string  $key
     * @param  \Illuminate\Contracts\Support\Arrayable|iterable  $values
     * @param  bool  $strict
     * @return static
     */
    public function whereNotIn($key, $values, $strict = false)
    {}

    /**
     * Filter items by the given key value pair using strict comparison.
     *
     * @param  string  $key
     * @param  \Illuminate\Contracts\Support\Arrayable|iterable  $values
     * @return static
     */
    public function whereNotInStrict($key, $values)
    {}

    /**
     * Filter the items, removing any items that don't match the given type(s).
     *
     * @template TWhereInstanceOf
     *
     * @param  class-string<TWhereInstanceOf>|array<array-key, class-string<TWhereInstanceOf>>  $type
     * @return static<TKey, TWhereInstanceOf>
     */
    public function whereInstanceOf($type)
    {}

    /**
     * Pass the collection to the given callback and return the result.
     *
     * @template TPipeReturnType
     *
     * @param  callable($this): TPipeReturnType  $callback
     * @return TPipeReturnType
     */
    public function pipe(callable $callback)
    {}

    /**
     * Pass the collection into a new class.
     *
     * @template TPipeIntoValue
     *
     * @param  class-string<TPipeIntoValue>  $class
     * @return TPipeIntoValue
     */
    public function pipeInto($class)
    {}

    /**
     * Pass the collection through a series of callable pipes and return the result.
     *
     * @param  array<callable>  $callbacks
     * @return mixed
     */
    public function pipeThrough($callbacks)
    {}

    /**
     * Reduce the collection to a single value.
     *
     * @template TReduceInitial
     * @template TReduceReturnType
     *
     * @param  callable(TReduceInitial|TReduceReturnType, TValue, TKey): TReduceReturnType  $callback
     * @param  TReduceInitial  $initial
     * @return TReduceInitial|TReduceReturnType
     */
    public function reduce(callable $callback, $initial = null)
    {}

    /**
     * Reduce the collection to a single value by mutating an initial value.
     *
     * @template TReduceIntoInitial
     *
     * @param  TReduceIntoInitial  $initial
     * @param  callable(TReduceIntoInitial, TValue, TKey): void  $callback
     * @return TReduceIntoInitial
     */
    public function reduceInto($initial, callable $callback)
    {}

    /**
     * Reduce the collection to multiple aggregate values.
     *
     * @param  callable  $callback
     * @param  mixed  ...$initial
     * @return array
     *
     * @throws \UnexpectedValueException
     */
    public function reduceSpread(callable $callback, ...$initial)
    {}

    /**
     * Reduce an associative collection to a single value.
     *
     * @template TReduceWithKeysInitial
     * @template TReduceWithKeysReturnType
     *
     * @param  callable(TReduceWithKeysInitial|TReduceWithKeysReturnType, TValue, TKey): TReduceWithKeysReturnType  $callback
     * @param  TReduceWithKeysInitial  $initial
     * @return TReduceWithKeysInitial|TReduceWithKeysReturnType
     */
    public function reduceWithKeys(callable $callback, $initial = null)
    {}

    /**
     * Create a collection of all elements that do not pass a given truth test.
     *
     * @param  (callable(TValue, TKey): bool)|bool|TValue  $callback
     * @return static
     */
    public function reject($callback = true)
    {}

    /**
     * Chunk the collection into chunks by comparing adjacent values using the given key or callback.
     *
     * @param  (callable(TValue, TKey): mixed)|string  $key
     * @return static<int, static<TKey, TValue>>
     */
    public function chunkBy($key)
    {}

    /**
     * Pass the collection to the given callback and then return it.
     *
     * @param  callable($this): mixed  $callback
     * @return $this
     */
    public function tap(callable $callback)
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
     * Return only unique items from the collection array using strict comparison.
     *
     * @param  (callable(TValue, TKey): mixed)|string|null  $key
     * @return static
     */
    public function uniqueStrict($key = null)
    {}

    /**
     * Collect the values into a collection.
     *
     * @return \Illuminate\Support\Collection<TKey, TValue>
     */
    public function collect()
    {}

    /**
     * Get the collection of items as a plain array.
     *
     * @return array<TKey, mixed>
     */
    public function toArray()
    {}

    /**
     * Convert the object into something JSON serializable.
     *
     * @return array<TKey, mixed>
     */
    public function jsonSerialize(): array
    {}

    /**
     * Get the collection of items as JSON.
     *
     * @param  int  $options
     * @return string
     */
    public function toJson($options = 0)
    {}

    /**
     * Get the collection of items as pretty print formatted JSON.
     *
     * @param  int  $options
     * @return string
     */
    public function toPrettyJson(int $options = 0)
    {}

    /**
     * Get a CachingIterator instance.
     *
     * @param  int  $flags
     * @return \CachingIterator
     */
    public function getCachingIterator($flags = CachingIterator::CALL_TOSTRING)
    {}

    /**
     * Convert the collection to its string representation.
     *
     * @return string
     */
    public function __toString()
    {}

    /**
     * Indicate that the model's string representation should be escaped when __toString is invoked.
     *
     * @param  bool  $escape
     * @return $this
     */
    public function escapeWhenCastingToString($escape = true)
    {}

    /**
     * Add a method to the list of proxied methods.
     *
     * @param  string  $method
     * @return void
     */
    public static function proxy($method)
    {}

    /**
     * Dynamically access collection proxies.
     *
     * @param  string  $key
     * @return mixed
     *
     * @throws \Exception
     */
    public function __get($key)
    {}

    /**
     * Results array of items from Collection or Arrayable.
     *
     * @param  mixed  $items
     * @return array<TKey, TValue>
     */
    protected function getArrayableItems($items)
    {}

    /**
     * Get an operator checker callback.
     *
     * @param  callable|string  $key
     * @param  string|null  $operator
     * @param  mixed  $value
     * @return \Closure
     */
    protected function operatorForWhere($key, $operator = null, $value = null)
    {}

    /**
     * Determine if the given value is callable, but not a string.
     *
     * @param  mixed  $value
     * @return bool
     */
    protected function useAsCallable($value)
    {}

    /**
     * Get a value retrieving callback.
     *
     * @param  callable|string|null  $value
     * @return callable
     */
    protected function valueRetriever($value)
    {}

    /**
     * Make a function to check an item's equality.
     *
     * @param  mixed  $value
     * @return \Closure(mixed): bool
     */
    protected function equality($value)
    {}

    /**
     * Make a function using another function, by negating its result.
     *
     * @param  \Closure  $callback
     * @return \Closure
     */
    protected function negate(Closure $callback)
    {}

    /**
     * Make a function that returns what's passed to it.
     *
     * @return \Closure(TValue): TValue
     */
    protected function identity()
    {}
}
