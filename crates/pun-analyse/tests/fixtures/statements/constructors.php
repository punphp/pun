<?php

namespace Constructors;

trait Base
{
	public function __construct(): void
	{
	}

	public function doFoo(): int
	{
		return 1;
	}
}

class Shadows
{
	use Base;

	public function __construct()
	{
	}
}

class Aliases
{
	use Base {
		doFoo as doBar;
	}
}

interface HasConstructor
{
	public function __construct(): void;
}
