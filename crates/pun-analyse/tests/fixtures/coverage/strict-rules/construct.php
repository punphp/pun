<?php

namespace StrictConstruct;

class Base
{
	public function __construct()
	{
	}
}

class NoCall extends Base
{
	public function __construct()
	{
	}
}

class Calls extends Base
{
	public function __construct()
	{
		parent::__construct();
	}
}

class Suppressed extends Base
{
	public function __construct()
	{
		@parent::__construct();
	}
}

class InIf extends Base
{
	public function __construct(bool $b)
	{
		if ($b) {
			parent::__construct();
		}
	}
}

class InElse extends Base
{
	public function __construct(bool $b)
	{
		if ($b) {
		} else {
			parent::__construct();
		}
	}
}

class InFunction extends Base
{
	public function __construct()
	{
		$f = function () {
			parent::__construct();
		};
	}
}

class Grandchild extends NoCall
{
	public function __construct()
	{
	}
}

class NoConstructor extends Base
{
}

class GreatGrandchild extends NoConstructor
{
	public function __construct()
	{
	}
}

abstract class AbstractCtor
{
	abstract public function __construct();
}

class FromAbstract extends AbstractCtor
{
	public function __construct()
	{
	}
}

class PrivateCtor
{
	private function __construct()
	{
	}
}

class FromPrivate extends PrivateCtor
{
	public function __construct()
	{
	}
}

class DeprecatedCtor
{
	/** @deprecated */
	public function __construct()
	{
	}
}

class FromDeprecated extends DeprecatedCtor
{
	public function __construct()
	{
	}
}

class OldStyle
{
	public function OldStyle()
	{
	}
}

class FromOldStyle extends OldStyle
{
	public function __construct()
	{
	}
}

class Lonely
{
	public function __construct()
	{
	}
}

trait CtorTrait
{
	public function __construct()
	{
	}
}

class UsesTrait extends Base
{
	use CtorTrait;
}

abstract class AbstractChild extends Base
{
	abstract public function __construct();
}

interface CtorInterface
{
	public function __construct();
}

function anon(): void
{
	new class extends Base {
		public function __construct()
		{
		}
	};
}

class FromException extends \Exception
{
	public function __construct()
	{
	}
}

class FromArrayObject extends \ArrayObject
{
	public function __construct()
	{
	}
}

class OtherStatic extends Base
{
	public function __construct()
	{
		Base::__construct();
		parent::other();
	}
}

class InWhile extends Base
{
	public function __construct(bool $b)
	{
		while ($b) {
			parent::__construct();
		}
	}
}

class InTry extends Base
{
	public function __construct()
	{
		try {
			parent::__construct();
		} catch (\Throwable $e) {
		}
	}
}
