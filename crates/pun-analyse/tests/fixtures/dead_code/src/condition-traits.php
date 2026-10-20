<?php

namespace DeadCode\ConditionTraits;

trait FooTrait
{
	public function doFoo(): void
	{
		if ($this->doBar()) {
			echo 1;
		}
	}

	public function doFoo2(): void
	{
		$a = 1;
		if ($a) {
			echo 2;
		}
		if (!$a) {
			echo 3;
		}
		if ($a > 0) {
			echo 4;
		}
	}

	public function doFoo3(): void
	{
		if ($this->doBar3()) {
			echo 3;
		}
	}
}

class Foo
{
	use FooTrait;

	public function doBar(): \stdClass
	{
		return new \stdClass();
	}

	public function doBar2(): \stdClass
	{
		return new \stdClass();
	}

	public function doBar3(): true
	{
		return true;
	}
}

class FooAnother
{
	use FooTrait;

	public function doBar(): ?\stdClass
	{
		return null;
	}

	public function doBar2(): \stdClass
	{
		return new \stdClass();
	}

	public function doBar3(): false
	{
		return false;
	}
}

trait ChecksTrait
{
	public function check(int $i): void
	{
		if ($this->isInt($i)) {
			echo 1;
		}
		if ($this->isNumber($i)) {
			echo 2;
		}
	}
}

class IntChecker
{
	use ChecksTrait;

	/** @phpstan-assert-if-true int $v */
	public function isInt(mixed $v): bool
	{
		return is_int($v);
	}

	/** @phpstan-assert-if-true int $v */
	public function isNumber(mixed $v): bool
	{
		return is_int($v);
	}
}

class StringChecker
{
	use ChecksTrait;

	/** @phpstan-assert-if-true string $v */
	public function isInt(mixed $v): bool
	{
		return is_string($v);
	}

	/** @phpstan-assert-if-true int|float $v */
	public function isNumber(mixed $v): bool
	{
		return is_int($v) || is_float($v);
	}
}
