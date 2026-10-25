<?php

namespace ArrayRand;

use function PHPStan\dumpType;

function acceptInt(int $key): void
{
    echo $key;
}

/** @param list<never> $values */
function genericBottomList(array $values): void
{
    dumpType('before');
    acceptInt(array_rand($values));
}

/** @param array<never, string> $values */
function genericBottomKeys(array $values): void
{
    dumpType('before');
    acceptInt(array_rand($values));
}
