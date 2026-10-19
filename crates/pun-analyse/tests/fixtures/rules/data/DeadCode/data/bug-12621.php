<?php // lint >= 8.4

declare(strict_types=1);

namespace Bug12621;

final class Test
{
	private string $a {
		get => $this->a ??= $this->b;
	}

	public function __construct(
		private readonly string $b
	) {}

	public function test(): string
	{
		return $this->a;
	}
}
