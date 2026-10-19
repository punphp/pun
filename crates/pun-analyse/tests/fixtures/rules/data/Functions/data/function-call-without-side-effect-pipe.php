<?php // lint >= 8.5

namespace FunctionCallWithoutSideEffectPipe;

/**
 * @phpstan-pure
 */
function pureFunc(int $i): int
{
	return $i;
}

function maybePureFunc(int $i): int
{
	return $i;
}

/**
 * @phpstan-impure
 */
function impurePureFunc(int $i): int
{
	echo '5';
	return $i;
}

function (): void {
	5 |> pureFunc(...);
	5 |> maybePureFunc(...);
	5 |> impurePureFunc(...);
};
