<?php

namespace NestedTooWideFunctionReturnType;

/**
 * @return array<array{int, bool}>
 */
function dataProvider(): array
{
	return [
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
