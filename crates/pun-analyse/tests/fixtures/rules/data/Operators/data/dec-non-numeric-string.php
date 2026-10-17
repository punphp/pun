<?php

namespace DecDecNonNumericString;

class Foo
{

	/**
	 * @param string $s
	 * @param numeric-string $t
	 */
	public function doFoo(
		string $s,
		string $t
	): void
	{
		$a = '1';
		$a--;

		$b = 'a';
		$b--;

		$s--;
		$t--;
	}

	public function doBar(bool $b): void
	{
		$null = null;
		$null--;

		$b++;
	}

	public function doBaz(bool $b): void
	{
		$null = null;
		$null++;

		$b--;
	}

}
