<?php

namespace ClassConstantAccessOnInternalSubclassOne {

	class Foo
	{
		public const FOO = 1;
	}

	/**
	 * @internal
	 */
	class Bar extends Foo
	{

		public const BAR = 1;

	}

}

namespace ClassConstantAccessOnInternalSubclassTwo {

	use ClassConstantAccessOnInternalSubclassOne\Bar;

	function (): void {
		Bar::FOO;
		Bar::BAR;
	};
}
