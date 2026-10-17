<?php // lint >= 8.4

namespace Bug12553;

interface TimestampsInterface
{
	public \DateTimeImmutable $createdAt { get; }
}

trait Timestamps
{
	public private(set) \DateTimeImmutable $createdAt {
		get {
			return $this->createdAt ??= new \DateTimeImmutable();
		}
	}
}

class Example implements TimestampsInterface
{
	use Timestamps;
}
