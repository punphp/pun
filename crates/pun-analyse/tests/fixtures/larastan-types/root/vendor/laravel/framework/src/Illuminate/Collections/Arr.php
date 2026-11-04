<?php

namespace Illuminate\Support;

use ArgumentCountError;
use ArrayAccess;
use Closure;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use JsonSerializable;
use Random\Randomizer;
use SortDirection;
use Traversable;
use WeakMap;

class Arr
{
    use Macroable;

    /**
     * Determine whether the given value is array accessible.
     *
     * @param  mixed  $value
     * @return bool
     */
    public static function accessible($value)
    {}

    /**
     * Determine whether the given value is arrayable.
     *
     * @param  mixed  $value
     * @return ($value is array
     *     ? true
     *     : ($value is \Illuminate\Contracts\Support\Arrayable
     *         ? true
     *         : ($value is \Traversable
     *             ? true
     *             : ($value is \Illuminate\Contracts\Support\Jsonable
     *                 ? true
     *                 : ($value is \JsonSerializable ? true : false)
     *             )
     *         )
     *     )
     * )
     */
    public static function arrayable($value)
    {}

    /**
     * Add an element to an array using "dot" notation if it doesn't exist.
     *
     * @param  array  $array
     * @param  string|int|float  $key
     * @param  mixed  $value
     * @return array
     */
    public static function add($array, $key, $value)
    {}

    /**
     * Get an array item from an array using "dot" notation.
     *
     * @throws \InvalidArgumentException
     */
    public static function array(ArrayAccess|array $array, string|int|null $key, ?array $default = null): array
    {}

    /**
     * Get a boolean item from an array using "dot" notation.
     *
     * @throws \InvalidArgumentException
     */
    public static function boolean(ArrayAccess|array $array, string|int|null $key, ?bool $default = null): bool
    {}

    /**
     * Collapse an array of arrays into a single array.
     *
     * @param  iterable  $array
     * @return array
     */
    public static function collapse($array)
    {}

    /**
     * Cross join the given arrays, returning all possible permutations.
     *
     * @template TValue
     *
     * @param  iterable<TValue>  ...$arrays
     * @return array<int, array<array-key, TValue>>
     */
    public static function crossJoin(...$arrays)
    {}

    /**
     * Divide an array into two arrays. One with keys and the other with values.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  array<TKey, TValue>  $array
     * @return array{TKey[], TValue[]}
     */
    public static function divide($array)
    {}

    /**
     * Flatten a multi-dimensional associative array with dots.
     *
     * @param  iterable  $array
     * @param  string  $prepend
     * @param  int  $depth
     * @return array
     */
    public static function dot($array, $prepend = '', $depth = INF)
    {}

    /**
     * Convert a flatten "dot" notation array into an expanded array.
     *
     * @param  iterable  $array
     * @return array
     */
    public static function undot($array)
    {}

    /**
     * Get all of the given array except for a specified array of keys.
     *
     * @param  array  $array
     * @param  array|string|int|float  $keys
     * @return array
     */
    public static function except($array, $keys)
    {}

    /**
     * Get all of the given array except for a specified array of values.
     *
     * @param  array  $array
     * @param  mixed  $values
     * @param  bool  $strict
     * @return array
     */
    public static function exceptValues($array, $values, $strict = false)
    {}

    /**
     * Determine if the given key exists in the provided array.
     *
     * @param  \ArrayAccess|array  $array
     * @param  string|int|float|null  $key
     * @return bool
     */
    public static function exists($array, $key)
    {}

    /**
     * Return the first element in an iterable passing a given truth test.
     *
     * @template TKey
     * @template TValue
     * @template TFirstDefault
     *
     * @param  iterable<TKey, TValue>  $array
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @param  TFirstDefault|(\Closure(): TFirstDefault)  $default
     * @return TValue|TFirstDefault
     */
    public static function first($array, ?callable $callback = null, $default = null)
    {}

    /**
     * Return the last element in an array passing a given truth test.
     *
     * @template TKey
     * @template TValue
     * @template TLastDefault
     *
     * @param  iterable<TKey, TValue>  $array
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @param  TLastDefault|(\Closure(): TLastDefault)  $default
     * @return TValue|TLastDefault
     */
    public static function last($array, ?callable $callback = null, $default = null)
    {}

    /**
     * Take the first or last {$limit} items from an array.
     *
     * @param  array  $array
     * @param  int  $limit
     * @return array
     */
    public static function take($array, $limit)
    {}

    /**
     * Flatten a multi-dimensional array into a single level.
     *
     * @param  iterable  $array
     * @param  int  $depth
     * @return array
     */
    public static function flatten($array, $depth = INF)
    {}

    /**
     * Get a float item from an array using "dot" notation.
     *
     * @throws \InvalidArgumentException
     */
    public static function float(ArrayAccess|array $array, string|int|null $key, ?float $default = null): float
    {}

    /**
     * Remove one or many array items from a given array using "dot" notation.
     *
     * @param  array  $array
     * @param  array|string|int|float  $keys
     * @return void
     */
    public static function forget(&$array, $keys)
    {}

    /**
     * Get the underlying array of items from the given argument.
     *
     * @template TKey of array-key = array-key
     * @template TValue = mixed
     *
     * @param  array<TKey, TValue>|Enumerable<TKey, TValue>|Arrayable<TKey, TValue>|WeakMap<object, TValue>|Traversable<TKey, TValue>|Jsonable|JsonSerializable|object  $items
     * @return ($items is WeakMap ? list<TValue> : array<TKey, TValue>)
     *
     * @throws \InvalidArgumentException
     */
    public static function from($items)
    {}

    /**
     * Get an item from an array using "dot" notation.
     *
     * @param  \ArrayAccess|array  $array
     * @param  string|int|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    public static function get($array, $key, $default = null)
    {}

    /**
     * Check if an item or items exist in an array using "dot" notation.
     *
     * @param  \ArrayAccess|array  $array
     * @param  string|array  $keys
     * @return bool
     */
    public static function has($array, $keys)
    {}

    /**
     * Determine if all keys exist in an array using "dot" notation.
     *
     * @param  \ArrayAccess|array  $array
     * @param  string|array  $keys
     * @return bool
     */
    public static function hasAll($array, $keys)
    {}

    /**
     * Determine if any of the keys exist in an array using "dot" notation.
     *
     * @param  \ArrayAccess|array  $array
     * @param  string|array|null  $keys
     * @return bool
     */
    public static function hasAny($array, $keys)
    {}

    /**
     * Determine if all items pass the given truth test.
     *
     * @param  iterable  $array
     * @param  (callable(mixed, array-key): bool)  $callback
     * @return bool
     */
    public static function every($array, callable $callback)
    {}

    /**
     * Determine if some items pass the given truth test.
     *
     * @param  iterable  $array
     * @param  (callable(mixed, array-key): bool)  $callback
     * @return bool
     */
    public static function some($array, callable $callback)
    {}

    /**
     * Get an integer item from an array using "dot" notation.
     *
     * @throws \InvalidArgumentException
     */
    public static function integer(ArrayAccess|array $array, string|int|null $key, ?int $default = null): int
    {}

    /**
     * Determines if an array is associative.
     *
     * An array is "associative" if it doesn't have sequential numerical keys beginning with zero.
     *
     * @param  array  $array
     * @return ($array is list ? false : true)
     */
    public static function isAssoc(array $array)
    {}

    /**
     * Determines if an array is a list.
     *
     * An array is a "list" if all array keys are sequential integers starting from 0 with no gaps in between.
     *
     * @param  array  $array
     * @return ($array is list ? true : false)
     */
    public static function isList($array)
    {}

    /**
     * Join all items using a string. The final items can use a separate glue string.
     *
     * @param  array  $array
     * @param  string  $glue
     * @param  string  $finalGlue
     * @return string
     */
    public static function join($array, $glue, $finalGlue = '')
    {}

    /**
     * Key an associative array by a field or using a callback.
     *
     * @param  iterable  $array
     * @param  callable|array|string  $keyBy
     * @return array
     */
    public static function keyBy($array, $keyBy)
    {}

    /**
     * Prepend the key names of an associative array.
     *
     * @template TValue
     *
     * @param  array<TValue>  $array
     * @param  string  $prependWith
     * @return array<string, TValue>
     */
    public static function prependKeysWith($array, $prependWith)
    {}

    /**
     * Get a subset of the items from the given array.
     *
     * @param  array  $array
     * @param  array|string  $keys
     * @return array
     */
    public static function only($array, $keys)
    {}

    /**
     * Get a subset of the items from the given array by value.
     *
     * @param  array  $array
     * @param  mixed  $values
     * @param  bool  $strict
     * @return array
     */
    public static function onlyValues($array, $values, $strict = false)
    {}

    /**
     * Select an array of values from an array.
     *
     * @param  array  $array
     * @param  array|string  $keys
     * @return array
     */
    public static function select($array, $keys)
    {}

    /**
     * Pluck an array of values from an array.
     *
     * @param  iterable  $array
     * @param  string|array|int|Closure|null  $value
     * @param  string|array|Closure|null  $key
     * @return array
     */
    public static function pluck($array, $value, $key = null)
    {}

    /**
     * Explode the "value" and "key" arguments passed to "pluck".
     *
     * @param  Closure|array|string  $value
     * @param  string|array|Closure|null  $key
     * @return array
     */
    protected static function explodePluckParameters($value, $key)
    {}

    /**
     * Run a map over each of the items in the array.
     *
     * @param  array  $array
     * @param  callable  $callback
     * @return array
     */
    public static function map(array $array, callable $callback)
    {}

    /**
     * Run an associative map over each of the items.
     *
     * The callback should return an associative array with a single key/value pair.
     *
     * @template TKey
     * @template TValue
     * @template TMapWithKeysKey of array-key
     * @template TMapWithKeysValue
     *
     * @param  array<TKey, TValue>  $array
     * @param  callable(TValue, TKey): array<TMapWithKeysKey, TMapWithKeysValue>  $callback
     * @return array
     */
    public static function mapWithKeys(array $array, callable $callback)
    {}

    /**
     * Run a map over each nested chunk of items.
     *
     * @template TKey
     * @template TValue
     *
     * @param  array<TKey, array>  $array
     * @param  callable(mixed...): TValue  $callback
     * @return array<TKey, TValue>
     */
    public static function mapSpread(array $array, callable $callback)
    {}

    /**
     * Push an item onto the beginning of an array.
     *
     * @param  array  $array
     * @param  mixed  $value
     * @param  mixed  $key
     * @return array
     */
    public static function prepend($array, $value, $key = null)
    { func_get_args(); }

    /**
     * Get a value from the array, and remove it.
     *
     * @param  array  $array
     * @param  string|int  $key
     * @param  mixed  $default
     * @return mixed
     */
    public static function pull(&$array, $key, $default = null)
    {}

    /**
     * Convert the array into a query string.
     *
     * @param  array  $array
     * @return string
     */
    public static function query($array)
    {}

    /**
     * Get one or a specified number of random values from an array.
     *
     * @param  array  $array
     * @param  int|null  $number
     * @param  bool  $preserveKeys
     * @return ($number is null ? mixed : array)
     *
     * @throws \InvalidArgumentException
     */
    public static function random($array, $number = null, $preserveKeys = false)
    {}

    /**
     * Set an array item to a given value using "dot" notation.
     *
     * If no key is given to the method, the entire array will be replaced.
     *
     * @param  array  $array
     * @param  string|int|null  $key
     * @param  mixed  $value
     * @return array
     */
    public static function set(&$array, $key, $value)
    {}

    /**
     * Push an item into an array using "dot" notation.
     *
     * @param  \ArrayAccess|array  $array
     * @param  string|int|null  $key
     * @param  mixed  $values
     * @return array
     */
    public static function push(ArrayAccess|array &$array, string|int|null $key, mixed ...$values): array
    {}

    /**
     * Shuffle the given array and return the result.
     *
     * @param  array  $array
     * @return array
     */
    public static function shuffle($array)
    {}

    /**
     * Get the first item in the array, but only if exactly one item exists. Otherwise, throw an exception.
     *
     * @param  array  $array
     * @param  (callable(mixed, array-key): array)|null  $callback
     *
     * @throws \Illuminate\Support\ItemNotFoundException
     * @throws \Illuminate\Support\MultipleItemsFoundException
     */
    public static function sole($array, ?callable $callback = null)
    {}

    /**
     * Sort the array using the given callback or "dot" notation.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  iterable<TKey, TValue>  $array
     * @param  callable|string|null|array<int, (callable(TValue, TValue): -1|0|1)|array{string, SortDirection|'asc'|'desc'}>  $callback
     * @return array<TKey, TValue>
     */
    public static function sort($array, $callback = null)
    {}

    /**
     * Sort the array in descending order using the given callback or "dot" notation.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  iterable<TKey, TValue>  $array
     * @param  callable|string|null|array<int, (callable(TValue, TValue): -1|0|1)|array{string, 'asc'|'desc'}>  $callback
     * @return array<TKey, TValue>
     */
    public static function sortDesc($array, $callback = null)
    {}

    /**
     * Recursively sort an array by keys and values.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  array<TKey, TValue>  $array
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @param  SortDirection|bool  $descending
     * @return array<TKey, TValue>
     */
    public static function sortRecursive($array, $options = SORT_REGULAR, $descending = false)
    {}

    /**
     * Recursively sort an array by keys and values in descending order.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  array<TKey, TValue>  $array
     * @param  int-mask-of<SORT_REGULAR|SORT_NUMERIC|SORT_STRING|SORT_LOCALE_STRING|SORT_NATURAL|SORT_FLAG_CASE>  $options
     * @return array<TKey, TValue>
     */
    public static function sortRecursiveDesc($array, $options = SORT_REGULAR)
    {}

    /**
     * Get a string item from an array using "dot" notation.
     *
     * @throws \InvalidArgumentException
     */
    public static function string(ArrayAccess|array $array, string|int|null $key, ?string $default = null): string
    {}

    /**
     * Conditionally compile classes from an array into a CSS class list.
     *
     * @param  array<string, bool>|array<int, string|int>|string  $array
     * @return ($array is array<string, false> ? '' : ($array is '' ? '' : ($array is array{} ? '' : non-empty-string)))
     */
    public static function toCssClasses($array)
    {}

    /**
     * Conditionally compile styles from an array into a style list.
     *
     * @param  array<string, bool>|array<int, string|int>|string  $array
     * @return ($array is array<string, false> ? '' : ($array is '' ? '' : ($array is array{} ? '' : non-empty-string)))
     */
    public static function toCssStyles($array)
    {}

    /**
     * Filter the array using the given callback.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  array<TKey, TValue>  $array
     * @param  callable(TValue, TKey): bool  $callback
     * @return array<TKey, TValue>
     */
    public static function where($array, callable $callback)
    {}

    /**
     * Filter the array using the negation of the given callback.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  array<TKey, TValue>  $array
     * @param  callable(TValue, TKey): bool  $callback
     * @return array<TKey, TValue>
     */
    public static function reject($array, callable $callback)
    {}

    /**
     * Partition the array into two arrays using the given callback.
     *
     * @template TKey of array-key
     * @template TValue of mixed
     *
     * @param  iterable<TKey, TValue>  $array
     * @param  callable(TValue, TKey): bool  $callback
     * @return array<int<0, 1>, array<TKey, TValue>>
     */
    public static function partition($array, callable $callback)
    {}

    /**
     * Filter items where the value is not null.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param  array<TKey, TValue|null>  $array
     * @return array<TKey, TValue>
     */
    public static function whereNotNull($array)
    {}

    /**
     * If the given value is not an array and not null, wrap it in one.
     *
     * @template TKey of array-key = array-key
     * @template TValue
     *
     * @param  array<TKey, TValue>|TValue|null  $value
     * @return ($value is null ? array{} : ($value is array ? array<TKey, TValue> : array{TValue}))
     */
    public static function wrap($value)
    {}
}
