<?php declare(strict_types=1);

namespace FailedReadConstructs;

function reads(): void
{
    $shape = ['present' => 1];
    echo $shape['echo-missing'];
    print $shape['print-missing'];
    eval($shape['eval-missing']);
    echo $undefined;
    $index = 0;
    echo $shape[$index++], $index++;
    \PHPStan\dumpType($index);
    $result = print $shape['print-result'];
    \PHPStan\dumpType($result);
    echo $shape['missing-sibling'], [];
    print [];
}

function exitRead(): never
{
    $shape = ['present' => 1];
    exit($shape['exit-missing']);
}

function dieRead(): never
{
    $shape = ['present' => 1];
    die($shape['die-missing']);
}

function nullControl(): never
{
    exit(null);
}

function falseControl(): never
{
    die(false);
}

function mixedControl(mixed $value): never
{
    exit($value);
}

/** @phpstan-pure */
function purityControl(): int
{
    $shape = ['present' => 1];
    echo $shape['pure-missing'];
    return 1;
}
