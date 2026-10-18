<?php

namespace SignatureNullable;

interface A {}
interface B {}

function defaults(
	int $a = NULL,
	int $b = \null,
	null $c = null,
	A&B $d = null,
	(A&B)|null $e = null,
	(A&B)|int $f = null,
	int|string|NULL $g = null,
	array $i = null,
	?int $j = null,
	int $k = SOME_CONSTANT,
): void {}

function required(
	?int $a = null,
	int $b = 1,
	$c,
	int|NULL $d = null,
	$e,
	int|null $f = null,
	int $g = null,
	$h,
	MIXED $i = null,
	$j,
	mixed $k = null,
	$l,
	int $m = PHP_INT_MAX,
	$n,
): void {}

$f = function (int $a = null, $b = 1, $c) {};
$g = fn (int $a = null, $b = 1, $c) => 1;

new class {
	public function method(int $a = null, $b = 1, $c): void {}
};
