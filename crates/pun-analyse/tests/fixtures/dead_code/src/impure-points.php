<?php

namespace DeadCode\ImpurePoints;

function pure(int $i): int
{
	$a = $i;
	$a++;
	[$b, $c] = [$a, $a];
	foreach ([1] as $k => $v) {
		$b += $v + $k;
	}
	unset($c);
	if ($b > 1) {
		return $b;
	}
	return dependsOnPure($b);
}

function dependsOnPure(int $i): int
{
	return $i + 1;
}

function echoes(): int
{
	echo 'x';
	return 1;
}

function globals(): int
{
	global $g;
	return 1;
}

function statics(): int
{
	static $s = 0;
	return 1;
}

function inline(): int
{
	?>html<?php
	return 1;
}

function echoTag(): int
{
	?><?= 'x' ?><?php
	return 1;
}

function declaresClass(): int
{
	class Inner
	{
	}
	return 1;
}

function unsetsProperty(\stdClass $o): int
{
	unset($o->p);
	return 1;
}

function declares(): int
{
	function nested(): void
	{
		echo 'x';
	}
	return 1;
}

function callsEchoes(): int
{
	return echoes();
}

function callsUnknown(callable $c): int
{
	return $c();
}

/** @throws \Exception */
function throwsTag(): int
{
	return 1;
}

function callsThrowsTag(): int
{
	return throwsTag();
}

function byRef(int &$i): int
{
	return 1;
}

final class Box
{
	public function __construct(private int $value)
	{
	}

	public function get(int $offset = 0): int
	{
		return $this->value + $offset;
	}

	public function viaThis(): int
	{
		return $this->get();
	}

	public static function make(int $i = 0): int
	{
		return 1;
	}

	public static function viaStatic(): int
	{
		return self::make();
	}

	public function viaNew(): int
	{
		new Box(1);
		return 1;
	}

	/** @return $this */
	public function fluent(): self
	{
		return $this;
	}
}

class Open
{
	public function get(): int
	{
		return 1;
	}

	private function hidden(): int
	{
		return 1;
	}

	public function viaHidden(): int
	{
		$this->hidden();
		return 1;
	}

	public function viaOpen(): int
	{
		return $this->get();
	}
}

function (Box $box, Open $open, ?Box $maybe): void {
	pure(1);
	dependsOnPure(1);
	echoes();
	globals();
	statics();
	inline();
	echoTag();
	declaresClass();
	unsetsProperty(new \stdClass());
	declares();
	callsEchoes();
	callsUnknown('strlen');
	callsThrowsTag();
	byRef($i);
	$box->get();
	$box->viaThis();
	$box->viaNew();
	$box->fluent();
	$maybe?->get();
	Box::make();
	Box::viaStatic();
	new Box(1);
	$open->get();
	$open->viaHidden();
	$open->viaOpen();
	$fresh = new Open();
	$fresh->get();
	$reused = new Open();
	$reused = $open;
	$reused->get();
	5 |> pure(...);
	5 |> Box::make(...);
	5 |> $box->get(...);
	5 |> (fn ($x) => pure($x));
	5 |> pure(1);
	pure(...);
	$closure = function () use ($fresh) {
		$fresh->get();
	};
	$byRefUse = function () use (&$fresh) {
	};
	$listed = new Open();
	[$listed] = [$open];
	$listed->get();
	$iterated = new Open();
	foreach ([$open] as $iterated) {
	}
	$iterated->get();
	$referenced = new Open();
	$alias = &$referenced;
	$referenced->get();
};
