<?php // lint >= 8.5

namespace TypesAssignedToPropertyCloneWith;

class Foo
{

	public function __construct(
		public int $i,
		public string $j,
	)
	{}

	public function doFoo(): void
	{
		clone ($this, [
			'i' => 1,
			'j' => 'foo',
		]);

		clone ($this, [
			'i' => 'wrong',
			'j' => 2,
		]);
	}

}
