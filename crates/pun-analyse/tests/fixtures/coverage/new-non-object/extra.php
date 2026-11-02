<?php declare(strict_types = 1);

namespace NewNonObjectExtra;

/**
 * @param int|class-string $intOrClass
 * @param mixed $mixed
 */
function doFoo($intOrClass, $mixed, ?object $nullableObject, \Unknown $unknown): void
{
	new $intOrClass;
	new $mixed;
	new $nullableObject;
	new $unknown;
	new (1 + 1);
	$f = 1.5;
	new $f(1, 2);
}
