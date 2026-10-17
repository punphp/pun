<?php

namespace Bug13171;

use Fiber;

/** @param Fiber<void, void, void, void> $a */
function foo (Fiber $a): void {
	$a->resume(1);
};
