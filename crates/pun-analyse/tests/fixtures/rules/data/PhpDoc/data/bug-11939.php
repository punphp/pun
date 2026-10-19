<?php // lint >= 8.1

declare(strict_types=1);

namespace Bug11939;

enum What
{
	case This;
	case That;

	/**
	 * @return ($this is self::This ? 'here' : 'there')
	 */
	public function where(): string
	{
		return match ($this) {
			self::This => 'here',
			self::That => 'there'
		};
	}
}

class Where
{
	/**
	 * @return ($what is What::This ? 'here' : 'there')
	 */
	public function __invoke(What $what): string
	{
		return match ($what) {
			What::This => 'here',
			What::That => 'there'
		};
	}
}
