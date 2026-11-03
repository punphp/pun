<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette\Schema\Schema;


/**
 * An int or a float.
 */
final class NumberType extends Type
{
	/** @deprecated  a number has no pattern */
	public function pattern(?string $pattern): static
	{}


	/** @deprecated  a number has no items */
	public function items(string|Schema $valueType = 'mixed', string|Schema|null $keyType = null): static
	{}


	/** @deprecated  a number has no default merging */
	public function mergeDefaults(bool $state = true): static
	{}
}
