<?php

namespace Level4Unused;

function labels(int $x): void
{
	start:
	unused:
	if ($x > 0) {
		$x--;
		goto start;
	}
	inner:
	$f = function () {
		goto inner;
		inner:
		outer:
	};
}

top:
