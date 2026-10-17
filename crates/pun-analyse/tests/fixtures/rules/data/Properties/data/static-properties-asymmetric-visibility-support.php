<?php // lint >= 8.5

namespace StaticPropertiesAsymmetricVisibilitySupport;

class Foo
{

	public private(set) static int $foo = 1;

	public protected(set) static int $bar = 1;

	public public(set) static int $baz = 1;

}
