<?php // lint >= 8.5

namespace PipeOperator;

class Foo
{

	/**
	 * @param callable(string &): void $cb
	 */
	public function doFoo(callable $cb): void
	{
		$a = 'hello';
		$a |> $cb;

		$a |> self::doBar(...);
	}

	public static function doBar(string &$s): void
	{

	}

}
