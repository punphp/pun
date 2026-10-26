<?php

namespace StrlenReturns;

use function PHPStan\dumpType;

function flow(bool $choice): void
{
    $length = strlen($choice ? 'x' : 'yyy');
    dumpType($length);
    dumpType(rand($length, 10));
    if (strlen('abc') === 3) {
        echo 'exact';
    }
    $str = $choice ? 'aa' : 'bbb';
    dumpType(strlen($str));
}

/** @param never $value */
function bottom($value): void
{
    dumpType(strlen($value));
}

function callables(): void
{
    dumpType(('strlen')('abc'));
    dumpType(((\strlen(...)))('abc'));
    $name = 'strlen';
    dumpType($name('abc'));
    $firstClass = \strlen(...);
    dumpType($firstClass('abc'));
    $closure = \Closure::fromCallable('strlen');
    dumpType($closure('abc'));
    dumpType((\Closure::fromCallable('strlen'))('abc'));
    dumpType((<<<'FN'
strlen
FN)('abc'));
    dumpType((<<<FN
strlen
FN)('abc'));
    $suffix = 'len';
    dumpType((<<<FN
str{$suffix}
FN)('abc'));
    dumpType(('str' . 'len')('abc'));
}
