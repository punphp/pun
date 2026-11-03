<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function array_key_exists, gettype, is_float, is_int, is_object, is_string, strlen;


/**
 * Validation utilities.
 */
class Validators
{
	use Nette\StaticClass;

	private const BuiltinTypes = [
		'string' => 1, 'int' => 1, 'float' => 1, 'bool' => 1, 'array' => 1, 'object' => 1,
		'callable' => 1, 'iterable' => 1, 'void' => 1, 'null' => 1, 'mixed' => 1, 'false' => 1,
		'never' => 1, 'true' => 1,
	];

	/** @var array<string, ?(callable(mixed): bool)> */
	protected static $validators = [
		// PHP types
		'array' => 'is_array',
		'bool' => 'is_bool',
		'boolean' => 'is_bool',
		'float' => 'is_float',
		'int' => 'is_int',
		'integer' => 'is_int',
		'null' => 'is_null',
		'object' => 'is_object',
		'resource' => 'is_resource',
		'scalar' => 'is_scalar',
		'string' => 'is_string',

		// pseudo-types
		'callable' => [self::class, 'isCallable'],
		'iterable' => 'is_iterable',
		'list' => [Arrays::class, 'isList'],
		'mixed' => [self::class, 'isMixed'],
		'none' => [self::class, 'isNone'],
		'number' => [self::class, 'isNumber'],
		'numeric' => [self::class, 'isNumeric'],
		'numericint' => [self::class, 'isNumericInt'],

		// string patterns
		'alnum' => 'ctype_alnum',
		'alpha' => 'ctype_alpha',
		'digit' => 'ctype_digit',
		'lower' => 'ctype_lower',
		'pattern' => null,
		'space' => 'ctype_space',
		'unicode' => [self::class, 'isUnicode'],
		'upper' => 'ctype_upper',
		'xdigit' => 'ctype_xdigit',

		// syntax validation
		'email' => [self::class, 'isEmail'],
		'identifier' => [self::class, 'isPhpIdentifier'],
		'uri' => [self::class, 'isUri'],
		'url' => [self::class, 'isUrl'],

		// environment validation
		'class' => 'class_exists',
		'interface' => 'interface_exists',
		'directory' => 'is_dir',
		'file' => 'is_file',
		'type' => [self::class, 'isType'],
	];

	/** @var array<string, callable(mixed): int> */
	protected static $counters = [
		'string' => 'strlen',
		'unicode' => [Strings::class, 'length'],
		'array' => 'count',
		'list' => 'count',
		'alnum' => 'strlen',
		'alpha' => 'strlen',
		'digit' => 'strlen',
		'lower' => 'strlen',
		'space' => 'strlen',
		'upper' => 'strlen',
		'xdigit' => 'strlen',
	];


	/**
	 * Verifies that the value is of expected types separated by pipe.
	 * @throws AssertionException
	 */
	public static function assert(mixed $value, string $expected, string $label = 'variable'): void
	{}


	/**
	 * Verifies that item $key in array exists and is of expected types separated by pipe.
	 * @param  mixed[]  $array
	 * @throws AssertionException
	 */
	public static function assertField(
		array $array,
		int|string $key,
		?string $expected = null,
		string $label = "item '%' in array",
	): void
	{}


	/**
	 * Verifies that the value is of expected types separated by pipe.
	 */
	public static function is(mixed $value, string $expected): bool
	{}


	/**
	 * Finds whether all values are of expected types separated by pipe.
	 * @param  iterable<mixed>  $values
	 */
	public static function everyIs(iterable $values, string $expected): bool
	{}


	/**
	 * Checks if the value is an integer or a float.
	 * @return ($value is int|float ? true : false)
	 */
	public static function isNumber(mixed $value): bool
	{}


	/**
	 * Checks if the value is an integer or a integer written in a string.
	 * @return ($value is non-empty-string ? bool : ($value is int ? true : false))
	 */
	public static function isNumericInt(mixed $value): bool
	{}


	/**
	 * Checks if the value is a number or a number written in a string.
	 * @return ($value is non-empty-string ? bool : ($value is int|float ? true : false))
	 */
	public static function isNumeric(mixed $value): bool
	{}


	/**
	 * Checks if the value is a syntactically correct callback.
	 */
	public static function isCallable(mixed $value): bool
	{}


	/**
	 * Checks if the value is a valid UTF-8 string.
	 */
	public static function isUnicode(mixed $value): bool
	{}


	/**
	 * Checks if the value is 0, '', false or null.
	 * @return ($value is 0|0.0|''|false|null ? true : false)
	 */
	public static function isNone(mixed $value): bool
	{}


	/** @internal */
	public static function isMixed(): bool
	{}


	/**
	 * Checks if a variable is a zero-based integer indexed array.
	 * @deprecated  use Nette\Utils\Arrays::isList
	 * @return ($value is list ? true : false)
	 */
	public static function isList(mixed $value): bool
	{}


	/**
	 * Checks if the value is in the given range [min, max], where the upper or lower limit can be omitted (null).
	 * Numbers, strings and DateTime objects can be compared.
	 * @param  array{int|float|string|\DateTimeInterface|null, int|float|string|\DateTimeInterface|null}  $range
	 */
	public static function isInRange(mixed $value, array $range): bool
	{}


	/**
	 * Checks if the value is a valid email address. It does not verify that the domain actually exists, only the syntax is verified.
	 */
	public static function isEmail(string $value): bool
	{}


	/**
	 * Checks if the value is a valid URL address.
	 */
	public static function isUrl(string $value): bool
	{}


	/**
	 * Checks if the value is a valid URI address, that is, actually a string beginning with a syntactically valid schema.
	 */
	public static function isUri(string $value): bool
	{}


	/**
	 * Checks whether the input is a class, interface or trait.
	 * @deprecated
	 */
	public static function isType(string $type): bool
	{}


	/**
	 * Checks whether the input is a valid PHP identifier.
	 */
	public static function isPhpIdentifier(string $value): bool
	{}


	/**
	 * Determines if type is PHP built-in type. Otherwise, it is the class name.
	 */
	public static function isBuiltinType(string $type): bool
	{}


	/**
	 * Determines if type is special class name self/parent/static.
	 */
	public static function isClassKeyword(string $name): bool
	{}


	/**
	 * Checks whether the given type declaration is syntactically valid.
	 */
	public static function isTypeDeclaration(string $type): bool
	{}
}
