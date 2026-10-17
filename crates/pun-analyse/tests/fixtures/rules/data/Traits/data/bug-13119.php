<?php // lint >= 8.3

namespace Bug13119;

trait Base {
	protected const array FOO = [];
}

class Foo
{
	use Base;
}

class Bar extends Foo
{
	protected const array FOO = [1, 2];
}
