<?php

namespace StrictDynamic;

class Foo
{
	public static function staticMethod(): void
	{
	}

	public function instanceMethod(): void
	{
	}

	public function doFoo(self $other): void
	{
		$this->staticMethod();
		$this->instanceMethod();
		$other->staticMethod();
		$this?->staticMethod();
		$name = 'staticMethod';
		$this->$name();
		$c1 = $this->staticMethod(...);
		$c2 = $other->staticMethod(...);
		$c3 = $this->instanceMethod(...);
		$other->instanceMethod();
		self::staticMethod();
		(new Bar())->inherited();
		(new Bar())->staticMethod();
	}
}

class Bar extends Foo
{
	public static function inherited(): void
	{
	}

	public function doBar(): void
	{
		$this->staticMethod();
		$this->inherited();
		$fn = function () {
			$this->inherited();
		};
	}
}

trait DynTrait
{
	public static function fromTrait(): void
	{
	}

	public function useIt(): void
	{
		$this->fromTrait();
	}
}

class UsesDynTrait
{
	use DynTrait;
}

function outside(Foo $foo): void
{
	$foo->staticMethod();
	$foo->staticMethod(...);
	$x = rand(0, 1) ? new Foo() : new Bar();
	$x->staticMethod();
	$y = rand(0, 1) ? new Foo() : 1;
	$y->staticMethod();
	E::A->make();
}

function mixedObject($m): void
{
	$m->staticMethod();
}

class StaticThis
{
	public static function staticMethod(): void
	{
	}

	public static function fromStatic(): void
	{
		$this->staticMethod();
	}
}

enum E
{
	public static function make(): self
	{
		return self::A;
	}

	case A;

	public function doE(): void
	{
		$this->make();
	}
}
