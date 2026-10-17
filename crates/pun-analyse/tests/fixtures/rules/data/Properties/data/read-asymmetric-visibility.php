<?php // lint >= 8.4

namespace ReadAsymmetricVisibility;

class Foo
{

	public private(set) int $a;
	public protected(set) int $b;
	public public(set) int $c;

	public function doFoo(): void
	{
		echo $this->a;
		echo $this->b;
		echo $this->c;
	}

}

class Bar extends Foo
{

	public function doBar(): void
	{
		echo $this->a;
		echo $this->b;
		echo $this->c;
	}

}

function (Foo $foo): void {
	echo $foo->a;
	echo $foo->b;
	echo $foo->c;
};
