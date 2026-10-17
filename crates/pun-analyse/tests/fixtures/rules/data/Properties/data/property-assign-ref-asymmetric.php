<?php // lint >= 8.4

namespace PropertyAssignRefAsymmetric;

class Foo
{

	private(set) int $a;

	protected(set) int $b;

	public(set) int $c;

	public function doFoo()
	{
		$foo = &$this->a;
		$bar = &$this->b;
		$bar = &$this->c;
	}

}

class Bar extends Foo
{

	public function doBar(Foo $foo)
	{
		$foo = &$this->a;
		$bar = &$this->b;
		$bar = &$this->c;
	}

}

function (Foo $foo): void {
	$a = &$foo->a;
	$b = &$foo->b;
	$c = &$foo->c;
};
