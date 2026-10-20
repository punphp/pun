<?php

namespace Bug11609;

class HelloWorld
{
	/** @param object{hello: string, world?: string} $a */
	public function sayHello(object $a, string $b): void
	{
		if ($a->hello !== null) {
			echo 'hello';
		}
		if ($a->world !== null) {

		}
	}
}
