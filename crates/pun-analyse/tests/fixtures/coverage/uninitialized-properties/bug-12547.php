<?php // lint >= 8.4

declare(strict_types = 1);

namespace Bug12547;

class Example {
	public \DateTimeImmutable $noon {
		get => new \DateTimeImmutable('12:00');
	}
}
