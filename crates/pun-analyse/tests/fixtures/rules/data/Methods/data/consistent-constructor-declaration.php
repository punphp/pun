<?php

namespace ConsistentConstructorDeclaration;

/** @phpstan-consistent-constructor */
class Foo
{

	public function __construct()
	{
		// not private
	}

}

/** @phpstan-consistent-constructor */
final class Bar
{

	private function __construct()
	{
		// private but class final
	}

}

/** @phpstan-consistent-constructor */
class Baz
{

	private function __construct()
	{
		// error
	}

}
