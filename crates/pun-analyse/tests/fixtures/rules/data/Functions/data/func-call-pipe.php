<?php // lint >= 8.5

namespace FuncCallPipe;

function doFoo(int $i, int $j): void
{

}

function doBar(int $i): void
{

}

class Foo
{

	public function doFoo(): void
	{
		1 |> doFoo(...);

		1 |> doBar(...);

		'Hello' |> doBar(...);

		doFoo(1) |> doBar(...);

		$a = 1 |> doBar(...);
	}

}

class TestImplode
{

	public function doFoo(): void
	{
		implode('foo');

		'foo' |> implode(...);
	}

}
