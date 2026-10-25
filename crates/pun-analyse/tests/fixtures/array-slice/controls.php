<?php

namespace ArraySlice;

use function PHPStan\dumpType;

function insufficient(): void
{
    array_slice();
    array_slice([]);
}

function invalidString(): void
{
    dumpType(array_slice('not an array', 0));
}

function invalidNull(): void
{
    dumpType(array_slice(null, 0));
}

function invalidInteger(): void
{
    dumpType(array_slice(123, 0));
}
