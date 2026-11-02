<?php declare(strict_types = 1);

namespace UnusedParametersExtra;

function noParameters(): int
{
	return 1;
}

function emptyBody(int $a): void
{
}

function promotedLooking(int $a, int $b): int
{
	$a = $b;
	return $a;
}

function flow(int $a): void
{
	$b = $a + 1;
	$b = 2;
	echo $b;
}

function viaFuncGetArgs(int $a): array
{
	return func_get_args();
}

function goto_(int $a): void
{
	goto end;
	end:
	echo 1;
}

/** @phpstan-assert-if-true int $a->foo */
function assertsProperty(object $a, int $b): bool
{
	return true;
}

trait T
{
	private function traitUnused(int $a): int
	{
		return 1;
	}
}

enum E
{
	case A;

	private function enumUnused(int $a): int
	{
		return 1;
	}

	public function publicUnused(int $a): int
	{
		return 1;
	}
}

abstract class Base
{
	abstract protected function abstractMethod(int $a): int;

	private function __magic(int $a): int
	{
		return 1;
	}

	private function noOp(int $a): void
	{
		// nothing
	}

	private static function staticFlow(int $a): void
	{
		$b = [$a];
		$b = [];
		echo count($b);
	}

	private function promotedNot(int ...$rest): int
	{
		return 1;
	}

	public function __construct(private int $promoted, int $plain, int $flow)
	{
		$x = $flow;
		$x = 1;
		echo $x;
	}
}

class UsesTrait
{
	use T;
}

function anonymous(): object
{
	return new class {
		private function anonymousUnused(int $a): int
		{
			return 1;
		}

		public function __construct(int $unused = 1)
		{
		}
	};
}

function closures(int $outer): \Closure
{
	return function (int $param) use ($outer): int {
		$x = $outer;
		$x = 2;
		return $x;
	};
}

function onlyALiteralReturned(int $a): int
{
	return 1;
}

class EmptyConstructor
{
	public function __construct(string $s)
	{
	}

	private function literal(int $a): int
	{
		return 2;
	}
}
