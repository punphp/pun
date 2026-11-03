<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema;

use Nette;
use Nette\Utils\Reflection;
use function count, is_array, is_float, is_int, is_object, is_scalar, is_string, strlen;


/**
 * @internal
 */
final class Helpers
{
	use Nette\StaticClass;

	public const PreventMerging = '_prevent_merging';


	/**
	 * Merges dataset. Left has higher priority than right one.
	 */
	public static function merge(mixed $value, mixed $base): mixed
	{}


	/**
	 * Returns the type of a property or parameter as a string, or null if not determinable.
	 */
	public static function getPropertyType(\ReflectionProperty|\ReflectionParameter $prop): ?string
	{}


	/**
	 * Returns an annotation value.
	 * @param  \ReflectionClass<object>|\ReflectionProperty  $ref
	 */
	public static function parseAnnotation(\ReflectionClass|\ReflectionProperty $ref, string $name): ?string
	{}


	/**
	 * Formats a value for use in error messages (e.g., 'hello', true, object stdClass).
	 */
	public static function formatValue(mixed $value): string
	{}


	/**
	 * Adds a TypeMismatch error to the context if the value does not match the expected type.
	 */
	public static function validateType(mixed $value, string $expected, Context $context): void
	{}


	/**
	 * Adds a range error to the context if the value (or its length for strings/arrays) is outside the given range.
	 * @param  array{int|float|null, int|float|null}  $range
	 */
	public static function validateRange(mixed $value, array $range, Context $context, string $types = ''): void
	{}


	/**
	 * Checks whether a value falls within the given [min, max] range (null means no bound).
	 * @param  array{int|float|null, int|float|null}  $range
	 */
	public static function isInRange(mixed $value, array $range): bool
	{}


	/**
	 * Adds a PatternMismatch error to the context if the value does not match the pattern.
	 */
	public static function validatePattern(string $value, string $pattern, Context $context): void
	{}


	/**
	 * Returns a closure that casts a value to the given type (built-in, backed enum, class with constructor, or plain class).
	 * A null stays null for an enum or a class, and for a built-in type only when written as ?type.
	 * @return \Closure(mixed, Context): mixed
	 */
	public static function getCastStrategy(string $type): \Closure
	{}
}
