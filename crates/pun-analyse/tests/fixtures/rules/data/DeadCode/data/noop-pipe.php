<?php // lint >= 8.5

namespace NoopPipe;

/** @phpstan-pure */
function doFoo(int $i): int
{
	return 5;
}

5 |> doFoo(...); // reported by a specific rule

5 |> 'doFoo'; // error
