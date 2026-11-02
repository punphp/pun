<?php declare(strict_types = 1);

namespace PureImpurePoints;

/** @phpstan-pure */
function evaluates(string $code): int
{
	eval($code);

	return 1;
}

/** @phpstan-pure */
function includes(): int
{
	include __DIR__ . '/missing-a.php';
	include_once __DIR__ . '/missing-b.php';
	require __DIR__ . '/missing-c.php';
	require_once __DIR__ . '/missing-d.php';

	return 1;
}

/**
 * @phpstan-pure
 * @return \Generator<int, int, mixed, void>
 */
function yields(): \Generator
{
	yield 1;
	yield 2 => 3;
	yield from [4];
}

final class Box
{

	public ?int $value = null;

	public static int $count = 0;

	/** @phpstan-pure */
	public function unsets(self $other): int
	{
		unset($other->value);

		return 1;
	}

	/** @phpstan-pure */
	public function prints(): int
	{
		return print 'x';
	}

	/** @phpstan-pure */
	public function dies(): int
	{
		die('x');
	}

	/** @phpstan-impure */
	public function throwsVoidPure(): int
	{
		return $this->pureThrowsVoid();
	}

	/**
	 * @phpstan-pure
	 * @throws void
	 */
	public function pureThrowsVoid(): int
	{
		return 1;
	}

	/** @phpstan-impure */
	public function throwsImplicitly(): int
	{
		return $this->pureImplicit();
	}

	/** @phpstan-pure */
	public function pureImplicit(): int
	{
		return 1;
	}

	/** @phpstan-impure */
	public function divides(int $a, int $b): int
	{
		return intdiv($a, 1) + $a % $b;
	}

	/** @phpstan-pure */
	public function readsStatic(): int
	{
		return self::$count + static::$count;
	}

	/** @phpstan-pure */
	public function incrementsStatic(): int
	{
		self::$count++;

		return 1;
	}

}

/** @phpstan-pure */
function echoTag(): void
{
	?><?= 'x' ?><?php
}

/** @phpstan-pure */
function closureByRefUse(): int
{
	$a = 1;
	$f = function () use (&$a): int {
		return $a;
	};

	return $f();
}

/** @phpstan-pure */
function arrayCallables(Box $box): int
{
	$cb = [$box, 'pureImplicit'];
	$cb();
	$cb = 'strlen';

	return $cb('x');
}

/** @phpstan-pure */
function concatToString(\Stringable $s): string
{
	return 'x' . $s;
}

/** @phpstan-impure */
function impureButPure(int $a): int
{
	return $a + 1;
}

/** @phpstan-impure */
function impureButThrows(int $a): int
{
	if ($a > 1) {
		throw new \RuntimeException();
	}

	return $a;
}
