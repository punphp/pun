<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use Random\Randomizer;
use function strlen;
use const PHP_VERSION_ID;


/**
 * Secure random string generator.
 */
final class Random
{
	use Nette\StaticClass;

	/**
	 * Generates a random string of given length from characters specified in second argument.
	 * Supports intervals, such as `0-9` or `A-Z`.
	 */
	public static function generate(int $length = 10, string $charlist = '0-9a-z'): string
	{}
}
