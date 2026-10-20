<?php declare(strict_types = 1);

namespace SortWithoutEffectExtra;

class Holder
{

	/** @var array{} */
	public array $items = [];

	public function run(): void
	{
		sort($this->items);
	}

}

/** @param list<int> $list */
function unionFlags(array $list, bool $b): void
{
	ksort($list, $b ? SORT_REGULAR : SORT_NUMERIC);
}
