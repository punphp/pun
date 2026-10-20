<?php

namespace CoverageValues;

function throwing(int $i, \Exception $e, string $s): void
{
	if ($i > 1) {
		throw $e;
	}
	throw new UnknownThrowable();
}

function throwInt(int $i): void
{
	throw $i;
}

function cloning(int|string $x, \stdClass $o, string $s): void
{
	$a = clone $o;
	$b = clone $s;
	$c = clone ($s);
	$d = clone $x;
	$e = clone new UnknownCloned();
}

/** @param array<string, int> $named */
function iterating(string $s, array $named, int $i): void
{
	foreach ($s as $v) {
	}
	foreach (new UnknownIterated() as $v) {
	}
	foreach ($named as $v) {
	}
	$all = [...$named, ...$i];
	$legacy = array(...$named, ...$s);
	$plain = [1, 'a' => 2];
}

/** @param array<int, string> $list */
function keys(array $list, \stdClass $o, ?int $n, string $s): void
{
	echo $list[$o];
	echo $list[1];
	echo $list[$n];
	echo $s[0];
	$list[] = 'a';
}

/** @return \Generator<string, int, int, void> */
function generator(): \Generator
{
	yield 'a' => 1;
	yield 1;
	yield 'b' => 'c';
	yield;
	$sent = yield 'c' => 2;
	yield from 1;
	yield from voidReturning();
	yield from sendingString();
	$result = yield from voidReturning();
	$other = yield from [1];
}

/** @return \Generator<string, int, void, void> */
function voidReturning(): \Generator
{
	yield 'a' => 1;
	$sent = yield 'b' => 2;
}

/** @return \Generator<string, int, string, int> */
function sendingString(): \Generator
{
	yield 'a' => 1;
	return 1;
}

function arrayYielding(): array
{
	yield 1;
}

/** @return never */
function neverYielding()
{
	yield 1;
}

function untyped()
{
	yield 1;
}

/** @return int|\Generator */
function maybeGenerator()
{
	yield 1;
}

function anonymous(): void
{
	$f = function (): \Generator {
		yield 'a';
	};
	$g = fn (): \Generator => yield 1;
	$h = fn (): int => 1;
	$k = function () {
		yield 1;
	};
}

class Defaults
{
	public string $wrong = 1;
	public static string $staticWrong = 2;
	public $untyped = null;
	public ?int $nullable = null;
	/** @var list<int> */
	public array $doc = ['a'];
	public string $hooked = 1 {
		get => $this->hooked;
	}
	public int $noDefault;
}

yield 1;
