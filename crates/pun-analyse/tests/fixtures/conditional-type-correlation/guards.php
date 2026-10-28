<?php declare(strict_types=1);

namespace ConditionalGuardBoundaries;

interface FirstTag {}
interface SecondTag {}

function overlappingInterfaces(bool $flag, FirstTag $first, SecondTag $second): void
{
    if ($flag) {
        $selector = $first;
        $value = $first;
    } else {
        $selector = $second;
    }
    if ($selector instanceof FirstTag) {
        \PHPStan\dumpType($value);
    }
}

/** @param positive-int $positive */
function absorbedArrayTarget(bool $flag, int $wide, int $positive): void
{
    if ($flag) {
        $selector = ['first'];
        $value = $positive;
    } else {
        $selector = ['second'];
        $value = $wide;
    }
    if ($selector === ['first']) {
        \PHPStan\dumpType($value);
    }
}

function arrayGuard(bool $flag): void
{
    if ($flag) {
        $selector = ['first'];
        $value = 1;
    } else {
        $selector = ['second'];
        $value = 2;
    }
    if ($selector === ['first']) {
        \PHPStan\dumpType($value);
    }
}

function indexWrite(bool $flag): void
{
    $index = 0;
    if ($flag) {
        $names = ['first', 'second'];
        $read = $names[$index];
        $value = 1;
    } else {
        $names = ['second', 'first'];
        $read = $names[$index];
        $value = 2;
    }
    if ($names[$index] === 'first') {
        \PHPStan\dumpType($value);
    }
    $index = 1;
    if ($names[$index] === 'first') {
        \PHPStan\dumpType($value);
    }
    $names = ['first', 'first'];
    if ($names[$index] === 'first') {
        \PHPStan\dumpType($value);
    }
}

function absentSpecifiedTarget(bool $flag): void
{
    if ($flag) {
        $selector = 'absent';
    } else {
        $selector = 'present';
        $value = 1;
    }
    if (isset($value) && $selector === 'absent') {
        \PHPStan\dumpType($value);
    }
}
