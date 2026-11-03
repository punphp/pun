<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Iterators;

use Nette;


/**
 * Enhanced caching iterator with first/last/counter tracking.
 *
 * @template TKey
 * @template TValue
 * @extends \CachingIterator<TKey, TValue, \Iterator<TKey, TValue>>
 * @property-read bool $first
 * @property-read bool $last
 * @property-read bool $empty
 * @property-read bool $odd
 * @property-read bool $even
 * @property-read int $counter
 * @property-read TKey $nextKey
 * @property-read TValue $nextValue
 */
class CachingIterator extends \CachingIterator implements \Countable
{
	use Nette\SmartObject;

	private int $counter = 0;


	/** @param  iterable<TKey, TValue>|\stdClass  $iterable */
	public function __construct(iterable|\stdClass $iterable)
	{}


	/**
	 * Is the current element the first one?
	 */
	public function isFirst(?int $gridWidth = null): bool
	{}


	/**
	 * Is the current element the last one?
	 */
	public function isLast(?int $gridWidth = null): bool
	{}


	public function isEmpty(): bool
	{}


	public function isOdd(): bool
	{}


	public function isEven(): bool
	{}


	public function getCounter(): int
	{}


	public function count(): int
	{}


	/**
	 * Forwards to the next element.
	 */
	public function next(): void
	{}


	/**
	 * Rewinds the Iterator.
	 */
	public function rewind(): void
	{}


	/** @return TKey */
	public function getNextKey(): mixed
	{}


	/** @return TValue */
	public function getNextValue(): mixed
	{}
}
