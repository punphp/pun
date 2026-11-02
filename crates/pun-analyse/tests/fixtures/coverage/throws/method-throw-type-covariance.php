<?php

namespace MethodThrowTypeCovariance;

class Foo
{

	public function noThrowType(): void
	{

	}

	/**
	 * @throws \LogicException
	 */
	public function logicException(): void
	{

	}

}

class Bar extends Foo
{

	public function noThrowType(): void
	{
		// ok
	}

	/**
	 * @throws \BadFunctionCallException
	 */
	public function logicException(): void
	{
		// ok
	}

}

class Baz extends Foo
{

	/**
	 * @throws \Exception
	 */
	public function noThrowType(): void
	{
		// ok with implicitThrows: true
		// error with implicitThrows: false
	}

	/**
	 * @throws \RuntimeException
	 */
	public function logicException(): void
	{
		// error - RuntimeException does not extend LogicException
	}

}

class VoidThrows extends Foo
{

	/**
	 * @throws void
	 */
	public function noThrowType(): void
	{
		// ok
	}

	/**
	 * @throws void
	 */
	public function logicException(): void
	{
		// ok
	}

}

class VoidInParent
{
	/**
	 * @throws void
	 */
	public function throwVoid(): void
	{

	}
}

class VoidInChild extends VoidInParent
{

	public function throwVoid(): void
	{
		// ok - @throws void inherited
	}
}

class VoidInChild2 extends VoidInParent
{

	/**
	 * @throws void
	 */
	public function throwVoid(): void
	{
		// ok
	}
}

class VoidInChild3 extends VoidInParent
{

	/**
	 * @throws \RuntimeException
	 */
	public function throwVoid(): void
	{
		// error
	}

}
