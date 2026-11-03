<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function is_array;


/**
 * Utilities for iterables.
 */
final class Iterables
{
	use Nette\StaticClass;

	/**
	 * Tests for the presence of value.
	 * @param  iterable<mixed>  $iterable
	 */
	public static function contains(iterable $iterable, mixed $value): bool
	{}


	/**
	 * Tests for the presence of key.
	 * @param  iterable<mixed>  $iterable
	 */
	public static function containsKey(iterable $iterable, mixed $key): bool
	{}


	/**
	 * Returns the first item (matching the predicate if given). If there is no such item, returns the result of $else or null.
	 * @template K
	 * @template V
	 * @template E
	 * @param  iterable<K, V>  $iterable
	 * @param  ?callable(V, K, iterable<K, V>): bool  $predicate
	 * @param  ?callable(): E  $else
	 * @return ($else is null ? ?V : V|E)
	 */
	public static function first(iterable $iterable, ?callable $predicate = null, ?callable $else = null): mixed
	{}


	/**
	 * Returns the key of the first item (matching the predicate if given). If there is no such item, returns the result of $else or null.
	 * @template K
	 * @template V
	 * @template E
	 * @param  iterable<K, V>  $iterable
	 * @param  ?callable(V, K, iterable<K, V>): bool  $predicate
	 * @param  ?callable(): E  $else
	 * @return ($else is null ? ?K : K|E)
	 */
	public static function firstKey(iterable $iterable, ?callable $predicate = null, ?callable $else = null): mixed
	{}


	/**
	 * Tests whether at least one element in the iterable passes the test implemented by the provided function.
	 * @template K
	 * @template V
	 * @param  iterable<K, V>  $iterable
	 * @param  callable(V, K, iterable<K, V>): bool  $predicate
	 */
	public static function some(iterable $iterable, callable $predicate): bool
	{}


	/**
	 * Tests whether all elements in the iterable pass the test implemented by the provided function.
	 * @template K
	 * @template V
	 * @param  iterable<K, V>  $iterable
	 * @param  callable(V, K, iterable<K, V>): bool  $predicate
	 */
	public static function every(iterable $iterable, callable $predicate): bool
	{}


	/**
	 * Returns a generator that yields only elements matching the given $predicate. Maintains original keys.
	 * @template K
	 * @template V
	 * @param  iterable<K, V>  $iterable
	 * @param  callable(V, K, iterable<K, V>): bool  $predicate
	 * @return \Generator<K, V>
	 */
	public static function filter(iterable $iterable, callable $predicate): \Generator
	{ yield; }


	/**
	 * Returns a generator that transforms values by calling $transformer. Maintains original keys.
	 * @template K
	 * @template V
	 * @template R
	 * @param  iterable<K, V>  $iterable
	 * @param  callable(V, K, iterable<K, V>): R  $transformer
	 * @return \Generator<K, R>
	 */
	public static function map(iterable $iterable, callable $transformer): \Generator
	{ yield; }


	/**
	 * Returns a generator that transforms keys and values by calling $transformer. If it returns null, the element is skipped.
	 * @template K
	 * @template V
	 * @template ResK
	 * @template ResV
	 * @param  iterable<K, V>  $iterable
	 * @param  callable(V, K, iterable<K, V>): ?array{ResK, ResV}  $transformer
	 * @return \Generator<ResK, ResV>
	 */
	public static function mapWithKeys(iterable $iterable, callable $transformer): \Generator
	{ yield; }


	/**
	 * Creates a repeatable iterator from a factory function.
	 * The factory is called every time the iterator is iterated.
	 * @template K
	 * @template V
	 * @param  callable(): iterable<K, V>  $factory
	 * @return \IteratorAggregate<K, V>
	 */
	public static function repeatable(callable $factory): \IteratorAggregate
	{}


	/**
	 * Wraps around iterator and caches its keys and values during iteration.
	 * This allows the data to be re-iterated multiple times.
	 * @template K
	 * @template V
	 * @param  iterable<K, V>  $iterable
	 * @return \IteratorAggregate<K, V>
	 */
	public static function memoize(iterable $iterable): \IteratorAggregate
	{}


	/**
	 * Creates an iterator from anything that is iterable.
	 * @template K
	 * @template V
	 * @param  iterable<K, V>  $iterable
	 * @return \Iterator<K, V>
	 */
	public static function toIterator(iterable $iterable): \Iterator
	{}
}
