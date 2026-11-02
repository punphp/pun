<?php

namespace ByRefForeachExtra;

class E extends \Exception {}

/** @param array<int, int> $a */
function branches(array $a, bool $b): void
{
	if ($b) {
		foreach ($a as &$x) {
		}
	}
	$x = 1;
	foreach ($a as &$y) {
	}
	if ($b) {
		unset($y);
	}
	$y = 1;
	foreach ($a as $k => &$z) {
	}
	unset($z);
	$z = 1;
	$k = 2;
}

/** @param array<int, int> $a */
function forms(array $a): void
{
	foreach ($a as &$x) {
	}
	$x[] = 1;
	$x++;
	[$x] = [1];
	$x .= 'a';
	try {
		throw new E();
	} catch (E $x) {
	}
	preg_match('/a/', 'a', $x);
	$f = function () {
		$x = 1;
	};
	$g = fn () => $x = 2;
}

/** @param array<int, int> $a */
function terminating(array $a, bool $b): void
{
	if ($b) {
		foreach ($a as &$x) {
		}
		return;
	}
	$x = 1;
	foreach ($a as $v) {
		foreach ($a as &$y) {
		}
		break;
	}
	$y = 1;
	while ($b) {
		foreach ($a as &$w) {
		}
		continue;
	}
	$w = 1;
	switch ($b) {
		case true:
			foreach ($a as &$s) {
			}
			break;
		default:
			$s = 0;
	}
	$s = 1;
	do {
		foreach ($a as &$d) {
		}
	} while ($b);
	$d = 1;
	for ($i = 0; $i < 3; $i++) {
		$e = 1;
		foreach ($a as &$e) {
		}
	}
	$e = 2;
}

/** @param array<int, int> $a */
function tried(array $a): void
{
	try {
		foreach ($a as &$x) {
		}
	} finally {
		$x = 1;
	}
	$x = 2;
	foreach ($a as &$this_) {
	}
	unset($this_);
}

class C
{
	/** @param array<int, int> $a */
	public function m(array $a): void
	{
		foreach ($a as &$x) {
		}
		$x = 1;
	}
}

$top = [1];
foreach ($top as &$t) {
}
$t = 1;
