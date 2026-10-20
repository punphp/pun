<?php // lint >= 8.0

namespace Bug9213;

class IterateSomething
{
	/** @var string|null */
	private $currentKey;

	/** @var string|null */
	private $currentValue;

	/** @param iterable<string, string> $collection */
	public function __construct(private iterable $collection) {}

	public function printAllTheThings(): void
	{
		foreach ($this->collection as $this->currentKey => $this->currentValue) {
			echo "{$this->currentKey} => {$this->currentValue}\n";
		}
	}
}
