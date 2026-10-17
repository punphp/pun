<?php

namespace PrivatePropertyWithAllowedPropertyTagIsPublic;

use AllowDynamicProperties;

/**
 * @property int $foo
 */
#[AllowDynamicProperties]
class Foo
{

	private int $foo;

}

function (Foo $foo): void {
	echo $foo->foo;
};
