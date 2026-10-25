<?php

namespace CyclicInitializers;

const FIRST = SECOND + 1;
const SECOND = FIRST + 1;
const SELF = \CyclicInitializers\SELF;

function cycle(): void
{
    \PHPStan\dumpType(FIRST);
    \PHPStan\dumpType(SECOND);
    \PHPStan\dumpType(\CyclicInitializers\SELF);
}
