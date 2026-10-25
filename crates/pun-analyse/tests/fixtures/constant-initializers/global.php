<?php

function globalCopies(): void
{
    $value = GLOBAL_CHAIN;
    \PHPStan\dumpType($value);
    $value = 'reassigned';
    \PHPStan\dumpType($value);
    \PHPStan\dumpType(GLOBAL_VALUE);
}

const GLOBAL_CHAIN = GLOBAL_VALUE + 2;
const GLOBAL_VALUE = 5.9 % 2.0;
