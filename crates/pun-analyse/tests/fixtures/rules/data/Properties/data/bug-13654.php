<?php

namespace Bug13654;

/**
 * @phpstan-type TType array{
 *    text: string,
 *	  missing: string,
 * }
 */
class HelloWorld
{
	/**
	 * @var TType|null
	 */
	private ?array $test = null;

	/**
	 * @param TType $test
	 */
	public function setTest(array $test): void
	{
		$this->test = $test;
	}

	/**
	 * @return TType
	 */
	public function modify(): array
	{
		if ($this->test === null) {
			throw new \Exception();
		}

		$this->test['text'] = 'asdsdsd';

		return $this->test;
	}
}
