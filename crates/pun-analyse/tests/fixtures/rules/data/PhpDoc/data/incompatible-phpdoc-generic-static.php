<?php

namespace IncompatiblePhpDocGenericStatic;

/**
 * @template T
 */
class Foo
{

	/**
	 * @return static<int, string>
	 */
	public function doFoo()
	{

	}

}
