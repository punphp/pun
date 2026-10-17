<?php // lint >= 8.5

namespace StaticCallPipe;

class Foo
{

	public static function doFoo(int $i, int $j): void
	{

	}

	public static function doBar(int $i): void
	{

	}

	public function doTest(): void
	{
		1 |> self::doFoo(...);

		1 |> self::doBar(...);

		'Hello' |> self::doBar(...);

		self::doFoo(1) |> self::doBar(...);

		$a = 1 |> self::doBar(...);
	}

}
