<?php

namespace ArrayFilterAllow;

use function array_filter;

class Foo
{

	/**
	 * @param array<self|null> $a
	 */
	public function doFoo(
		array $a
	): array
	{
		return array_filter($a);
	}

	/**
	 * @param array<self> $a
	 */
	public function doFoo2(
		array $a
	): array
	{
		return array_filter($a);
	}

	/**
	 * @param array<int|null> $a
	 */
	public function doFoo3(
		array $a
	): array
	{
		return array_filter($a);
	}

	/** @param array<bool> $a */
	public function doFoo4(array $a): array
	{
		return array_filter($a);
	}

	/** @param array<int> $a */
	public function doFoo5(array $a): array
	{
		return array_filter($a);
	}

	/** @param array<array<int>> $a */
	public function doFoo6(array $a): array
	{
		return array_filter($a);
	}

}
-----
27: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
37: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
49: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
