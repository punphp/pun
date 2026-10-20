<?php

namespace CoverageNamedArguments;

function fixed(int $a, string $b = ''): void
{
}

function variadic(int $a, int ...$rest): void
{
}

function implicit(): void
{
	func_get_args();
}

#[\Attribute]
class Attr
{
	public function __construct(public int $value, public string $label = '')
	{
	}
}

class Target
{
	public function __construct(int $x, int $y)
	{
	}

	public function run(int $a, int $b): void
	{
	}

	public static function make(int $a): void
	{
	}
}

#[Attr(value: 1, nope: 2)]
#[Attr(label: 'x')]
function attributed(array $list): void
{
	fixed(a: 1, b: 'x');
	fixed(b: 'x');
	fixed(1, a: 2);
	fixed(a: 1, 2);
	fixed(...['a' => 1]);
	fixed(...['b' => 'x']);
	fixed(...[1, 'x']);
	fixed(a: 1, ...$list);
	fixed(...$list, b: 'x');
	fixed(...$list, ...$list);
	fixed(...$list, 1);
	fixed(1, 'x', 3, 4);
	variadic(1, 2, 3);
	variadic(a: 1, extra: 2, more: 3, extra: 4);
	variadic(...$list, rest: 1);
	implicit(1, 2);
	implicit(x: 1);
	array_merge(a: [1]);
	join('a', 'b', 'c');
	array_udiff([1]);
	ibase_wait_event();
	new Target(y: 1);
	new Target(x: 1, z: 2);
	new class(x: 1) {
		public function __construct(int $x, int $y = 0)
		{
		}
	};
	(new Target(1, 2))->run(b: 2);
	Target::make(b: 1);
}
