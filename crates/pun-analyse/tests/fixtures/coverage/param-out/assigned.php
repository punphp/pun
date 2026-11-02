<?php

namespace ParamOutCoverage;

/** @param-out int $p */
function narrowerOut(&$p): void
{
	$p = 'str';
}

function noParamOut(string &$p): void
{
	$p = 1;
}

/** @param-out string $p */
function widerOut(string|int &$p): void
{
	$p = 'ok';
}

/** @param-out int $p */
function inBranches(&$p, bool $b): void
{
	if ($b) {
		$p = 1;
	} else {
		$p = 'no';
	}
}

/**
 * @param list<int> $p
 * @param-out list<int> $p
 */
function appended(array &$p): void
{
	$p[] = 'str';
}

/** @param-out int $p */
function viaCall(&$p): void
{
	noParamOut($p);
}

function incremented(string &$p): void
{
	$p = 'a';
	$p++;
}

/** @throws \RuntimeException */
function mayThrow(): void
{
}

/** @param-out int $p */
function caught(&$p): void
{
	try {
		$p = 1;
		mayThrow();
	} catch (\RuntimeException $p) {
	}
}

function caughtByRef(int &$p): void
{
	try {
		mayThrow();
	} catch (\LogicException | \RuntimeException $p) {
		echo 1;
	}
}

/** @param-out int $p */
function foreachTarget(&$p, array $values): void
{
	$p = 0;
	foreach ($values as $p) {
		echo 1;
	}
}

/** @param-out int $p */
function unsetOffset(&$p): void
{
	$p = [1, 2];
	unset($p[0]);
}

function inClosure(string &$p): void
{
	$f = function () {
		$p = 1;
	};
}

class Holder
{
	/** @param-out string $p */
	public function method(&$p): void
	{
		$p = 5;
	}

	public static function staticMethod(int &$p): void
	{
		$p = 'x';
	}
}
