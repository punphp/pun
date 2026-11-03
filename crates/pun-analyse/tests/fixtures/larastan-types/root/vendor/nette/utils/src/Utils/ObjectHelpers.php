<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use Nette\MemberAccessException;
use function strlen;


/**
 * Nette\SmartObject helpers.
 * @internal
 */
final class ObjectHelpers
{
	use Nette\StaticClass;

	/**
	 * @return never
	 * @throws MemberAccessException
	 */
	public static function strictGet(string $class, string $name): void
	{}


	/**
	 * @return never
	 * @throws MemberAccessException
	 */
	public static function strictSet(string $class, string $name): void
	{}


	/**
	 * @return never
	 * @throws MemberAccessException
	 */
	public static function strictCall(string $class, string $method, array $additionalMethods = []): void
	{}


	/**
	 * @return never
	 * @throws MemberAccessException
	 */
	public static function strictStaticCall(string $class, string $method): void
	{}


	/**
	 * Returns array of magic properties defined by annotation @property.
	 * @return array of [name => bit mask]
	 * @internal
	 */
	public static function getMagicProperties(string $class): array
	{}


	/**
	 * Finds the best suggestion (for 8-bit encoding).
	 * @param  (\ReflectionFunctionAbstract|\ReflectionParameter|\ReflectionClass|\ReflectionProperty|string)[]  $possibilities
	 * @internal
	 */
	public static function getSuggestion(array $possibilities, string $value): ?string
	{}


	private static function parseFullDoc(\ReflectionClass $rc, string $pattern): array
	{}


	/**
	 * Checks if the public non-static property exists.
	 * Returns 'event' if the property exists and has event like name
	 * @internal
	 */
	public static function hasProperty(string $class, string $name): bool|string
	{}
}
