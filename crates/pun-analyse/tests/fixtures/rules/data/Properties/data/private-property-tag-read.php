<?php

namespace PrivatePropertyTagRead;

use AllowDynamicProperties;

/**
 * @property-read int $foo
 * @property-read int $bar
 */
#[AllowDynamicProperties]
class Foo
{

	private int $foo;

	public int $bar;

}

function (Foo $foo): void {
	$foo->foo = 1;
	$foo->bar = 2;
};
