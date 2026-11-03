<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette;
use Nette\Schema\{Context, Helpers, Kind, Schema};
use function is_array;


/**
 * Schema that accepts any of a fixed set of values or sub-schemas (union type / enumeration).
 */
final class AnyOf implements Schema
{
	use Base;

	/** @var mixed[] */
	private array $set;
	private bool $dynamic = false;


	public function __construct(mixed ...$set)
	{}


	/**
	 * Sets the first variant as the default value (instead of null).
	 */
	public function firstIsDefault(): self
	{}


	/**
	 * Allows null as an accepted value in addition to the defined variants.
	 */
	public function nullable(): self
	{}


	/**
	 * Allows the value to be a DynamicParameter as an accepted variant.
	 */
	public function dynamic(): self
	{}


	/********************* inspection ****************d*g**/


	/**
	 * Scalar variants are reported as 'values', schema variants as 'variants'; scalars alone make an Enum,
	 * anything with a schema variant a Union.
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


	private function findAlternative(mixed $value, Context $context): mixed
	{}


	public function completeDefault(Context $context): mixed
	{}
}
