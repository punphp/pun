<?php

namespace InternalConstructorDefinition {
	class Foo
	{

		/**
		 * @internal
		 */
		public function __construct()
		{

		}

	}

	$a = new Foo();
}

namespace InternalConstructorUsage {
	$a = new \InternalConstructorDefinition\Foo();
}
