<?php

namespace UndefinedVariablesExtra;

echo $argc;
echo $argv[0];
echo $undefinedAtTop;

if (rand(0, 1)) {
	$maybeAtTop = 1;
}
echo $maybeAtTop;

function cli(): void
{
	echo $argc;
	echo $argv;
}

function maybe(): void
{
	if (rand(0, 1)) {
		$maybe = 1;
	}
	echo $maybe;
	echo $undefined;
	echo compact('maybe', 'undefined', 'defined');
	$defined = 1;
	echo compact(['maybe', ['undefined']]);
}

function dynamic(string $name): void
{
	$known = 1;
	echo $$name;
	$which = rand(0, 1) ? 'known' : 'unknown';
	echo $$which;
}

function matchArms(int $value): int
{
	match ($value) {
		1 => $result = 1,
		default => $result = 2,
	};
	match ($value) {
		1 => $partial = 1,
		2 => $partial = 2,
	};
	return $result + $partial;
}

class Holder
{
	public Holder $next;

	public array $list = [];

	public static int $count = 0;

	public function call(mixed $value): void
	{
	}

	public static function callStatic(mixed $value): void
	{
	}
}

function nullsafe(?Holder $holder, Holder $plain): void
{
	$holder?->next->call($a = 1);
	echo $a;
	$holder?->next::callStatic($b = 1);
	echo $b;
	echo $holder?->next->{$c = 'list'};
	echo $c;
	echo $holder?->next->list[$d = 0];
	echo $d;
	echo $holder?->next::${$e = 'count'};
	echo $e;
	$plain->next->call($f = 1);
	echo $f;
}

function multiIsset(): void
{
	if (rand(0, 1)) {
		$key = 'a';
	}
	if (rand(0, 1)) {
		$map = ['a' => 1];
	}
	if (isset($key, $map[$key])) {
		echo $map[$key];
	}
	if (isset($map[$key])) {
		echo 1;
	}
	echo $map[$key] ?? 0;
	echo empty($map[$key]) ? 0 : 1;
}

/** @return array{x: int, y?: int} */
function shape(): array
{
	return ['x' => 1];
}

function extracted(): void
{
	if (rand(0, 1)) {
		$x = 0;
		$y = 0;
	}
	extract(shape());
	echo $x;
	echo $y;
	echo $z;
}

function extractedUnknown(array $values): void
{
	extract($values);
	echo $anything;
}

class Cases
{
	private const A = 1;
	private const B = 2;

	/** @return self::A|self::B */
	private function pick(): int
	{
		return 1;
	}

	public function exhaustive(): int
	{
		switch ($this->pick()) {
			case self::A:
				$value = 1;
				break;
			case self::B:
				$value = 2;
				break;
		}

		return $value;
	}

	public function partial(): int
	{
		switch ($this->pick()) {
			case self::A:
				$value = 1;
				break;
		}

		return $value;
	}
}

function truthyInt(): void
{
	$count = rand();
	if ($count) {
		$var = 1;
	}
	if ($count) {
		echo $var;
	}
	$negative = rand(-5, 5);
	if ($negative) {
		$other = 1;
	}
	if ($negative) {
		echo $other;
	}
}
