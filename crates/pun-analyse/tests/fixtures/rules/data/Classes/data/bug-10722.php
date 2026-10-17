<?php // lint >= 8.0

namespace Bug10722;

class BaseClass {
	public function __construct(protected string $value) {
	}
}

/**
 * @phpstan-consistent-constructor
 */
class ChildClass extends BaseClass {
	public function fromString(string $value): static {
		return new static($value);
	}
}

/**
 * @phpstan-consistent-constructor
 */
class ChildClass2 extends BaseClass {

}

class ChildClass3 extends ChildClass2 {
	public function fromString(string $value): static {
		return new static($value);
	}
}
