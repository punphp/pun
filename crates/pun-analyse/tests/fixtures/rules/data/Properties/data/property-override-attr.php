<?php // lint >= 8.5

namespace PropertyOverrideAttr;

class Foo
{

	public int $foo;

}

class Bar extends Foo
{

	#[\Override]
	public int $foo;

	#[\Override]
	public int $bar;

}

trait FooTrait
{

	#[\Override]
	public int $foo;

	#[\Override]
	public int $bar;

}

class Baz extends Foo
{

	use FooTrait;

}

class Lorem extends Foo
{

	public int $foo;

}

trait BarTrait
{

	public int $foo;

}

class Ipsum extends Foo
{

	use BarTrait;

}
