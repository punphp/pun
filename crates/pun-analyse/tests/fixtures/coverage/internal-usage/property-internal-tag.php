<?php

namespace PropertyInternalTagOne {

	class Foo
	{
		/** @internal */
		public $internal;

		public $notInternal;
	}

	/**
	 * @internal
	 */
	class FooInternal
	{

		public $foo;

	}

	function (Foo $foo): void {
		$foo->internal;
		$foo->notInternal;
	};

	function (FooInternal $foo): void {
		$foo->foo;
	};

}

namespace PropertyInternalTagOne\Test {

	function (\PropertyInternalTagOne\Foo $foo): void {
		$foo->internal;
		$foo->notInternal;
	};

	function (\PropertyInternalTagOne\FooInternal $foo): void {
		$foo->foo;
	};
}

namespace PropertyInternalTagTwo {

	function (\PropertyInternalTagOne\Foo $foo): void {
		$foo->internal;
		$foo->notInternal;
	};

	function (\PropertyInternalTagOne\FooInternal $foo): void {
		$foo->foo;
	};

}

namespace {

	function (\PropertyInternalTagOne\Foo $foo): void {
		$foo->internal;
		$foo->notInternal;
	};

	function (\PropertyInternalTagOne\FooInternal $foo): void {
		$foo->foo;
	};

	class FooWithInternalPropertyWithoutNamespace
	{
		/** @internal */
		public $internal;

		public $notInternal;
	}

	/**
	 * @internal
	 */
	class FooInternalWithPropertyWithoutNamespace
	{

		public $foo;

	}

	function (FooWithInternalPropertyWithoutNamespace $foo): void {
		$foo->internal;
		$foo->notInternal;
	};

	function (FooInternalWithPropertyWithoutNamespace $foo): void {
		$foo->foo;
	};

}

namespace SomeNamespace {

	function (\FooWithInternalPropertyWithoutNamespace $foo): void {
		$foo->internal;
		$foo->notInternal;
	};

	function (\FooInternalWithPropertyWithoutNamespace $foo): void {
		$foo->foo;
	};

}
