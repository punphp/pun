<?php

namespace DeadCodeFixture;

function labels(): void
{
	unused:
	echo 1;
	used:
	echo 2;
	goto used;
}

class Toggles
{
	public function __construct(
		int $unused,
		private int $promoted,
	)
	{
		$a = 1;
		$f = function () use (
			$a
		): void {
		};
		$f();
	}
}
