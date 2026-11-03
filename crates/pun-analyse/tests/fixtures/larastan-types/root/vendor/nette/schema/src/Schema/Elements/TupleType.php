<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette;
use Nette\Schema\{Context, Kind, Message, Schema};
use function count;


/**
 * A fixed-size list where each position has its own schema; a later layer replaces the tuple wholesale.
 */
final class TupleType extends Type
{
	/** @var list<Schema> */
	private array $items;

	/** the rest elements */
	private ?Schema $otherItems = null;


	/** @param Schema[]  $shape */
	public function __construct(array $shape)
	{}


	/**
	 * Not supported for tuples; always throws.
	 */
	public function default(mixed $value): static
	{}


	/**
	 * Allows positions beyond the shape, validating them against the given type.
	 */
	public function otherItems(string|Schema $type = 'mixed'): self
	{}


	/**
	 * Appends positions to the tuple.
	 * @param  Schema[]|self  $shape
	 */
	public function extend(array|self $shape): self
	{}


	/** @return list<Schema> */
	public function getShape(): array
	{}


	public function describe(): array
	{}


	protected function mergeValues(mixed $value, mixed $base): mixed
	{}


	protected function normalizeItems(array $value, Context $context): array
	{}


	protected function validateItems(array &$value, Context $context): void
	{}
}
