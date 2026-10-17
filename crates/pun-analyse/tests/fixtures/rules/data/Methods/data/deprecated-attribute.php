<?php

namespace MethodDeprecatedAttribute;

use Deprecated;

class HelloWorld
{
	#[Deprecated]
	public function sayHello(): void
	{
	}
}
