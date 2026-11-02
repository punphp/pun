<?php

namespace NestedTooWideMethodParameterOutType;

class Foo
{

	/**
	 * @param array<mixed> $a
	 * @param-out array<array{int, bool}> $a
	 */
	public function doFoo(array &$a): void
	{
		$a = [
			[
				1,
				false,
			],
			[
				2,
				false,
			],
		];
	}

}
