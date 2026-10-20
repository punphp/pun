<?php

namespace Conditions;

/** @phpstan-assert-if-true int $v */
function isInt(mixed $v): bool
{
	return is_int($v);
}

function alwaysTrue(): true
{
	return true;
}

class Checks
{
	/** @phpstan-assert-if-true int $v */
	public function isInt(mixed $v): bool
	{
		return is_int($v);
	}

	/** @phpstan-assert-if-true int $v */
	public static function isIntStatic(mixed $v): bool
	{
		return is_int($v);
	}

	public function yes(): true
	{
		return true;
	}

	public static function staticYes(): true
	{
		return true;
	}

	public function ifTrue(): void
	{
		if (true) {
			echo 1;
		}
	}

	public function ifFalse(): void
	{
		if (false) {
			echo 1;
		}
	}

	public function elseifNotLast(bool $b, int $i): void
	{
		if ($b) {
			echo 1;
		} elseif (true) {
			echo 2;
		} elseif ($i) {
			echo 3;
		}
	}

	public function elseifLast(bool $b): void
	{
		if ($b) {
			echo 1;
		} elseif (true) {
			echo 2;
		}
	}

	public function elseifThrowingElse(bool $b): void
	{
		if ($b) {
			echo 1;
		} elseif (1) {
			echo 2;
		} else {
			throw new \Exception();
		}
	}

	public function elseifFalse(bool $b): void
	{
		if ($b) {
			echo 1;
		} elseif (0) {
			echo 2;
		} else {
			echo 3;
		}
	}

	public function notTrue(): void
	{
		if (!true) {
			echo 1;
		}
	}

	public function notFalseNotLast(bool $b, int $i): void
	{
		if ($b) {
			echo 1;
		} elseif (!false) {
			echo 2;
		} elseif ($i) {
			echo 3;
		}
	}

	public function notFalseLast(bool $b): void
	{
		if ($b) {
			echo 1;
		} elseif (!false) {
			echo 2;
		}
	}

	public function orNotLast(bool $b, bool $c, int $i): void
	{
		if ($b) {
			echo 1;
		} elseif ($c || true) {
			echo 2;
		} elseif ($i) {
			echo 3;
		}
	}

	public function orLast(bool $b, bool $c): void
	{
		if ($b) {
			echo 1;
		} elseif ($c || true) {
			echo 2;
		}
	}

	public function xorNotLast(bool $b, bool $c, int $i): void
	{
		if ($b) {
			echo 1;
		} elseif ($c xor true) {
			echo 2;
		} elseif ($i) {
			echo 3;
		}
	}

	public function xorLast(bool $b, bool $c): void
	{
		if ($b) {
			echo 1;
		} elseif ($c xor true) {
			echo 2;
		}
	}

	public function andNotLast(bool $b, int $i): void
	{
		if ($b) {
			echo 1;
		} elseif (1 && 1) {
			echo 2;
		} elseif ($i) {
			echo 3;
		}
	}

	public function andLast(bool $b): void
	{
		if ($b) {
			echo 1;
		} elseif (1 && 1) {
			echo 2;
		}
	}

	public function calls(int $i): void
	{
		if (alwaysTrue()) {
			echo 1;
		}
		if (is_int($i)) {
			echo 2;
		}
		if (isInt($i)) {
			echo 3;
		}
		if ($this->isInt($i)) {
			echo 4;
		}
		if (self::isIntStatic($i)) {
			echo 5;
		}
		if ($this->yes()) {
			echo 6;
		}
		if (self::staticYes()) {
			echo 7;
		}
		if (class_exists(Checks::class)) {
			echo 8;
		}
	}

	public function strictInArray(): void
	{
		if (in_array(1, [1], true)) {
			echo 1;
		}
	}

	public function looseInArray(): void
	{
		if (in_array(1, [1])) {
			echo 1;
		}
	}

	public function skipped(int $i, bool $b, mixed $m): void
	{
		if ($i == 1) {
			echo 0;
		}
		if ($i === 2) {
			echo 1;
		}
		if ($i != 3) {
			echo 2;
		}
		if ($i <> 4) {
			echo 3;
		}
		if ($i !== 5) {
			echo 4;
		}
		if ($i < 6) {
			echo 5;
		}
		if ($i <= 7) {
			echo 6;
		}
		if ($i > 8) {
			echo 7;
		}
		if ($i >= 9) {
			echo 8;
		}
		if (isset($m)) {
			echo 9;
		}
		if (empty($m)) {
			echo 10;
		}
		if ($m instanceof Checks) {
			echo 11;
		}
		if ($b ? $i : 0) {
			echo 12;
		}
	}

	public function lastBeforeThrow(bool $b, bool $c): void
	{
		if ($b) {
			echo 1;
		} elseif ($c && true) {
			echo 2;
		}
		throw new \Exception();
	}

	public function lastBeforeThrowWithoutElseif(bool $b): void
	{
		if ($b && true) {
			echo 1;
		}
		throw new \Exception();
	}

	public function operators(bool $a, bool $b, int $i): void
	{
		$x = true && $a;
		$x = $a && true;
		$x = $a && false;
		$x = $a || true;
		$x = $a || false;
		$x = (true and $a);
		$x = ($a or true);
		$x = (true xor $a);
		$x = ($a xor false);
		$x = !true;
		$x = !false;
		$x = $i ? 1 : 2;
		$x = 1 ? 1 : 2;
		$x = 0 ?: 2;
		$a && true;
		$a || false;
		echo $x;
	}

	public function shortCircuitAnd(bool $a): void
	{
		$x = false && $a;
		echo $x;
	}

	public function shortCircuitOr(bool $a): void
	{
		$x = true || $a;
		echo $x;
	}

	public function results(bool $a): void
	{
		$x = $a && !$a;
		echo $x;
	}

	public function whileFalse(): void
	{
		while (false) {
			echo 1;
		}
	}

	public function whileBreak(bool $b): void
	{
		while (true) {
			if ($b) {
				break;
			}
		}
	}

	public function whileReturn(bool $b): void
	{
		while (true) {
			if ($b) {
				return;
			}
		}
	}

	public function whileContinue(bool $b, array $a): void
	{
		while (true) {
			foreach ($a as $v) {
				continue 2;
			}
			if ($b) {
				continue;
			}
		}
	}

	public function whileForever(bool $b): void
	{
		while (true) {
			if ($b) {
				continue;
			}
		}
	}

	public function whileMaybe(bool $b): void
	{
		while ($b) {
			echo 1;
		}
	}

	public function doForever(): void
	{
		do {
			echo 1;
		} while (true);
	}

	public function doBreak(bool $b): void
	{
		do {
			if ($b) {
				break;
			}
		} while (true);
	}

	public function doContinue(bool $b): void
	{
		do {
			if ($b) {
				continue;
			}
		} while (true);
	}

	public function doContinueOuter(array $a): void
	{
		do {
			foreach ($a as $v) {
				continue 2;
			}
		} while (true);
	}

	public function doOnce(): void
	{
		do {
			echo 1;
		} while (false);
	}

	public function doOnceBreak(bool $b): void
	{
		do {
			if ($b) {
				break;
			}
		} while (false);
	}

	public function doMaybe(bool $b): void
	{
		do {
			echo 1;
		} while ($b);
	}

	public function generator(): \Generator
	{
		while (true) {
			yield 1;
		}
	}

	public function generatorDo(): \Generator
	{
		do {
			yield 1;
		} while (true);
	}

	public function nestedYield(): void
	{
		while (true) {
			$f = function () {
				yield 1;
			};
			$g = fn () => yield 2;
			$h = new class {
				public function a(): \Generator
				{
					yield 3;
				}
			};
		}
	}

	public function nullableMatch(?bool $b): void
	{
		match ($b) {
			true => 1,
			false => 2,
			null => 3,
		};
	}

	public function never(): never
	{
		while (true) {
			echo 1;
		}
	}

	/** @return never */
	public function neverDoc()
	{
		while (true) {
			echo 1;
		}
	}

	public function neverClosure(): void
	{
		$f = function (): never {
			while (true) {
				echo 1;
			}
		};
		$g = fn () => $f;
		$f();
	}

	public function switchList(bool $b, bool $c): void
	{
		switch ($b) {
			case true:
				if ($c && true) {
					echo 1;
				}
				throw new \Exception();
		}
	}

	public function tryLists(bool $b): void
	{
		try {
			if ($b && true) {
				echo 2;
			}
			throw new \Exception();
		} catch (\Exception $e) {
			if ($b && true) {
				echo 3;
			}
			throw new \Exception();
		} finally {
			echo 4;
		}
	}

	public function finallyList(bool $b): void
	{
		try {
			echo 1;
		} finally {
			if ($b && true) {
				echo 4;
			}
			throw new \Exception();
		}
	}

	public function forList(bool $b): void
	{
		for ($i = 0; $i < 1; $i++) {
			if ($b && true) {
				echo 1;
			}
			throw new \Exception();
		}
	}

	public function foreachList(bool $b, array $a): void
	{
		foreach ($a as $v) {
			if ($b && true) {
				echo 2;
			}
			throw new \Exception();
		}
	}

	public function whileList(bool $b, bool $c): void
	{
		while ($b) {
			if ($c && true) {
				echo 3;
			}
			throw new \Exception();
		}
	}

	public function doList(bool $b, bool $c): void
	{
		do {
			if ($c && true) {
				echo 4;
			}
			throw new \Exception();
		} while ($b);
	}

	public function ifLists(bool $b, bool $c, bool $d): void
	{
		if ($b) {
			if ($d && true) {
				echo 1;
			}
			throw new \Exception();
		} elseif ($c) {
			if ($d && true) {
				echo 2;
			}
			throw new \Exception();
		} else {
			if ($d && true) {
				echo 3;
			}
			throw new \Exception();
		}
	}
}

trait ConditionTrait
{
	public function inTrait(bool $b): void
	{
		if ($b) {
			echo 1;
		}
	}
}

class UsesConditionTrait
{
	use ConditionTrait;
}

function topLevel(bool $b): void
{
	if ($b && true) {
		echo 1;
	}
	throw new \Exception();
}

if (PHP_VERSION_ID > 0 && true) {
	echo 1;
}
throw new \Exception();
