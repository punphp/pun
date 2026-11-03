<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function func_get_args, ini_get, is_array, is_object, is_string, sprintf;


/**
 * PHP callable tools.
 */
final class Callback
{
	use Nette\StaticClass;

	/**
	 * Invokes internal PHP function with own error handler.
	 * @param  callable-string  $function
	 * @param  list<mixed>  $args
	 * @param  callable(string, int): (bool|void|null)  $onError
	 */
	public static function invokeSafe(string $function, array $args, callable $onError): mixed
	{}


	/**
	 * Checks that $callable is a valid PHP callback and returns it. With $syntax set to true, only verifies
	 * the structural validity without checking whether the class or method actually exists.
	 * @return callable
	 * @throws Nette\InvalidArgumentException
	 */
	public static function check(mixed $callable, bool $syntax = false): mixed
	{}


	/**
	 * Converts PHP callback to textual form. Class or method may not exists.
	 */
	public static function toString(mixed $callable): string
	{}


	/**
	 * Returns reflection for method or function used in PHP callback.
	 * @param  callable  $callable  type check is escalated to ReflectionException
	 * @throws \ReflectionException  if callback is not valid
	 */
	public static function toReflection(mixed $callable): \ReflectionMethod|\ReflectionFunction
	{}


	/**
	 * Checks whether PHP callback is function or static method.
	 */
	public static function isStatic(callable $callable): bool
	{}


	/**
	 * Unwraps closure created by Closure::fromCallable().
	 * @return callable|array{object|class-string, string}|string
	 */
	public static function unwrap(\Closure $closure): callable|array|string
	{}
}
