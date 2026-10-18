<?php

namespace AssignRef;

class P
{
	protected readonly int $pr;

	public private(set) int $ps;

	protected int $plain;

	public int $open;

	public static int $st;

	public function __construct()
	{
		$this->pr = 1;
		$this->ps = 1;
	}
}

class Q extends P
{
	public function f(P $p): void
	{
		$a = &$p->pr;
		$b = &$p->ps;
		$c = &$p->plain;
		$d = &$this->ps;
	}
}

function (P $p): void {
	$a = &$p->pr;
	$b = &$p->ps;
	$c = &$p->plain;
	$d = &$p->open;
	$e = &$p::$st;
	$f = &$p->nope;
	$g = &$p->st;
};

function (P $p, string $name): void {
	$a = &$p->$name;
};
