<?php // lint >= 8.4

namespace IssetVirtualProperty;

class Example {
	public \DateTimeImmutable $noon {
		get => new \DateTimeImmutable('12:00');
	}

	public ?\DateTimeImmutable $nullableNoon {
		get => new \DateTimeImmutable('12:00');
	}

	public function doFoo(): void
	{
		if (isset($this->noon)) {

		}
		if (isset($this->nullableNoon)) {

		}
	}
}
