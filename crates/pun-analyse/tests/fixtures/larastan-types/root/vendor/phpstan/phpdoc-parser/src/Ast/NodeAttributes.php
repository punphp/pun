<?php declare(strict_types = 1);

namespace PHPStan\PhpDocParser\Ast;

use function array_key_exists;

trait NodeAttributes
{

	/** @var array<string, mixed> */
	private array $attributes = [];

	/**
	 * @param mixed $value
	 */
	public function setAttribute(string $key, $value): void
	{}

	public function hasAttribute(string $key): bool
	{}

	/**
	 * @return mixed
	 */
	public function getAttribute(string $key)
	{}

}
