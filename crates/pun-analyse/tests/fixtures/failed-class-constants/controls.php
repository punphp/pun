<?php

namespace FailedClassConstants;

final class One { const NUMBER = 11; }
final class Two { const NUMBER = 'two'; }
final class Missing {}

function observations(mixed $unknown, One|Two $receiver): void
{
    $failed = One::MISSING;
    \PHPStan\dumpType($failed);
    \PHPStan\dumpType($copy = $failed);
    \PHPStan\dumpType($copy);
    echo 'after-failed-assignment';

    $_invalid = One::{123};
    \PHPStan\dumpType($_invalid);
    echo 'after-dynamic-invalid';

    \PHPStan\dumpType(One::NUMBER);
    \PHPStan\dumpType($receiver::NUMBER);
    $_literal = One::{'NUMBER'};
    \PHPStan\dumpType($_literal);
    $class = One::class;
    \PHPStan\dumpType($class);

    $_sequence = ($receiver = new One())::{($selector = 'NUMBER')};
    \PHPStan\dumpType($receiver);
    \PHPStan\dumpType($selector);
    \PHPStan\dumpType($_sequence);

    $genuine = $unknown;
    \PHPStan\dumpType($genuine);
    echo 'after-controls';
}

function partialUnion(One|Missing $receiver): void
{
    $_partial = $receiver::NUMBER;
    \PHPStan\dumpType($_partial);
    echo 'after-partial-union';
}
