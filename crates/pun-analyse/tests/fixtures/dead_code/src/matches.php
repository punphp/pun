<?php

namespace DeadCode\Matches;

enum Suit
{
	case Hearts;
	case Spades;
	case Clubs;

	public function color(): string
	{
		return match ($this) {
			self::Hearts => 'Red',
			self::Spades => 'Black',
		};
	}

	public function same(): static
	{
		return $this;
	}

	public function viaStatic(): string
	{
		return match ($this->same()) {
			Suit::Hearts => 'Red',
		};
	}
}

function ints(int $i, bool $b, ?bool $n): void
{
	echo match ($i) {
		1 => 'a',
		2 => 'b',
	};
	echo match ($b) {
		true => 'a',
		false => 'b',
		'x' => 'c',
	};
	echo match ($n) {
		true => 'a',
		null => 'b',
	};
	echo match ($b) {
		true, false => 'a',
	};
	try {
		echo match ($i) {
			1 => 'a',
		};
	} catch (\UnhandledMatchError $e) {
	}
	echo match (true) {
		$b => 'a',
		default => 'b',
	};
}

enum Pair
{
	case One;
	case Two;

	public function name(): string
	{
		return match ($this) {
			self::One => 'one',
			self::Two => 'two',
			default => 'other',
		};
	}
}

enum Single
{
	case Only;
}

function single(Single $s): int
{
	return match ($s) {
		Single::Only => 1,
		default => 2,
	};
}

function nested(Pair $p, int $i): string
{
	return match ($p) {
		Pair::One => match ($p) {
			Pair::One => 'a',
		},
		default => '',
	};
}

/** @throws \UnhandledMatchError */
function throwing(int $i): string
{
	return match ($i) {
		1 => 'a',
	};
}

function shortArms(int $i): int
{
	return match (true) {true=>1};
}

function strings(string $s, array $a): void
{
	echo match ($s) {
		'a' => 1,
	};
	echo match ($a) {
		[] => 1,
		default => 2,
	};
}
