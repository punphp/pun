<?php

namespace SelfOutCoverage;

/**
 * @template T
 */
class Box
{
	/**
	 * @template NewT
	 * @param NewT $value
	 * @phpstan-self-out self<NewT>
	 */
	public function set($value): void
	{
	}

	/** @phpstan-self-out int */
	public function notAnObject(): void
	{
	}

	/** @phpstan-self-out self<int> */
	public static function onStatic(): void
	{
	}

	/** @phpstan-self-out int&string */
	public function unresolvable(): void
	{
	}

	/** @phpstan-self-out self<int&string> */
	public function nestedUnresolvable(): void
	{
	}

	/** @phpstan-self-out \stdClass */
	public function otherClass(): void
	{
	}

	/** @phpstan-self-out static */
	public function staticType(): void
	{
	}

	/** @phpstan-self-out self<int, string> */
	public function moreTypes(): void
	{
	}
}

class Plain
{
	/** @phpstan-self-out self<int> */
	public function notGeneric(): void
	{
	}
}

interface Shape
{
	/** @phpstan-self-out Shape|null */
	public function maybe(): void;
}
