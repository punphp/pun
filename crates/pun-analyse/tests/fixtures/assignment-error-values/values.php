<?php

namespace AssignmentErrorValues;

function assignments(mixed $unknown): void
{
    [$missing, $alsoMissing] = [];
    ['age' => $age] = ['name' => 'kept'];
    [$first, $second] = [1];
    \PHPStan\dumpType($errorCopy = $missing);
    \PHPStan\dumpType($age);
    \PHPStan\dumpType($second);
    \PHPStan\dumpType($missing);
    \PHPStan\dumpType($alsoMissing);
    \PHPStan\dumpType($first);
    \PHPStan\dumpType($nullCopy = null);
    \PHPStan\dumpType($mixedCopy = $unknown);
}
