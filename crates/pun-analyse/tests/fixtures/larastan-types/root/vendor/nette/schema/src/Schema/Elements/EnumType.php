<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette;
use Nette\Schema\{Context, Kind};
use function is_int, is_string;


/**
 * A backed enum: accepts a case or its backing value and yields the case.
 */
final class EnumType extends Type
{
	public function __construct(
		/** @var class-string<\BackedEnum> */
		private string $enum,
	) {}


	public function doNormalize(mixed $value, Context $context): mixed
	{}


	public function describe(): array
	{}
}
