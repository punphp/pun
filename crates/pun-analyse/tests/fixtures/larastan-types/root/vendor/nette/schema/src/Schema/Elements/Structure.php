<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette;
use Nette\Schema\{Context, Helpers, Kind, MergeMode, Schema};
use function array_key_exists, is_array, is_object;


final class Structure implements Schema
{
	use Base;

	/** @var Schema[] */
	private array $items;

	/** for array|list */
	private ?Schema $otherItems = null;

	/** @var array{?int, ?int} */
	private array $range = [null, null];
	private bool $skipDefaults = false;
	private ?MergeMode $mergeMode = null;


	/** @param Schema[]  $shape */
	public function __construct(array $shape)
	{}


	/**
	 * Not supported for structures; always throws.
	 */
	public function default(mixed $value): self
	{}


	public function min(?int $min): self
	{}


	public function max(?int $max): self
	{}


	/**
	 * Allows extra keys not defined in the shape, validating their values against the given type.
	 */
	public function otherItems(string|Schema $type = 'mixed'): self
	{}


	/**
	 * When enabled, missing properties are not filled with their default values.
	 */
	public function skipDefaults(bool $state = true): self
	{}


	/**
	 * Sets how the structure is combined when merging multiple configuration layers.
	 */
	public function mergeMode(MergeMode $mode): self
	{}


	/**
	 * Creates a new structure by merging this shape with additional properties.
	 * @param  Schema[]|self  $shape
	 */
	public function extend(array|self $shape): self
	{}


	/** @return Schema[] */
	public function getShape(): array
	{}


	/********************* inspection ****************d*g**/


	/**
	 * @return array<string, mixed>
	 * @internal
	 */
	public function describe(): array
	{}


	/********************* processing ****************d*g**/


	public function normalize(mixed $value, Context $context): mixed
	{}


	public function merge(mixed $value, mixed $base): mixed
	{}


	public function complete(mixed $value, Context $context): mixed
	{}


	/** @param  array<mixed>  $value */
	private function validateItems(array &$value, Context $context): void
	{}


	public function completeDefault(Context $context): mixed
	{}
}
