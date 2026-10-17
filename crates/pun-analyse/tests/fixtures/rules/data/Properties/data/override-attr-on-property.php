<?php // lint >= 8.5

namespace OverrideAttrOnProperty;

use Override;

class Foo
{

	#[Override]
	public int $foo;

	public function __construct(
		#[Override]
		public int $bar,
	) {}

}
