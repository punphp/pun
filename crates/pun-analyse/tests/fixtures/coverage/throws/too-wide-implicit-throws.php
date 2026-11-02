<?php

namespace TooWideImplicitThrows;

use LogicException;

class Foo
{

	/**
	 * @throws LogicException
	 */
	public function doFoo(): void
	{
		throw new LogicException();
	}

}

final class Bar extends Foo
{

	public function doFoo(): void
	{

	}

}
