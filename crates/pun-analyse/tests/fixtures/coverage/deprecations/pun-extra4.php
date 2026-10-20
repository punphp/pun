<?php

namespace DeprecationsExtra4;

class MyMysqli extends \mysqli
{
	public function doKill(): void
	{
		self::kill(1);
		static::kill(1);
	}
}

class HasStatic
{
	/** @deprecated */
	public static $old = 1;

	public function read(): void
	{
		echo $this::$old;
	}
}
