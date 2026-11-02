<?php

namespace Extra;

const NAMESPACED = 'x', BAR_CONSTANT = false;
#[\Deprecated]
const NAMESPACED_TOO = 'x';

class Foo
{
	const BAR_CONSTANT = false;
}

/** @param array<int, int> $ints */
function f(bool $b, string $s, array $ints): void
{
	\define('EXTRA_LIST', $ints);
	define('EXTRA_LIST', [1, 2]);
	define('EXTRA_OBJECT', new \stdClass());
	define('EXTRA_OBJECT', new Foo());
	define('EXTRA_SHAPE', ['a' => 'x']);
	define('EXTRA_UNTYPED', false);
	define($b ? 'BAR_CONSTANT' : 'A_NON_EMPTY_STRING', false);
	define($b ? 'BAR_CONSTANT' : '', false);
	define($s, false);
	define('Extra\NAMESPACED', 'x');
	define('NAMESPACED', 'x');
	define('BAR_CONSTANT');
	define(value: false, constant_name: 'BAR_CONSTANT');
}
