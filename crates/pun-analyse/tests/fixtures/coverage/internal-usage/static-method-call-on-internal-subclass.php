<?php

namespace StaticMethodCallOnInternalSubclassOne {

	class Foo
	{
		public static function doFoo(): void
		{

		}
	}

	/**
	 * @internal
	 */
	class Bar extends Foo
	{

		public static function doBar(): void
		{

		}

	}

}

namespace StaticMethodCallOnInternalSubclassTwo {

	use StaticMethodCallOnInternalSubclassOne\Bar;

	function (): void {
		Bar::doFoo();
		Bar::doBar();
	};
}
