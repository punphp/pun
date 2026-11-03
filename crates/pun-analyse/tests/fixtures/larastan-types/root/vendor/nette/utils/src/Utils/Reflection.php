<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function ord;


/**
 * PHP reflection helpers.
 */
final class Reflection
{
	use Nette\StaticClass;

	/** @deprecated use Nette\Utils\Validators::isBuiltinType() */
	public static function isBuiltinType(string $type): bool
	{}


	#[\Deprecated('use Nette\Utils\Validators::isClassKeyword()')]
	public static function isClassKeyword(string $name): bool
	{}


	/**
	 * Returns the default value of a parameter. Resolves constants and class constants used as default values.
	 * @throws \ReflectionException if the constant cannot be resolved
	 */
	public static function getParameterDefaultValue(\ReflectionParameter $param): mixed
	{}


	/**
	 * Returns a reflection of a class or trait that contains a declaration of given property. Property can also be declared in the trait.
	 * @return \ReflectionClass<object>
	 */
	public static function getPropertyDeclaringClass(\ReflectionProperty $prop): \ReflectionClass
	{}


	/**
	 * Returns a reflection of a method that contains a declaration of $method.
	 * Usually, each method is its own declaration, but the body of the method can also be in the trait and under a different name.
	 */
	public static function getMethodDeclaringMethod(\ReflectionMethod $method): \ReflectionMethod
	{}


	/**
	 * Finds out if reflection has access to PHPdoc comments. Comments may not be available due to the opcode cache.
	 */
	public static function areCommentsAvailable(): bool
	{}


	/**
	 * Returns a human-readable string representation of a reflection object.
	 */
	public static function toString(\Reflector $ref): string
	{}


	/**
	 * Expands the name of the class to full name in the given context of given class.
	 * Thus, it returns how the PHP parser would understand $name if it were written in the body of the class $context.
	 * @param  \ReflectionClass<object>  $context
	 * @throws Nette\InvalidArgumentException
	 */
	public static function expandClassName(string $name, \ReflectionClass $context): string
	{}


	/**
	 * Returns the use statements from the file where the class is defined.
	 * @param  \ReflectionClass<object>  $class
	 * @return array<string, class-string>  Map of alias to fully qualified class name
	 */
	public static function getUseStatements(\ReflectionClass $class): array
	{}


	/**
	 * Parses PHP code to [class => [alias => class, ...]]
	 * @return array<string, array<string, string>>
	 */
	private static function parseUseStatements(string $code, ?string $forClass = null): array
	{}


	/**
	 * @param  \PhpToken[]  $tokens
	 * @param  string|int|int[]  $take
	 */
	private static function fetch(array &$tokens, string|int|array $take): ?string
	{}
}
