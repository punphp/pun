<?php

namespace NestedTooWideFunctionParameterOutType;

/**
 * @param array<mixed> $a
 * @param-out array<array{int, bool}> $a
 */
function doFoo(array &$a): void
{
	$a = [
		[
			1,
			false,
		],
		[
			2,
			false,
		],
	];
}
