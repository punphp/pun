<?php

use function PHPStan\dumpType;

function unreachableDump(): void
{
    return;
    dumpType(42);
}
