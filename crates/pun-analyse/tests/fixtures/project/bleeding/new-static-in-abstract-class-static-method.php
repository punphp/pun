<?php

namespace NewStaticInAbstractClassStaticMethod;

class Foo
{

	public function doFoo(): void
	{
		new static();
	}

	public static function staticDoFoo(): void
	{
		new static();
	}

}

abstract class Bar
{

	public function doFoo(): void
	{
		new static();
	}

	public static function staticDoFoo(): void
	{
		new static();
	}

}
