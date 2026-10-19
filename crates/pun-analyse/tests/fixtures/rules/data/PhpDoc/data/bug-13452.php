<?php

namespace Bug13452;

use ReflectionClass;

class Event
{

}

class HelloWorld
{

	/**
	 * @param ReflectionClass<covariant Event> $ref
	 */
	public function doFoo(ReflectionClass $ref): void
	{

	}

}
