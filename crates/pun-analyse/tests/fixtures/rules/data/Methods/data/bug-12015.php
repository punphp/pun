<?php

namespace Bug12015;

class HelloWorld
{

	/**
	 * @param-out int $ref
	 */
	public static function passRef(?int &$ref): void
	{
		$ref = 1;
	}
}

function test(): void
{
	HelloWorld::passRef($storeHere);
	echo $storeHere;
}
