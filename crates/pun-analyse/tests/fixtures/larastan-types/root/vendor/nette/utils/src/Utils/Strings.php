<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use JetBrains\PhpStorm\Language;
use Nette;
use function defined, extension_loaded, function_exists, in_array, is_array, is_int, is_object, is_string, strlen;
use const ENT_IGNORE, ENT_NOQUOTES;


/**
 * String tools library.
 */
class Strings
{
	use Nette\StaticClass;

	public const TrimCharacters = " \t\n\r\0\x0B\u{A0}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{200B}\u{2028}\u{3000}";

	#[\Deprecated('use Strings::TrimCharacters')]
	public const TRIM_CHARACTERS = self::TrimCharacters;


	/**
	 * @deprecated use Nette\Utils\Validators::isUnicode()
	 */
	public static function checkEncoding(string $s): bool
	{}


	/**
	 * Removes all invalid UTF-8 characters from a string.
	 */
	public static function fixEncoding(string $s): string
	{}


	/**
	 * Returns a specific character in UTF-8 from code point (number in range 0x0000..D7FF or 0xE000..10FFFF).
	 * @throws Nette\InvalidArgumentException if code point is not in valid range
	 */
	public static function chr(int $code): string
	{}


	/**
	 * Returns a code point of specific character in UTF-8 (number in range 0x0000..D7FF or 0xE000..10FFFF).
	 */
	public static function ord(string $c): int
	{}


	/**
	 * @deprecated use str_starts_with()
	 */
	public static function startsWith(string $haystack, string $needle): bool
	{}


	/**
	 * @deprecated use str_ends_with()
	 */
	public static function endsWith(string $haystack, string $needle): bool
	{}


	/**
	 * @deprecated use str_contains()
	 */
	public static function contains(string $haystack, string $needle): bool
	{}


	/**
	 * Returns a part of UTF-8 string specified by starting position and length. If start is negative,
	 * the returned string will start at the start'th character from the end of string.
	 */
	public static function substring(string $s, int $start, ?int $length = null): string
	{}


	/**
	 * Removes control characters, normalizes line breaks to `\n`, removes leading and trailing blank lines,
	 * trims end spaces on lines, normalizes UTF-8 to the normal form of NFC.
	 */
	public static function normalize(string $s): string
	{}


	/** @deprecated use Strings::unixNewLines() */
	public static function normalizeNewLines(string $s): string
	{}


	/**
	 * Converts line endings to \n used on Unix-like systems.
	 * Line endings are: \n, \r, \r\n, U+2028 line separator, U+2029 paragraph separator.
	 */
	public static function unixNewLines(string $s): string
	{}


	/**
	 * Converts line endings to platform-specific, i.e. \r\n on Windows and \n elsewhere.
	 * Line endings are: \n, \r, \r\n, U+2028 line separator, U+2029 paragraph separator.
	 */
	public static function platformNewLines(string $s): string
	{}


	/**
	 * Converts UTF-8 string to ASCII, ie removes diacritics etc.
	 */
	public static function toAscii(string $s): string
	{}


	/**
	 * Modifies the UTF-8 string to the form used in the URL, ie removes diacritics and replaces all characters
	 * except letters of the English alphabet and numbers with a hyphens.
	 */
	public static function webalize(string $s, ?string $charlist = null, bool $lower = true): string
	{}


	/**
	 * Truncates a UTF-8 string to given maximal length, while trying not to split whole words. Only if the string is truncated,
	 * an ellipsis (or something else set with third argument) is appended to the string.
	 */
	public static function truncate(string $s, int $maxLen, string $append = "\u{2026}"): string
	{}


	/**
	 * Indents a multiline text from the left. Second argument sets how many indentation chars should be used,
	 * while the indent itself is the third argument (*tab* by default).
	 */
	public static function indent(string $s, int $level = 1, string $chars = "\t"): string
	{}


	/**
	 * Converts all characters of UTF-8 string to lower case.
	 */
	public static function lower(string $s): string
	{}


	/**
	 * Converts the first character of a UTF-8 string to lower case and leaves the other characters unchanged.
	 */
	public static function firstLower(string $s): string
	{}


	/**
	 * Converts all characters of a UTF-8 string to upper case.
	 */
	public static function upper(string $s): string
	{}


	/**
	 * Converts the first character of a UTF-8 string to upper case and leaves the other characters unchanged.
	 */
	public static function firstUpper(string $s): string
	{}


	/**
	 * Converts the first character of every word of a UTF-8 string to upper case and the others to lower case.
	 */
	public static function capitalize(string $s): string
	{}


	/**
	 * Compares two UTF-8 strings or their parts, without taking character case into account. If length is null, whole strings are compared,
	 * if it is negative, the corresponding number of characters from the end of the strings is compared,
	 * otherwise the appropriate number of characters from the beginning is compared.
	 */
	public static function compare(string $left, string $right, ?int $length = null): bool
	{}


	/**
	 * Finds the common prefix of strings or returns empty string if the prefix was not found.
	 * @param  string[]  $strings
	 */
	public static function findPrefix(array $strings): string
	{}


	/**
	 * Returns number of characters (not bytes) in UTF-8 string.
	 * That is the number of Unicode code points which may differ from the number of graphemes.
	 */
	public static function length(string $s): int
	{}


	/**
	 * Removes all left and right side spaces (or the characters passed as second argument) from a UTF-8 encoded string.
	 */
	public static function trim(string $s, string $charlist = self::TrimCharacters): string
	{}


	/**
	 * Pads a UTF-8 string to given length by prepending the $pad string to the beginning.
	 * @param  non-empty-string  $pad
	 */
	public static function padLeft(string $s, int $length, string $pad = ' '): string
	{}


	/**
	 * Pads UTF-8 string to given length by appending the $pad string to the end.
	 * @param  non-empty-string  $pad
	 */
	public static function padRight(string $s, int $length, string $pad = ' '): string
	{}


	/**
	 * Reverses UTF-8 string.
	 */
	public static function reverse(string $s): string
	{}


	/**
	 * Returns part of $haystack before $nth occurence of $needle or returns null if the needle was not found.
	 * Negative value means searching from the end.
	 */
	public static function before(string $haystack, string $needle, int $nth = 1): ?string
	{}


	/**
	 * Returns part of $haystack after $nth occurence of $needle or returns null if the needle was not found.
	 * Negative value means searching from the end.
	 */
	public static function after(string $haystack, string $needle, int $nth = 1): ?string
	{}


	/**
	 * Returns position in characters of $nth occurence of $needle in $haystack or null if the $needle was not found.
	 * Negative value of `$nth` means searching from the end.
	 */
	public static function indexOf(string $haystack, string $needle, int $nth = 1): ?int
	{}


	/**
	 * Returns position in characters of $nth occurence of $needle in $haystack or null if the needle was not found.
	 */
	private static function pos(string $haystack, string $needle, int $nth = 1): ?int
	{}


	/**
	 * Splits the string by a regular expression. Expressions in parentheses will be captured and returned as well.
	 * @return list<string>
	 */
	public static function split(
		string $subject,
		#[Language('RegExp')]
		string $pattern,
		bool|int $captureOffset = false,
		bool $skipEmpty = false,
		int $limit = -1,
		bool $utf8 = false,
	): array
	{}


	/**
	 * Searches the string for the first match of the regular expression and returns
	 * an array with the found expression and individual subexpressions, or null.
	 * @return ?array<string>
	 */
	public static function match(
		string $subject,
		#[Language('RegExp')]
		string $pattern,
		bool|int $captureOffset = false,
		int $offset = 0,
		bool $unmatchedAsNull = false,
		bool $utf8 = false,
	): ?array
	{}


	/**
	 * Searches the string for all occurrences matching the regular expression and returns
	 * an array of arrays containing the found expression and each subexpression.
	 * @return ($lazy is true ? \Generator<int, array<string>> : list<array<string>>)
	 */
	public static function matchAll(
		string $subject,
		#[Language('RegExp')]
		string $pattern,
		bool|int $captureOffset = false,
		int $offset = 0,
		bool $unmatchedAsNull = false,
		bool $patternOrder = false,
		bool $utf8 = false,
		bool $lazy = false,
	): array|\Generator
	{}


	/**
	 * Replaces all matches of the regular expression $pattern, which can be a string or array in the form `pattern => replacement`.
	 * @param  string|array<string, string>  $pattern
	 */
	public static function replace(
		string $subject,
		#[Language('RegExp')]
		string|array $pattern,
		string|callable $replacement = '',
		int $limit = -1,
		bool $captureOffset = false,
		bool $unmatchedAsNull = false,
		bool $utf8 = false,
	): string
	{}


	/**
	 * @param  list<array<array{string, int}>>  $groups
	 * @return list<array<array{string, int}>>
	 */
	private static function bytesToChars(string $s, array $groups): array
	{}


	/**
	 * @param  callable-string  $func
	 * @param  list<mixed>  $args
	 * @internal
	 */
	public static function pcre(string $func, array $args): mixed
	{}
}
