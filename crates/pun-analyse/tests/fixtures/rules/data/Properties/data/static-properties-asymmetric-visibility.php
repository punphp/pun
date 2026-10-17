<?php // lint >= 8.5

namespace StaticPropertiesAsymmetricVisibility;

class Foo
{

	public private(set) static int $foo = 1;

	public protected(set) static int $bar = 1;

	public static function doFoo(): void
	{
		self::$foo = 2;
		self::$bar = 2;
	}

}

class Bar extends Foo
{

	public static function doFoo(): void
	{
		self::$foo = 2;
		self::$bar = 2;
	}

}

function (): void {
	Foo::$foo = 2;
	Foo::$bar = 2;
};
