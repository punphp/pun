<?php

namespace OverridingInterfaceConstructor;

interface HasCtor
{
	public function __construct(int $a);
}

class P implements HasCtor
{
	public function __construct(int $a)
	{
	}
}

class C extends P
{
	public function __construct(string $a)
	{
	}
}
