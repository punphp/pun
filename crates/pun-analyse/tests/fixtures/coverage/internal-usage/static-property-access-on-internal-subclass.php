<?php

namespace StaticPropertyAccessOnInternalSubclassOne {

	class Foo
	{
		public static $foo;
	}

	/**
	 * @internal
	 */
	class Bar extends Foo
	{

		public static $bar;

	}

}

namespace StaticPropertyAccessOnInternalSubclassTwo {

	use StaticPropertyAccessOnInternalSubclassOne\Bar;

	function (): void {
		Bar::$foo;
		Bar::$bar;
	};
}
