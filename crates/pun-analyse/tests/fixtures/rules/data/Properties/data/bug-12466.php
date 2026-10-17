<?php // lint >= 8.4

namespace Bug12466OverridenProperty;

interface Foo
{

	public int|string $onlyGet { get; }

	public int|string $onlySet { set; }

}

class Bar implements Foo
{

	public int $onlyGet {
		get {
			return 1;
		}
	}

	public int|string|null $onlySet {
		set {
			$this->onlySet = $value;
		}
	}

}

class Baz implements Foo
{

	public int|string|null $onlyGet {
		get {
			return null;
		}
	}

	public int $onlySet {
		set {
			$this->onlySet = $value;
		}
	}

}

interface FooWithPhpDocs
{

	/** @var array<int|string> */
	public array $onlyGet { get; }

	/** @var array<int|string> */
	public array $onlySet { set; }

}

class BarWithPhpDocs implements FooWithPhpDocs
{

	/** @var array<int> */
	public array $onlyGet {
		get {
			return [];
		}
	}

	/** @var array<int|string|null> */
	public array $onlySet {
		set {
			$this->onlySet = $value;
		}
	}

}

class BazWithPhpDocs implements FooWithPhpDocs
{

	/** @var array<int|string|null> */
	public array $onlyGet {
		get {
			return [];
		}
	}

	/** @var array<int> */
	public array $onlySet {
		set {
			$this->onlySet = $value;
		}
	}

}
