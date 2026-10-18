<?php

namespace Loops;

foreach ([1] as $v) {
	function inLoop(): void
	{
		break;
	}
	if (rand(0, 1)) {
		while (true) {
			break 2;
		}
	}
}

function loops(array $a): void
{
	foreach ($a as $v) {
		if (rand(0, 1)) {
			do {
				continue 3;
			} while (false);
		}
	}
	foreach ($a as $v) {
		$f = function () {
			foreach ([1] as $w) {
				break 2;
			}
		};
	}
	foreach ($a as $v) {
		switch ($v) {
			case 1:
				if (rand(0, 1)) {
					break 0;
				}
		}
	}
	foreach ($a as $v) {
		if (rand(0, 1)) {
			break 99999999999999999999;
		}
	}
}

function &byRef(?\stdClass $a)
{
	$f = function () use ($a) {
		return $a?->b;
	};
	$g = function &() use ($a) {
		return;
	};
	return $a?->foo();
}

class Types
{
	public function __construct(
		public ?mixed $promoted,
		private int|VOID $other,
		?never $plain,
	) {
	}

	public int $hooked {
		set (?mixed $value) {
		}
	}

	public function dnf((\Countable&\Traversable)|mixed $a, int $b): int|string|never
	{
	}
}

function scoped(): void
{
	global $a, $this, $$a;
	static $b = 1, $this;
}
