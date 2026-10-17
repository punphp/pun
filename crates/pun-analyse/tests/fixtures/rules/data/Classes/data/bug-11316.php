<?php // lint >= 8.0

namespace Bug11316;

/** @phpstan-consistent-constructor */
class Model
{
	public static function create(): static
	{
		return new static();
	}
}

class ParentWithoutConstructor
{

}

/** @phpstan-consistent-constructor */
class ChildExtendingParentWithoutConstructor extends ParentWithoutConstructor
{

	public static function create(): static
	{
		return new static();
	}

}
