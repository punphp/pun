<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette;

use Nette\Utils\ObjectHelpers;


/**
 * Strict class for better experience.
 * - 'did you mean' hints
 * - access to undeclared members throws exceptions
 * - support for @property annotations
 * - support for calling event handlers stored in $onEvent via onEvent()
 */
trait SmartObject
{
	/**
	 * @param  mixed[]  $args
	 * @return mixed
	 * @throws MemberAccessException
	 */
	public function __call(string $name, array $args)
	{}


	/**
	 * @param  mixed[]  $args
	 * @return never
	 * @throws MemberAccessException
	 */
	public static function __callStatic(string $name, array $args)
	{}


	/**
	 * @return mixed
	 * @throws MemberAccessException if the property is not defined.
	 */
	public function &__get(string $name)
	{}


	/**
	 * @throws MemberAccessException if the property is not defined or is read-only
	 */
	public function __set(string $name, mixed $value): void
	{}


	/**
	 * @throws MemberAccessException
	 */
	public function __unset(string $name): void
	{}


	public function __isset(string $name): bool
	{}
}
