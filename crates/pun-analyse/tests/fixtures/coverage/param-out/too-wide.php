<?php

namespace ParamOutTooWide;

function unusedNull(?string &$p): void
{
	$p = 'foo';
}

/** @param-out string|int|null $p */
function unusedMembers(&$p): void
{
	$p = 'foo';
}

function boolTrue(bool &$b): void
{
	$b = true;
}

/** @param-out bool $b */
function boolOut(&$b): void
{
	$b = false;
}

function unusedThroughReturn(int|string|null &$p): int
{
	if (rand(0, 1)) {
		$p = 1;
		return 1;
	}
	$p = 2;
	return 2;
}

/**
 * @param array<mixed> $a
 * @param-out array<array{int, bool}> $a
 */
function nested(array &$a): void
{
	$a = [[1, false], [2, false]];
}

/**
 * @param-out list<int|string> $a
 */
function nestedList(array &$a): void
{
	$a = [1, 2];
}

function variadic(string|null &...$refs): void
{
	foreach ($refs as $key => $value) {
		$refs[$key] = 'foo';
	}
}

class Foo
{
	public function publicMethod(?string &$p): void
	{
		$p = 'foo';
	}

	private function privateMethod(?string &$p): void
	{
		$p = 'foo';
	}

	final public function finalMethod(?string &$p): void
	{
		$p = 'foo';
	}

	public function usePrivate(): void
	{
		$x = null;
		$this->privateMethod($x);
	}
}
