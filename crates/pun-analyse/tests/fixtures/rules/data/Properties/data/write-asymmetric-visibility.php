<?php // lint >= 8.4

namespace WriteAsymmetricVisibility;

class Foo
{

	public private(set) int $a;
	public protected(set) int $b;
	public public(set) int $c;

	public function doFoo(): void
	{
		$this->a = 1;
		$this->b = 1;
		$this->c = 1;
	}

}

class Bar extends Foo
{

	public function doBar(): void
	{
		$this->a = 1;
		$this->b = 1;
		$this->c = 1;
	}

}

function (Foo $foo): void {
	$foo->a = 1;
	$foo->b = 1;
	$foo->c = 1;
};

class ReadonlyProps
{

	public readonly int $a;

	protected readonly int $b;

	private readonly int $c;

	public function doFoo(): void
	{
		$this->a = 1;
		$this->b = 1;
		$this->c = 1;
	}

}

class ChildReadonlyProps extends ReadonlyProps
{

	public function doBar(): void
	{
		$this->a = 1;
		$this->b = 1;
		$this->c = 1;
	}

}

function (ReadonlyProps $foo): void {
	$foo->a = 1;
	$foo->b = 1;
	$foo->c = 1;
};

class ArrayProp
{

	public private(set) array $a = [];

}

function (ArrayProp $foo): void {
	$foo->a[] = 1;
};
