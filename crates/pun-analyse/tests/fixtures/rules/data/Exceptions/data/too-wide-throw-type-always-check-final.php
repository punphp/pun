<?php

namespace TooWideThrowTypeAlwaysCheckFinal;

use LogicException;
use RuntimeException;

final class Foo
{

	/**
	 * @throws LogicException|RuntimeException
	 */
	public function doFoo(): void
	{
		throw new LogicException();
	}

}

class Bar
{

	/**
	 * @throws LogicException|RuntimeException
	 */
	public function doFoo(): void
	{
		// first declaration
		throw new LogicException();
	}

}

class Baz extends Bar
{

	public function doFoo(): void
	{
		throw new LogicException();
	}

	/**
	 * @throws LogicException|RuntimeException
	 */
	private function doBar(): void
	{
		throw new LogicException();
	}

}
