<?php

namespace DeadCodeFixture;

/** doc */
#[\Deprecated]
trait NeverUsed
{
}

#[\Attribute]
trait UsedByClass
{
	private const TRAIT_CONST = 1;

	private int $fromTrait = 1;

	public function fromTrait(): int
	{
		return $this->helper() + self::LOCAL + $this->counter;
	}
}

trait UsedByTrait
{
}

trait Outer
{
	use UsedByTrait;
}

class UsesTrait
{
	use UsedByClass;

	private const LOCAL = 2;

	private int $counter = 0;

	private function helper(): int
	{
		return 1;
	}
}
