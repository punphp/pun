<?php // lint >= 8.0

declare(strict_types = 1);

namespace Bug6947;

abstract class HelloWorld
{
	public function sayHello(): void
	{
		if (is_string($this->getValue())) {

		} elseif (is_array($this->getValue())) {

		}
	}

	abstract public function getValue():int|float|string|null;
}
