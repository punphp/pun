<?php

namespace StaticPropertyInternalTagOne {

	class Foo
	{
		/** @internal */
		public static $internal;

		public static $notInternal;
	}

	/**
	 * @internal
	 */
	class FooInternal
	{

		public static $foo;

	}

	function (Foo $foo): void {
		$foo::$internal;
		$foo::$notInternal;
	};

	function (FooInternal $foo): void {
		$foo::$foo;
	};

}

namespace StaticPropertyInternalTagOne\Test {

	function (\StaticPropertyInternalTagOne\Foo $foo): void {
		$foo::$internal;
		$foo::$notInternal;
	};

	function (\StaticPropertyInternalTagOne\FooInternal $foo): void {
		$foo::$foo;
	};
}

namespace StaticPropertyInternalTagTwo {

	function (\StaticPropertyInternalTagOne\Foo $foo): void {
		$foo::$internal;
		$foo::$notInternal;
	};

	function (\StaticPropertyInternalTagOne\FooInternal $foo): void {
		$foo::$foo;
	};

}

namespace {

	function (\StaticPropertyInternalTagOne\Foo $foo): void {
		$foo::$internal;
		$foo::$notInternal;
	};

	function (\StaticPropertyInternalTagOne\FooInternal $foo): void {
		$foo::$foo;
	};

	class FooWithInternalStaticPropertyWithoutNamespace
	{
		/** @internal */
		public static $internal;

		public static $notInternal;
	}

	/**
	 * @internal
	 */
	class FooInternalWithStaticPropertyWithoutNamespace
	{

		public static $foo;

	}

	function (FooWithInternalStaticPropertyWithoutNamespace $foo): void {
		$foo::$internal;
		$foo::$notInternal;
	};

	function (FooInternalWithStaticPropertyWithoutNamespace $foo): void {
		$foo::$foo;
	};

}

namespace SomeNamespace {

	function (\FooWithInternalStaticPropertyWithoutNamespace $foo): void {
		$foo::$internal;
		$foo::$notInternal;
	};

	function (\FooInternalWithStaticPropertyWithoutNamespace $foo): void {
		$foo::$foo;
	};

}
