<?php

namespace IncompatibleAssertTypeWithMethodReturnType;

class Foo
{

	/**
	 * @return list<int>
	 */
	public function getValues(): array
	{

	}

	/**
	 * @phpstan-assert-if-false non-empty-list<static> $this->getValues()
	 */
	public function isEmpty(): bool
	{

	}

}
