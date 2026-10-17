<?php

namespace Bug12951Core {

	class NumberFormatter extends \Bug12951Polyfill\NumberFormatter
	{

	}

}

namespace Bug12951Polyfill {

	/** @internal */
	class NumberFormatter
	{

		public const NUMERIC_COLLATION = 1;

		public static $prop;

		public function __construct()
		{

		}

		public static function doBar(): void
		{

		}

	}

}
