<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function ini_get, strlen;
use const PHP_OS_FAMILY;


/**
 * Miscellaneous utilities.
 */
class Helpers
{
	public const IsWindows = PHP_OS_FAMILY === 'Windows';


	/**
	 * Executes a callback and returns the captured output as a string.
	 * @param  callable(): void  $func
	 */
	public static function capture(callable $func): string
	{}


	/**
	 * Returns the last occurred PHP error or an empty string if no error occurred. Unlike error_get_last(),
	 * it is not affected by the PHP directive html_errors and always returns text, not HTML.
	 */
	public static function getLastError(): string
	{}


	/**
	 * Converts false to null, does not change other values.
	 */
	public static function falseToNull(mixed $value): mixed
	{}


	/**
	 * Returns value clamped to the inclusive range of min and max.
	 * @return ($value is float ? float : ($min is float ? float : ($max is float ? float : int)))
	 */
	public static function clamp(int|float $value, int|float $min, int|float $max): int|float
	{}


	/**
	 * Finds the string from $possibilities most similar to $value using Levenshtein distance, or null if none is close enough.
	 * @param  string[]  $possibilities
	 */
	public static function getSuggestion(array $possibilities, string $value): ?string
	{}


	/**
	 * Compares two values in the same way that PHP does. Recognizes operators: >, >=, <, <=, =, ==, ===, !=, !==, <>
	 * @param  '>'|'>='|'<'|'<='|'='|'=='|'==='|'!='|'!=='|'<>'  $operator
	 */
	public static function compare(mixed $left, string $operator, mixed $right): bool
	{}


	/**
	 * Splits a class name into namespace and short class name.
	 * @return array{string, string}
	 */
	public static function splitClassName(string $name): array
	{}
}
