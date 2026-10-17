<?php // lint >= 8.5

namespace MethodCallPipe;

class Foo
{

	public function doFoo(int $i, int $j): void
	{

	}

	public function doBar(int $i): void
	{

	}

	public function doTest(): void
	{
		1 |> $this->doFoo(...);

		1 |> $this->doBar(...);

		'Hello' |> $this->doBar(...);

		$this->doFoo(1) |> $this->doBar(...);

		$a = 1 |> $this->doBar(...);
	}

}
