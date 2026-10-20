<?php

namespace NestedTooWidePropertyType;

class Foo
{

	/** @var array<array{int, bool}> */
	private array $a = [];

	public function doFoo(): void
	{
		$this->a = [[1, false]];
	}

}
