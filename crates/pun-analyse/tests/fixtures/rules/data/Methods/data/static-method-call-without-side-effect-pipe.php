<?php // lint >= 8.5

namespace StaticMethodCallWithoutSideEffectPipe;

class Foo
{

	/**
	 * @phpstan-pure
	 */
	public static function doPure(int $i): int
	{
		return 5;
	}

	public static function doMaybePure(int $i): int
	{
		return 5;
	}

	/**
	 * @phpstan-impure
	 */
	public static function doImpure(int $i): int
	{
		echo '5';
		return $i;
	}

}

function (): void {
	5 |> Foo::doPure(...);
	5 |> Foo::doMaybePure(...);
	5 |> Foo::doImpure(...);
};
