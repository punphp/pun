<?php

namespace UppercaseString;

class Foo
{

	/** @param uppercase-string $s */
	function doFoo($s) {
		if ($s === 'ab') {}
		if ('ab' === $s) {}
		if ('ab' !== $s) {}
		if ($s === 'AB') {}
		if ($s !== 'AB') {}
		if ($s === 'aBc') {}
		if ($s !== 'aBc') {}
		if ($s === '01') {}
		if ($s === '1E2') {}
		if ($s === MyClass::myConst) {}
		if ($s === A_GLOBAL_CONST) {}
		if (doFoo('HI') === 'ab') {}
	}

	/** @param uppercase-string $s */
	function doFoo2($s, int $x, int $y) {
		$part = substr($s, $x, $y);

		if ($part === 'ab') {}
    }

}
