<?php

namespace Versions;

trait T
{
	const FOO = 1;

	final public const BAR = 2;
}

class A
{
	use T;

	final public const BAZ = 3;

	public const int TYPED = 4;

	public function doFoo(): void
	{
		try {
		} catch (\Exception) {
		}
		try {
		} catch (\Exception $e) {
		}
		if (PHP_VERSION_ID >= 80000) {
			try {
			} catch (\Exception) {
			}
		}
		throw new \Exception();
	}

	public function doBar(?int $i): int
	{
		$f = strlen(...);
		$g = $this->doFoo(...);
		$h = self::doBaz(...);
		$u = undefined_fn(...);
		$fn = fn () => throw new \Exception();
		return $i ?? throw new \Exception();
	}

	public static function doBaz(): void
	{
		(throw new \Exception());
	}
}
