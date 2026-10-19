<?php

namespace ReadonlyUpperConstructor;

class Upper
{
	private readonly int $p;

	public function __CONSTRUCT()
	{
		echo $this->p;
		$this->p = 1;
	}
}

class Lower
{
	private readonly int $p;

	public function __construct()
	{
		echo $this->p;
		$this->p = 1;
	}
}
