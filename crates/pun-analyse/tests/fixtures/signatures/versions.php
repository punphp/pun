<?php

namespace SignatureVersions;

function unions(int|string $a, int|float $b): int|string {}

function returnUnion(int $a): int|string {}

function defaults($a = 1, $b, ?int $c = null, $d): void {}

class Holder
{
	public function method(int $a, int|string $b): void {}

	public function returns(): int|string {}
}

$f = function (int|string $a): int|string {};
$g = function (): int|string {};
$h = fn (int $a): int|string => $a;
