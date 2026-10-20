<?php

namespace CoveragePhpDocTags;

function branches(int $i): void
{
	if ($i === 1) {
		echo 1;
	}
	/** @phpstan-elseif */
	elseif ($i === 2) {
		echo 2;
	}
	/** @phpstan-else */
	else {
		echo 3;
	}
	if ($i === 1):
		echo 1;
	/** @phpstan-colon-elseif */
	elseif ($i === 2):
		echo 2;
	/** @phpstan-colon-else */
	else:
		echo 3;
	endif;
	/** @Foo(1 2) @phpstan-unknown */
	echo 4;
}

/**
 * @phpstan-require-extends
 */
interface RequireNothing
{
}

/**
 * @phpstan-sealed int
 */
class SealedInt
{
}
