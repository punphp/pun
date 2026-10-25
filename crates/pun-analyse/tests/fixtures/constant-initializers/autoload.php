<?php

namespace LazyInitializers;

function loaded(): void
{
    $class = Loaded::class;
    $value = VALUE;
    \PHPStan\dumpType($value);
    $value = 9;
    \PHPStan\dumpType($value);
}
