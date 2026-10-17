<?php // lint >= 8.4

namespace Bug12466;

interface Foo
{

	public int $a { get; set;}

}

class Bar implements Foo
{

	public string $a;

}

interface MoreProps
{

	public int $a { get; set; }

	public int $b { get; }

	public int $c { set; }

}

class TestMoreProps implements MoreProps
{

	// not writable
	public int $a {
		get {
			return 1;
		}
	}

	// not readable
	public int $b {
		set {
			$this->a = 1;
		}
	}

	// not writable
	public int $c {
		get {
			return 1;
		}
	}

}
