<?php

namespace Bug11241b;

/**
 * @property-read string $property1
 */
class HelloWorld
{
	public bool $property1 = false;
}

$object = new HelloWorld();
$object->property1 = 'foo';
