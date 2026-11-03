<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use function is_string;


/**
 * ReflectionMethod preserving the original class name.
 * @internal
 */
final class ReflectionMethod extends \ReflectionMethod
{
	/** @var \ReflectionClass<object> */
	private readonly \ReflectionClass $originalClass;


	/** @param  class-string|object  $objectOrMethod */
	public function __construct(object|string $objectOrMethod, ?string $method = null)
	{}


	/** @return \ReflectionClass<object> */
	public function getOriginalClass(): \ReflectionClass
	{}
}
