<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use JetBrains\PhpStorm\Language;
use Nette;
use function array_key_exists, count, func_num_args, is_array, is_int, is_object;
use const PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY;


/**
 * Array manipulation utilities.
 */
class Arrays
{
	use Nette\StaticClass;

	/**
	 * Returns item from array. If it does not exist, it throws an exception, unless a default value is set.
	 * @template T
	 * @param  array<T>  $array
	 * @param  array-key|array-key[]  $key
	 * @param  ?T  $default
	 * @return ?T
	 * @throws Nette\InvalidArgumentException if item does not exist and default value is not provided
	 */
	public static function get(array $array, string|int|array $key, mixed $default = null): mixed
	{}


	/**
	 * Returns reference to array item. If the index does not exist, new one is created with value null.
	 * @template T
	 * @param  array<T>  $array
	 * @param  array-key|array-key[]  $key
	 * @return ?T
	 * @throws Nette\InvalidArgumentException if traversed item is not an array
	 */
	public static function &getRef(array &$array, string|int|array $key): mixed
	{}


	/**
	 * Recursively merges two arrays. Useful for merging tree structures. Behaves like the + operator:
	 * key/value pairs from the second array are added to the first, with the first array's values taking
	 * precedence on key collisions. Nested arrays are merged recursively instead of replaced.
	 * @template T1
	 * @template T2
	 * @param  array<T1>  $array1
	 * @param  array<T2>  $array2
	 * @return array<T1|T2|array<mixed>>
	 */
	public static function mergeTree(array $array1, array $array2): array
	{}


	/**
	 * Returns zero-indexed position of given array key. Returns null if key is not found.
	 * @param  array<mixed>  $array
	 */
	public static function getKeyOffset(array $array, string|int $key): ?int
	{}


	/**
	 * @param  array<mixed>  $array
	 * @deprecated  use  getKeyOffset()
	 */
	public static function searchKey(array $array, string|int $key): ?int
	{}


	/**
	 * Tests an array for the presence of value.
	 * @param  array<mixed>  $array
	 */
	public static function contains(array $array, mixed $value): bool
	{}


	/**
	 * Returns the first item (matching the predicate if given). If there is no such item, returns the result of $else or null.
	 * @template K of int|string
	 * @template V
	 * @template E
	 * @param  array<K, V>  $array
	 * @param  ?callable(V, K, array<K, V>): bool  $predicate
	 * @param  ?callable(): E  $else
	 * @return ($else is null ? ?V : V|E)
	 */
	public static function first(array $array, ?callable $predicate = null, ?callable $else = null): mixed
	{}


	/**
	 * Returns the last item (matching the predicate if given). If there is no such item, returns the result of $else or null.
	 * @template K of int|string
	 * @template V
	 * @template E
	 * @param  array<K, V>  $array
	 * @param  ?callable(V, K, array<K, V>): bool  $predicate
	 * @param  ?callable(): E  $else
	 * @return ($else is null ? ?V : V|E)
	 */
	public static function last(array $array, ?callable $predicate = null, ?callable $else = null): mixed
	{}


	/**
	 * Returns the key of first item (matching the specified predicate if given) or null if there is no such item.
	 * @template K of int|string
	 * @template V
	 * @param  array<K, V>  $array
	 * @param  ?callable(V, K, array<K, V>): bool  $predicate
	 * @return ?K
	 */
	public static function firstKey(array $array, ?callable $predicate = null): int|string|null
	{}


	/**
	 * Returns the key of last item (matching the specified predicate if given) or null if there is no such item.
	 * @template K of int|string
	 * @template V
	 * @param  array<K, V>  $array
	 * @param  ?callable(V, K, array<K, V>): bool  $predicate
	 * @return ?K
	 */
	public static function lastKey(array $array, ?callable $predicate = null): int|string|null
	{}


	/**
	 * Inserts the contents of the $inserted array into the $array immediately before the $key.
	 * If $key is null (or does not exist), it is inserted at the beginning.
	 * @param  array<mixed>  $array
	 * @param  array<mixed>  $inserted
	 */
	public static function insertBefore(array &$array, string|int|null $key, array $inserted): void
	{}


	/**
	 * Inserts the contents of the $inserted array into the $array immediately after the $key.
	 * If $key is null (or does not exist), it is inserted at the end.
	 * @param  array<mixed>  $array
	 * @param  array<mixed>  $inserted
	 */
	public static function insertAfter(array &$array, string|int|null $key, array $inserted): void
	{}


	/**
	 * Renames key in array.
	 * @param  array<mixed>  $array
	 */
	public static function renameKey(array &$array, string|int $oldKey, string|int $newKey): bool
	{}


	/**
	 * Returns only those array items, which matches a regular expression $pattern.
	 * @param  string[]  $array
	 * @return string[]
	 */
	public static function grep(
		array $array,
		#[Language('RegExp')]
		string $pattern,
		bool|int $invert = false,
	): array
	{}


	/**
	 * Transforms multidimensional array to flat array.
	 * @param  array<mixed>  $array
	 * @return array<mixed>
	 */
	public static function flatten(array $array, bool $preserveKeys = false): array
	{}


	/**
	 * Checks if the array is indexed in ascending order of numeric keys from zero, a.k.a list.
	 * @return ($value is list ? true : false)
	 */
	public static function isList(mixed $value): bool
	{}


	/**
	 * Transforms a flat array of rows into an associative tree using a path expression like 'field|field[]field->field=field'.
	 * @param  array<mixed>  $array
	 * @param  string|list<string>  $path
	 * @return array<mixed>|\stdClass
	 */
	public static function associate(array $array, string|array $path): array|\stdClass
	{}


	/**
	 * Converts array to associative: items with numeric keys are converted to keys, with $filling as their value.
	 * @param  array<mixed>  $array
	 * @return array<string, mixed>
	 */
	public static function normalize(array $array, mixed $filling = null): array
	{}


	/**
	 * Returns and removes the value of an item from an array. If it does not exist, it throws an exception,
	 * or returns $default, if provided.
	 * @template T
	 * @param  array<T>  $array
	 * @param  ?T  $default
	 * @return ?T
	 * @throws Nette\InvalidArgumentException if item does not exist and default value is not provided
	 */
	public static function pick(array &$array, string|int $key, mixed $default = null): mixed
	{}


	/**
	 * Tests whether at least one element in the array passes the test implemented by the provided function.
	 * @template K of int|string
	 * @template V
	 * @param  array<K, V>  $array
	 * @param  callable(V, K, array<K, V>): bool  $predicate
	 */
	public static function some(iterable $array, callable $predicate): bool
	{}


	/**
	 * Tests whether all elements in the array pass the test implemented by the provided function.
	 * @template K of int|string
	 * @template V
	 * @param  array<K, V>  $array
	 * @param  callable(V, K, array<K, V>): bool  $predicate
	 */
	public static function every(iterable $array, callable $predicate): bool
	{}


	/**
	 * Returns a new array containing all key-value pairs matching the given $predicate.
	 * @template K of int|string
	 * @template V
	 * @param  array<K, V>  $array
	 * @param  callable(V, K, array<K, V>): bool  $predicate
	 * @return array<K, V>
	 */
	public static function filter(array $array, callable $predicate): array
	{}


	/**
	 * Returns an array containing the original keys and results of applying the given transform function to each element.
	 * @template K of int|string
	 * @template V
	 * @template R
	 * @param  array<K, V>  $array
	 * @param  callable(V, K, array<K, V>): R  $transformer
	 * @return array<K, R>
	 */
	public static function map(iterable $array, callable $transformer): array
	{}


	/**
	 * Returns an array containing new keys and values generated by applying the given transform function to each element.
	 * If the function returns null, the element is skipped.
	 * @template K of int|string
	 * @template V
	 * @template ResK of int|string
	 * @template ResV
	 * @param  array<K, V>  $array
	 * @param  callable(V, K, array<K, V>): ?array{ResK, ResV}  $transformer
	 * @return array<ResK, ResV>
	 */
	public static function mapWithKeys(array $array, callable $transformer): array
	{}


	/**
	 * Invokes all callbacks and returns array of results.
	 * @param  iterable<callable>  $callbacks
	 * @return array<mixed>
	 */
	public static function invoke(iterable $callbacks, mixed ...$args): array
	{}


	/**
	 * Invokes method on every object in an array and returns array of results.
	 * @param  iterable<object>  $objects
	 * @return array<mixed>
	 */
	public static function invokeMethod(iterable $objects, string $method, mixed ...$args): array
	{}


	/**
	 * Copies the elements of the $array array to the $object object and then returns it.
	 * @template T of object
	 * @param  iterable<mixed>  $array
	 * @param  T  $object
	 * @return T
	 */
	public static function toObject(iterable $array, object $object): object
	{}


	/**
	 * Converts value to array key.
	 */
	public static function toKey(mixed $value): int|string
	{}


	/**
	 * Returns a copy of $array where every item is cast to string and wrapped with $prefix and $suffix.
	 * @param  string[]  $array
	 * @return string[]
	 */
	public static function wrap(array $array, string $prefix = '', string $suffix = ''): array
	{}
}
