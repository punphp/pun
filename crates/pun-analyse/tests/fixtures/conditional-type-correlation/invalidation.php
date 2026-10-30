<?php declare(strict_types=1);

namespace ConditionalInvalidation;

function specifiedIndexGuard(bool $flag): void
{
    $names = ['first', 'second'];
    $index = 0;
    if ($flag) {
        $names[$index] = 'first';
        $value = 1;
    } else {
        $names[$index] = 'second';
        $value = 2;
    }
    if ($names[$index] === 'first') {
        \PHPStan\dumpType($value);
    }
    $index = 1;
    if ($names[$index] === 'second') {
        \PHPStan\dumpType($value);
    }
}

function specifiedIndexTarget(bool $flag): void
{
    $values = [10, 20];
    $index = 0;
    if ($flag) {
        $selector = 'first';
        $values[$index] = 1;
    } else {
        $selector = 'second';
        $values[$index] = 2;
    }
    if ($selector === 'first') {
        \PHPStan\dumpType($values[$index]);
    }
    $index = 1;
    if ($selector === 'first') {
        \PHPStan\dumpType($values[$index]);
    }
}
