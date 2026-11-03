<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;


/**
 * Floating-point numbers comparison.
 */
class Floats
{
	use Nette\StaticClass;

	private const Epsilon = 1e-10;


	public static function isZero(float $value): bool
	{}


	public static function isInteger(float $value): bool
	{}


	/**
	 * Compare two floats. If $a < $b it returns -1, if they are equal it returns 0 and if $a > $b it returns 1
	 * @throws \LogicException if one of parameters is NAN
	 */
	public static function compare(float $a, float $b): int
	{}


	/**
	 * Returns true if $a = $b
	 * @throws \LogicException if one of parameters is NAN
	 */
	public static function areEqual(float $a, float $b): bool
	{}


	/**
	 * Returns true if $a < $b
	 * @throws \LogicException if one of parameters is NAN
	 */
	public static function isLessThan(float $a, float $b): bool
	{}


	/**
	 * Returns true if $a <= $b
	 * @throws \LogicException if one of parameters is NAN
	 */
	public static function isLessThanOrEqualTo(float $a, float $b): bool
	{}


	/**
	 * Returns true if $a > $b
	 * @throws \LogicException if one of parameters is NAN
	 */
	public static function isGreaterThan(float $a, float $b): bool
	{}


	/**
	 * Returns true if $a >= $b
	 * @throws \LogicException if one of parameters is NAN
	 */
	public static function isGreaterThanOrEqualTo(float $a, float $b): bool
	{}
}
