<?php

namespace ReadonlyFlow2;

final class ParenthesizedByRef
{
	private readonly array $list;

	public function __construct()
	{
		$this->list = [3, 1];
		sort(($this->list));
	}
}

function anonymous(): object
{
	return new class {
		private readonly int $a;

		public function __construct()
		{
			$this->init();
		}

		private function init(): void
		{
			$this->a = 1;
		}
	};
}

trait Initializes
{
	public function init()
	{
		$this->a = 1;
		echo $this->a;
	}
}

final class AliasedConstructor
{
	use Initializes {
		init as __construct;
	}

	private readonly int $a;
}
