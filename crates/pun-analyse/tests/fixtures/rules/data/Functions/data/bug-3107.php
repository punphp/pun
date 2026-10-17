<?php

namespace Bug3107;

class Holder
{
	/** @var string */
	public $val;
}

/** @param mixed $mixed */
function test($mixed): void
{
	$holder = new Holder();
	$holder->val = $mixed;

	$a = [];
	$a[$holder->val] = 1;
	take($a);
}

/** @param array<string, int> $a */
function take($a): void {}
