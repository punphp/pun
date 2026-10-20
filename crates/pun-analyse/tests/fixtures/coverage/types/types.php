<?php

namespace CoverageTypes;

enum Suit: string
{
	case Hearts = 'h';
	case Spades = 's';
}

function target(int $i): int
{
	return $i;
}

/**
 * @template T
 * @param T $t
 */
function template($t): void
{
	return $t;
}

/** @param int<min, 5> $i */
function toFive(int $i): void
{
	return $i;
}

/** @param int<1, 5> $i */
function oneToFive(int $i): void
{
	return $i;
}

/** @param literal-string $s */
function literal(string $s): void
{
	return $s;
}

/** @param lowercase-string $s */
function lower(string $s): void
{
	return $s;
}

/** @param uppercase-string $s */
function upper(string $s): void
{
	return $s;
}

/** @param callable-string $s */
function callableString(string $s): void
{
	return $s;
}

function enumFrom(string $s): void
{
	return Suit::from($s);
}

/** @param object{a: int, b?: string} $o */
function shape(object $o): void
{
	return $o;
}

function firstClass(): void
{
	return target(...);
}

/** @param array{a: int, b?: string} $a */
function optionalKey(array $a): void
{
	return $a;
}

/** @param list{int, 1?: string} $l */
function optionalElement(array $l): void
{
	return $l;
}

/** @param non-empty-list<int> $l */
function nonEmptyList(array $l): void
{
	return $l;
}

function returnsNull(): null
{
	return rand(0, 1) ? null : 1;
}

function gmp(\GMP $g): bool
{
	return $g == [];
}

function bcmath(\BcMath\Number $n): bool
{
	return $n == [] || $n < 1.5;
}

function deadCode(): void
{
	return;
	$a = [] == 1;
	echo [];
	preg_match($a, '');
}

/**
 * @template T of array
 * @param T $t
 */
function echoTemplate($t): void
{
	echo $t;
}

function casts(array $a): void
{
	echo (int) $a, (float) $a, (string) $a, (bool) $a, -$a;
}

function voidMatches(int $i): void
{
	$a = match ($i) {
		1 => target(1),
		default => throw new \Exception(),
	};
	$b = match ($i) {
		1 => match ($i) {
			default => target(1),
		},
		default => $i,
	};
	$c = match ($i) {
		1 => exit(),
		default => $i,
	};
}

function regexes(string $s): void
{
	preg_match(subject: $s, pattern: '/(/');
	preg_replace_callback_array([1 => fn () => '', '/(/' => fn () => ''], $s);
	preg_replace(['a' => '/(/'], '', $s);
	preg_replace([rand() => $s], '', $s);
}

function colonIf(int $i): int
{
	if ($i === 1):
		return 1;
	elseif ($i === 2):
		return 2;
	endif;
	return 3;
}

function nested(): int
{
	class Inner
	{
		public function f(): string
		{
			return 'a';
		}
	}
	interface InnerInterface
	{
	}
	trait InnerTrait
	{
	}
	enum InnerEnum
	{
	}
	return 1;
}

/**
 * @template T
 * @return T
 */
function templateMissing()
{
}

interface WithHooks
{
	public string $name { get; }
}

class Hooked
{
	public int $count {
		set => $value;
	}

	public int $other {
		get => 'a';
	}
}

trait Inner2
{
	public function inner2(): int
	{
		return 1;
	}
}

trait Outer
{
	use Inner2;
}

class UsesNested
{
	use Outer;
}

function closureVariable(): void
{
	$f = function (int $i): int {
		return $i;
	};
	return $f;
}

class Keys
{
	const A = 'a';
}

/** @param array{Keys::A: int} $a */
function constantKey(array $a): void
{
	return $a;
}

function knownList(int $i): void
{
	$l = [1, 2];
	$l[] = $i;
	return $l;
}

function bcmathInt(\BcMath\Number $n): bool
{
	return $n < 1;
}

function colonIfMissing(int $i): int
{
	if ($i === 1):
		return 1;
	elseif ($i === 2):
		return 2;
	endif;
}

/**
 * @template T
 * @return \Generator<int, int, int, T>
 */
function genTemplate(): \Generator
{
	yield 1;
}

/** @param list<int> $xs */
function growingList(array $xs): void
{
	$l = [1, 2];
	foreach ($xs as $x) {
		$l[] = $x;
	}
	return $l;
}
