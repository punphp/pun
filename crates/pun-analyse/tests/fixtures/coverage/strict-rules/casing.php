<?php

namespace StrictCasing;

interface Iface
{
	public function interfaceMethod(): void;
}

class ParentClass
{
	public function parentMethod(): void
	{
	}

	private function privateMethod(): void
	{
	}

	public static function staticMethod(): void
	{
	}
}

class Child extends ParentClass implements Iface
{
	public function parentmethod(): void
	{
	}

	public function INTERFACEMETHOD(): void
	{
	}

	public function privatemethod(): void
	{
	}

	public static function staticMethod(): void
	{
	}

	public function own(): void
	{
	}
}

class Grand extends Child
{
	public function PARENTMETHOD(): void
	{
	}

	public function interfacemethod(): void
	{
	}
}

trait CasingTrait
{
	public function parentmethod(): void
	{
	}
}

class WithTrait extends ParentClass
{
	use CasingTrait;
}

class ExtendsBuiltin extends \ArrayObject
{
	public function COUNT(): int
	{
		return 0;
	}
}

class ImplementsBuiltin implements \Countable
{
	public function count(): int
	{
		return 0;
	}
}

class ImplementsBuiltinWrong implements \Countable
{
	public function Count(): int
	{
		return 0;
	}
}

interface ChildIface extends Iface
{
	public function interfacemethod(): void;
}

enum CasingEnum implements Iface
{
	public function interfacemethod(): void
	{
	}
}
