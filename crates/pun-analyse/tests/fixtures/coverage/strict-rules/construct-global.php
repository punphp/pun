<?php

class StrictOldStyle
{
	public function StrictOldStyle()
	{
	}
}

class StrictFromOldStyle extends StrictOldStyle
{
	public function __construct()
	{
	}
}

abstract class StrictAbstractOldStyle
{
	abstract public function StrictAbstractOldStyle();
}

class StrictFromAbstractOldStyle extends StrictAbstractOldStyle
{
	public function StrictAbstractOldStyle()
	{
	}

	public function __construct()
	{
	}
}

class StrictCase extends StrictOldStyle
{
	public function __CONSTRUCT()
	{
	}
}

class StrictUpperParent extends StrictFromOldStyle
{
	public function __construct()
	{
		PARENT::__construct();
		return;
	}
}

class StrictNested extends StrictFromOldStyle
{
	public function __construct()
	{
		{
			parent::__construct();
		}
	}
}

class StrictLoops extends StrictFromOldStyle
{
	public function __construct(array $a)
	{
		for (;;):
		endfor;
		foreach ($a as $x) {
		}
		foreach ($a as $x):
			parent::__construct();
		endforeach;
	}
}

class StrictDo extends StrictFromOldStyle
{
	public function __construct()
	{
		do {
			parent::__construct();
		} while (false);
	}
}

class StrictDeclare extends StrictFromOldStyle
{
	public function __construct()
	{
		declare(ticks=1) {
			parent::__construct();
		}
	}
}

class StrictInnerFunction extends StrictFromOldStyle
{
	public function __construct()
	{
		function strictInner()
		{
			parent::__construct();
		}
	}
}

class StrictClosureStatement extends StrictFromOldStyle
{
	public function __construct()
	{
		function () {
			parent::__construct();
		};
	}
}

class StrictColonIf extends StrictFromOldStyle
{
	public function __construct(bool $b)
	{
		while ($b):
		endwhile;
		declare(ticks=1):
		enddeclare;
		for (;;) {
		}
		if ($b):
			parent::__construct();
		endif;
	}
}
