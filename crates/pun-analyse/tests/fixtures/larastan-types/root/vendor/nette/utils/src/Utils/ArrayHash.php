<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function count, is_array, is_scalar, sprintf;


/**
 * Array-like object with property access.
 * @template T
 * @implements \IteratorAggregate<array-key, T>
 * @implements \ArrayAccess<array-key, T>
 */
class ArrayHash extends \stdClass implements \ArrayAccess, \Countable, \IteratorAggregate
{
	/**
	 * Transforms array to ArrayHash.
	 * @param  array<T>  $array
	 */
	public static function from(array $array, bool $recursive = true): static
	{}


	/**
	 * @return \Iterator<array-key, T>
	 */
	public function &getIterator(): \Iterator
	{ yield; }


	public function count(): int
	{}


	/**
	 * Replaces or appends an item.
	 * @param  array-key  $key
	 * @param  T  $value
	 */
	public function offsetSet($key, $value): void
	{}


	/**
	 * Returns an item.
	 * @param  array-key  $key
	 * @return T
	 */
	public function offsetGet($key): mixed
	{}


	/**
	 * Determines whether an item exists.
	 * @param  array-key  $key
	 */
	public function offsetExists($key): bool
	{}


	/**
	 * Removes the element from this list.
	 * @param  array-key  $key
	 */
	public function offsetUnset($key): void
	{}
}
