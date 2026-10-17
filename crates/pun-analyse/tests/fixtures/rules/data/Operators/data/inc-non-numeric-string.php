<?php

namespace IncDecNonNumericString;

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
		$a++;

		$b = 'a';
		$b++;

		$s++;
		$t++;
	}

}
