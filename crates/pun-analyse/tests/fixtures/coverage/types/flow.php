<?php

namespace CoverageTypesFlow;

const LIMIT = 0;

enum Suit
{
	case Hearts;
	case Spades;
}

function colonDeclare(): int
{
	declare(ticks=1):
		return 1;
	enddeclare;
}

function colonForeach(array $a): int
{
	foreach ($a as $v):
		return $v;
	endforeach;
}

function colonFor(int $n): int
{
	for ($i = 0; $i < $n; $i++):
		return $i;
	endfor;
}

function colonSwitch(int $i): int
{
	switch ($i):
		case 1:
			return 1;
		default:
			return 2;
	endswitch;
}

function dynamicBreak(int $n): int
{
	while (true) {
		break $n;
	}
}

function nsTrue(): int
{
	while (\true) {
	}
}

function nsFalse(): int
{
	while (\FALSE) {
	}
}

function intTrue(): int
{
	while (1) {
	}
}

function otherConstant(): int
{
	while (LIMIT) {
	}
}

function namedEnum(Suit $s): int
{
	switch ($s) {
		case Suit::Hearts:
			return 1;
		case Suit::Spades:
			return 2;
	}
}

function bools(bool $b): int
{
	switch ($b) {
		case true:
			return 1;
		case false:
			return 2;
	}
}

function ints(): int
{
	$i = rand(0, 1) ? 1 : 2;
	switch ($i) {
		case rand():
			return 3;
		case 1:
			return 1;
		case 2:
			return 2;
	}
}

function nulls(): int
{
	$i = rand(0, 1) ? 1 : null;
	switch ($i) {
		case null:
			return 2;
		case 1:
			return 1;
	}
}

function nonLiteral(float $f): int
{
	switch ($f) {
		case 1.5:
			return 1;
	}
}

function callbackExit(): int
{
	array_map(function () {
		exit;
	}, []);
}

function inMatch(int $i): int
{
	match ($i) {
		1 => fn () => exit,
		default => 2,
	};
}

/** @return \Generator<int, int, int, string> */
function gen(): \Generator
{
	yield 1;
}

/** @return \Generator<int, int, int, void> */
function genVoid(): \Generator
{
	yield 1;
}

/** @return \Generator<int, int, int, mixed> */
function genMixed(): \Generator
{
	yield 1;
}

function objectSwitch(\stdClass $o): int
{
	switch ($o) {
		case new \stdClass():
			return 1;
	}
}

function stringSwitch(string $s): int
{
	switch ($s) {
		case 'a':
			return 1;
	}
}

function parenthesized(): int
{
	while ((\false)) {
	}
}
