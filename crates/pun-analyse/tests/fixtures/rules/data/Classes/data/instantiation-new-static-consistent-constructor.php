<?php // lint >= 8.0

namespace InstantiationNewStaticConsistentConstructor;

/**
 * @phpstan-consistent-constructor
 */
class Foo
{

	public function __construct(int $i)
	{

	}

	public function doFoo(): void
	{
		$a = new static('s');
	}

}

class BaseClass {
	public function __construct(protected string $value) {
	}
}

/**
 * @phpstan-consistent-constructor
 */
class ChildClass2 extends BaseClass
{

}

class ChildClass3 extends ChildClass2 {
	public function fromInt(int $value): static {
		return new static($value);
	}
}
