<?php // lint >= 8.5

namespace CallCallablePipe;

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
		1 |> 'CallCallablePipe\\doFoo';

		1 |> 'CallCallablePipe\\doBar';

		'Hello' |> 'CallCallablePipe\\doBar';

		('CallCallablePipe\\doFoo')(1) |> 'CallCallablePipe\\doBar';

		$a = 1 |> 'CallCallablePipe\\doBar';
	}

}
