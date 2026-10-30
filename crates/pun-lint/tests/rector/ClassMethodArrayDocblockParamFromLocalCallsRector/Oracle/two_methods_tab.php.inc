<?php

final class TwoMethodsTab
{
	public function go(): void
	{
		$this->first([1]);
		$this->second([true]);
	}

	private function first(array $items): void
	{
	}

	/** Second. */
	private function second(array $flags): void
	{
	}
}
-----
<?php

final class TwoMethodsTab
{
	public function go(): void
	{
		$this->first([1]);
		$this->second([true]);
	}

	/**
     * @param int[] $items
     */
    private function first(array $items): void
	{
	}

	/** Second.
     * @param bool[] $flags */
    private function second(array $flags): void
	{
	}
}
