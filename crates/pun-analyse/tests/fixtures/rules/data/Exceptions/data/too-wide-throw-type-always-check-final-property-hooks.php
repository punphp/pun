<?php // lint >= 8.4

namespace TooWideThrowsPropertyHookAlwaysCheckFinal;

use LogicException;
use RuntimeException;

class Foo
{

	private int $foo {
		/** @throws LogicException|RuntimeException */
		get {
			throw new LogicException();
		}
	}

	protected int $bar {
		/** @throws LogicException|RuntimeException */
		get {
			throw new LogicException();
		}
	}

	public int $baz {
		/** @throws LogicException|RuntimeException */
		get {
			throw new LogicException();
		}
	}

	final public int $baz2 {
		/** @throws LogicException|RuntimeException */
		get {
			throw new LogicException();
		}
	}

	public int $baz3 {
		/** @throws LogicException|RuntimeException */
		final get {
			throw new LogicException();
		}
	}

}

final class FinalFoo
{

	public int $baz {
		/** @throws LogicException|RuntimeException */
		get {
			throw new LogicException();
		}
	}

}
