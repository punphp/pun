<?php

namespace DeadCode\Checks;

/** @phpstan-assert int $i */
function assertInt(mixed $i): void
{
}

/** @phpstan-assert !null $i */
function assertNotNull(mixed $i): void
{
}

/**
 * @phpstan-assert int $a
 * @phpstan-assert string $c
 */
function assertThree(mixed $a, mixed $b, mixed $c): void
{
}

/** @phpstan-assert-if-true int $i */
function isInt(mixed $i): bool
{
	return is_int($i);
}

class Checker
{
	/** @phpstan-assert string $s */
	public function assertString(mixed $s): void
	{
	}

	/** @phpstan-assert string $s */
	public static function staticString(mixed $s): void
	{
	}

	public function plain(mixed $s): void
	{
	}

	public function methods(int $i, string $s, mixed $m, int $j, int $k): void
	{
		$this->assertString($s);
		$this->assertString($i);
		$this->assertString($m);
		self::staticString($s);
		self::staticString($j);
		$this->plain($k);
	}
}

function functions(int $i, string $s, mixed $m, ?int $n, bool $b, mixed $o): void
{
	$x = is_int($i);
	$y = is_string($i);
	$z = strtolower($s);
	assert(true);
	assertInt($i);
	assertInt($s);
	assertInt($m);
	assertInt(i: $i);
	assertNotNull($i);
	assertNotNull($n);
	assertThree(1, 2, 'c');
	assertThree(1, 2, 3);
	assertThree($o, 2, 'c');
	if (is_int($i)) {
		echo 1;
	} elseif ($b) {
		echo 2;
	}
	if ($b) {
		echo 1;
	} elseif (is_int($i)) {
		echo 2;
	}
	if (isInt($i)) {
		echo 4;
	}
	echo $x, $y, $z;
}
