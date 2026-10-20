<?php // lint >= 8.5

namespace MethodCallWithoutSideEffectPipe;

class Foo
{

	/**
	 * @phpstan-pure
	 */
	public function doPure(int $i): int
	{
		return 5;
	}

	public function doMaybePure(int $i): int
	{
		return 5;
	}

	/**
	 * @phpstan-impure
	 */
	public function doImpure(int $i): int
	{
		echo '5';
		return $i;
	}

}

function (Foo $foo): void {
	5 |> $foo->doPure(...);
	5 |> $foo->doMaybePure(...);
	5 |> $foo->doImpure(...);
};
