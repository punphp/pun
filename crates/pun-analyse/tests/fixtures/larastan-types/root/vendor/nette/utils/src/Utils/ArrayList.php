<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function count, is_int;


/**
 * Generic list with integer indices.
 * @template T
 * @implements \IteratorAggregate<int, T>
 * @implements \ArrayAccess<int, T>
 */
class ArrayList implements \ArrayAccess, \Countable, \IteratorAggregate
{
	/** @var list<T> */
	private array $list = [];


	/**
	 * Transforms array to ArrayList.
	 * @param  list<T>  $array
	 */
	public static function from(array $array): static
	{}


	/**
	 * @return \Iterator<int, T>
	 */
	public function &getIterator(): \Iterator
	{ yield; }


	public function count(): int
	{}


	/**
	 * Replaces or appends an item.
	 * @param  ?int  $index
	 * @param  T  $value
	 * @throws Nette\OutOfRangeException
	 */
	public function offsetSet($index, $value): void
	{}


	/**
	 * Returns an item.
	 * @param  int  $index
	 * @return T
	 * @throws Nette\OutOfRangeException
	 */
	public function offsetGet($index): mixed
	{}


	/**
	 * Determines whether an item exists.
	 * @param  int  $index
	 */
	public function offsetExists($index): bool
	{}


	/**
	 * Removes the element at the specified position in this list.
	 * @param  int  $index
	 * @throws Nette\OutOfRangeException
	 */
	public function offsetUnset($index): void
	{}


	/**
	 * Prepends an item.
	 * @param  T  $value
	 */
	public function prepend(mixed $value): void
	{}
}
