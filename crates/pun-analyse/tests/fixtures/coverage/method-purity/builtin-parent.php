<?php declare(strict_types = 1);

namespace MethodPurityBuiltinParent;

class ImpureCount implements \Countable
{

	/** @phpstan-impure */
	public function count(): int
	{
		return random_int(0, 1);
	}

}
