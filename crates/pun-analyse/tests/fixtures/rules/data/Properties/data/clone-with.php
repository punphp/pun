<?php // lint >= 8.5

namespace AccessPropertiesInAssignCloneWith;

class Foo
{

	private int $priv;

	protected int $prot;

	public int $pub;

	public function doFoo(): void
	{
		clone ($this, [
			'priv' => 1,
			'prot' => 1,
			'pub' => 1,
		]);
	}

}

function (Foo $foo): void {
	clone ($foo, [
		'priv' => 1,
		'prot' => 1,
		'pub' => 1,
	]);
};

class FooReadonly
{

	private readonly int $priv;

	protected readonly int $prot;

	public readonly int $pub;

	public public(set) readonly int $pubSet;

	public function doFoo(): void
	{
		clone ($this, [
			'priv' => 1,
			'prot' => 1,
			'pub' => 1,
		]);
	}

}

function (FooReadonly $foo): void {
	clone ($foo, [
		'priv' => 1,
		'prot' => 1,
		'pub' => 1,
		'pubSet' => 1,
	]);
};
