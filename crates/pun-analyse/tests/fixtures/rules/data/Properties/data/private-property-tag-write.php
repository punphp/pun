<?php

namespace PrivatePropertyTagWrite;

use AllowDynamicProperties;

/**
 * @property-write int $foo
 * @property-write int $bar
 */
#[AllowDynamicProperties]
class Foo
{

	private int $foo;

	public int $bar;

}

function (Foo $foo): void {
	echo $foo->foo;
	echo $foo->bar;
};
