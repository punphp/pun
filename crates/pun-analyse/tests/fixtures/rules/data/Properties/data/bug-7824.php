<?php declare(strict_types = 1);

namespace Bug7824;

use Ds\Map;

final class A
{
	/** @var Map<string, string> */
	private Map $map;

	public function __construct()
	{
		$this->map = new Map();
	}
}
