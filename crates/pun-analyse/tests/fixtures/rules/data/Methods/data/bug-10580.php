<?php // lint >= 8.0

namespace Bug10580;

interface FooI {
	/** @return $this */
	public function fooThisInterface(): FooI;
	/** @return $this */
	public function fooThisClass(): FooI;
	/** @return $this */
	public function fooThisSelf(): self;
	/** @return $this */
	public function fooThisStatic(): static;
}

final class FooA implements FooI
{
	public function fooThisInterface(): FooI { return new FooA(); }
	public function fooThisClass(): FooA { return new FooA(); }
	public function fooThisSelf(): self { return new FooA(); }
	public function fooThisStatic(): static { return new FooA(); }
}

final class FooB implements FooI
{
	/** @return $this */
	public function fooThisInterface(): FooI { return new FooB(); }
	/** @return $this */
	public function fooThisClass(): FooB { return new FooB(); }
	/** @return $this */
	public function fooThisSelf(): self { return new FooB(); }
	/** @return $this */
	public function fooThisStatic(): static { return new FooB(); }
	/** @return $this */
	public function fooThis(): static { return new FooB(); }
}
