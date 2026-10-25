<?php declare(strict_types=1);

namespace FailedReadArguments;

function takesInt(int $value): void {}

function takesPair(int $value, int $other): void {}

/** @param-out int $value */
function initialize(int &$value): void
{
    $value = 7;
}

function arguments(mixed $unknown): void
{
    $shape = ['present' => 1];
    takesInt($shape['missing']);
    takesInt($undefined);
    takesPair($shape['absent'], 'wrong');
    takesPair($shape['no-other']);
    takesInt('wrong');
    takesInt($unknown);
    takesInt(null);
    takesInt(false);
    takesPair(other: $shape['missing-named'], value: 'wrong');
    takesInt(unknown: $shape['unknown-parameter']);
    $index = 0;
    takesInt($shape[$index++]);
    \PHPStan\dumpType($index);
    $output = 1;
    \PHPStan\dumpType($output);
    initialize($output);
    \PHPStan\dumpType($output);
    takesInt($output);
    initialize(1);
}
