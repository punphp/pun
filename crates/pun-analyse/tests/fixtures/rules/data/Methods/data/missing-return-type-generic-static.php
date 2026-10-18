<?php

namespace MissingReturnTypeGenericStatic;

/**
 * @template T
 */
class Foo
{

	/** @return static<array> */
	public function doFoo()
	{

	}

}
