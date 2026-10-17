<?php

namespace Bug12544;

trait Foo {
	public function hello(): string
	{
		return "Hello from Foo!";
	}
}

class Bar {
	use Foo {
		hello as private somethingElse;
	}
}

function (Bar $bar): void {
	$bar->hello();
	$bar->somethingElse();
};
