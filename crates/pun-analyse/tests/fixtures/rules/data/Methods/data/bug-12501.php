<?php // lint >= 8.4

declare(strict_types = 1);

namespace Bug12501;

final readonly class EmptyObject {
	public function __construct(
		public null $value1 = null,
	) {}
}
