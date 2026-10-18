<?php

namespace AssignEdge;

class Plain
{
	public static $s;

	public function doFoo(?\stdClass $a, array $arr, \Closure $c): void
	{
		$a?->foo()->bar = 1;
		$x = &$a?->foo();
		$x = &$arr[];
		$x = &$arr[][0];
		$arr[$arr[]] = 1;
		$arr[] .= 'x';
		[$a?->x => $y] = [];
		[[$a?->x]] = [];
		list($arr[], [$arr[]]) = [];
		$c::$s = 1;
		($a?->b)::$s = 1;
		[$arr[0], [$arr->foo()]] = [];
		($a?->b)::foo()->x = 1;
		[...$arr[]] = [];
		$this++;
		--$this;
		$this[0] = 1;
		$this[] = 1;
		$this->prop = 1;
		$this[0] .= 'a';
		unset($this[0]);
		unset($this[][0]);
		unset($this);
		foreach ($arr as [$a?->x]) {
		}
		foreach ($arr as $arr[]) {
		}
		foreach ($arr as &$arr[][0]) {
		}
		$$a = 1;
		${'this'} = 1;
	}

	public function byRef(array $arr, Plain $p, $mixed, string $name, \PDOStatement $stmt, \Closure $c): void
	{
		takesRef($arr[]);
		takesRef(value: $arr[]);
		takesValue($arr[]);
		takesVariadic(1, $arr[], $arr[]);
		preg_match('/a/', 'a', $arr[]);
		sort($arr[]);
		$p->ref($arr[]);
		$p->value($arr[]);
		$this->ref($arr[]);
		self::staticRef($arr[]);
		static::staticRef($arr[]);
		Plain::staticRef($arr[]);
		new Plain($arr[]);
		new self($arr[]);
		unknownFunction($arr[]);
		$p->unknown($arr[]);
		takesRef($this[0]);
		takesRef($this);
		takesValue($this[0]);
		preg_match('/a/', 'a', $this[0]);
		(fn (&$r) => 1)($arr[]);
		$this->ref($this[0]);
		\AssignEdge\takesRef($arr[]);
		\AssignEdge\takesValue($arr[]);
		$c($arr[]);
		$p->{$name}($arr[]);
		$mixed->ref($arr[]);
		Plain::{$name}($arr[]);
		$p::staticRef($arr[]);
		$mixed::staticRef($arr[]);
		$stmt->bindParam(1, $arr[]);
		$stmt->bindValue(1, $arr[]);
	}

	public function __construct(&$r = null)
	{
	}

	public function ref(&$r): void
	{
	}

	public function value($r): void
	{
	}

	public static function staticRef(&$r): void
	{
	}
}

class Child extends Plain
{
	public function doBar(array $arr): void
	{
		parent::staticRef($arr[]);
	}
}

class Access implements \ArrayAccess
{
	public function doFoo(): void
	{
		$this = 1;
		$this[0] = 1;
		$fn = function () {
			$this = 2;
		};
	}

	public function offsetExists(mixed $offset): bool { return true; }
	public function offsetGet(mixed $offset): mixed { return null; }
	public function offsetSet(mixed $offset, mixed $value): void {}
	public function offsetUnset(mixed $offset): void {}
}

function takesRef(&$value): void
{
}

function takesValue($value): void
{
}

function takesVariadic($a, &...$rest): void
{
}

try {
	throw new \Exception();
} catch (\Exception $this) {
}

$this = 1;
