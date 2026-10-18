<?php // lint >= 8.0

namespace Bug9657;

/**
 * @template T
 */
trait Convertable
{
	/**
	 * @return T
	 */
	abstract public function toOther(): mixed;
}

final class Thing
{
	/** @use Convertable<list<never>> */
	use Convertable;

	public function toOther(): array
	{
		return [];
	}
}
