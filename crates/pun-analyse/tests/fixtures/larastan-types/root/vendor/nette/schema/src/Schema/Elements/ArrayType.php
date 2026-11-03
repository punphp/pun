<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette\Schema\{Context, MergeMode, Schema};
use function is_array;


/**
 * An array, a list or an iterable, optionally with a schema for its items and keys.
 */
final class ArrayType extends Type
{
	public function __construct(
		string $type,
		/** a single value is accepted and wrapped into a one-item list */
		private readonly bool $wrap = false,
	) {}


	public function doNormalize(mixed $value, Context $context): mixed
	{}


	/**
	 * Sets the schema of items and optionally of keys.
	 */
	public function items(string|Schema $valueType = 'mixed', string|Schema|null $keyType = null): static
	{}


	/**
	 * Sets how the array is combined when merging multiple configuration layers.
	 */
	public function mergeMode(MergeMode $mode): static
	{}


	/** @deprecated  an array has no pattern */
	public function pattern(?string $pattern): static
	{}


	public function describe(): array
	{}
}
