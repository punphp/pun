<?php declare(strict_types=1);

namespace ConditionalTypeCorrelation;

class LeftReceiver { public function first(): int { return 1; } }
class RightReceiver { public function second(): string { return 'second'; } }

function receiver(bool $flag, LeftReceiver $left, RightReceiver $right): void
{
    if ($flag) {
        $method = 'first';
        $object = $left;
    } else {
        $method = 'second';
        $object = $right;
    }
    if ($method === 'second') {
        \PHPStan\dumpType($object);
        \PHPStan\dumpType($object->second());
    } else {
        \PHPStan\dumpType($object);
    }
    \PHPStan\dumpType($object);
    if ('first' === $method) {
        \PHPStan\dumpType($object);
    }
}

function reversedReceiver(bool $flag, LeftReceiver $left, RightReceiver $right): void
{
    if ($flag) {
        $method = 'second';
        $object = $left;
    } else {
        $method = 'first';
        $object = $right;
    }
    if ($method !== 'first') {
        \PHPStan\dumpType($object);
        \PHPStan\dumpType($object->first());
    }
}

function unpack(bool $flag): void
{
    if ($flag) {
        $method = 'first';
        $arguments = ['first' => 1];
    } else {
        $method = 'second';
        $arguments = ['second' => 'value'];
    }
    if ($method === 'first') {
        \PHPStan\dumpType($arguments);
        consume(...$arguments);
    } else {
        \PHPStan\dumpType($arguments);
        consume(...$arguments);
    }
}
function consume(int $first = 0, string $second = ''): void {}

/**
 * @param int<0, 10> $low
 * @param int<5, 15> $high
 */
function overlappingRanges(bool $flag, int $low, int $high): void
{
    if ($flag) {
        $number = $low;
        $result = 'low';
    } else {
        $number = $high;
        $result = 'high';
    }
    if ($number === 2) {
        \PHPStan\dumpType($result);
    }
    if (12 === $number) {
        \PHPStan\dumpType($result);
    }
    if ($number === 7) {
        \PHPStan\dumpType($result);
    }
}

/** @param positive-int $positive */
function absorbedGuard(bool $flag, int $wide, int $positive): void
{
    if ($flag) {
        $number = $wide;
        $result = 'wide';
    } else {
        $number = $positive;
        $result = 'positive';
    }
    if ($number === -1) {
        \PHPStan\dumpType($result);
    }
    if ($number === 1) {
        \PHPStan\dumpType($result);
    }
}

/** @param positive-int $positive */
function absorbedTarget(bool $flag, int $wide, int $positive): void
{
    if ($flag) {
        $method = 'wide';
        $number = $wide;
    } else {
        $method = 'positive';
        $number = $positive;
    }
    if ($method === 'positive') {
        \PHPStan\dumpType($number);
    }
    if ($method === 'wide') {
        \PHPStan\dumpType($number);
    }
}

/**
 * @param 'a'|'b' $first
 * @param 'b'|'c' $second
 */
function overlappingStrings(bool $flag, string $first, string $second): void
{
    if ($flag) {
        $selector = $first;
        $result = 1;
    } else {
        $selector = $second;
        $result = 2;
    }
    if ($selector === 'a') {
        \PHPStan\dumpType($result);
    }
    if ($selector === 'b') {
        \PHPStan\dumpType($result);
    }
    if ($selector === 'c') {
        \PHPStan\dumpType($result);
    }
}

function absence(bool $flag): void
{
    if ($flag) {
        $selector = 'present';
        $value = 1;
    } else {
        $selector = 'absent';
    }
    if ($selector === 'absent') {
        echo $value;
    } else {
        \PHPStan\dumpType($value);
    }
}

function guardWrite(bool $flag): void
{
    if ($flag) {
        $selector = 'first';
        $value = 1;
    } else {
        $selector = 'second';
        $value = 2;
    }
    $selector = 'first';
    if ($selector === 'first') {
        \PHPStan\dumpType($value);
    }
}

function targetWrite(bool $flag): void
{
    if ($flag) {
        $selector = 'first';
        $value = 1;
    } else {
        $selector = 'second';
        $value = 2;
    }
    $value = 3;
    if ($selector === 'first') {
        \PHPStan\dumpType($value);
    }
}

function targetUnset(bool $flag): void
{
    if ($flag) {
        $selector = 'first';
        $value = 1;
    } else {
        $selector = 'second';
        $value = 2;
    }
    unset($value);
    if ($selector === 'first') {
        echo $value;
    }
}

function unrelated(bool $chooseName, bool $chooseValue): void
{
    if ($chooseName) {
        $selector = 'first';
    } else {
        $selector = 'second';
    }
    if ($chooseValue) {
        $value = 1;
    } else {
        $value = 2;
    }
    if ($selector === 'first') {
        \PHPStan\dumpType($value);
    }
}

function threeBranches(int $pick): void
{
    if ($pick === 0) {
        $selector = 'first';
        $value = 1;
    } elseif ($pick === 1) {
        $selector = 'second';
        $value = 2;
    } else {
        $selector = 'third';
        $value = 3;
    }
    if ($selector === 'first') {
        \PHPStan\dumpType($value);
    }
    if ($selector === 'second') {
        \PHPStan\dumpType($value);
    }
    if ($selector === 'third') {
        \PHPStan\dumpType($value);
    }
}

function transitive(bool $flag): void
{
    if ($flag) {
        $selector = 'first';
        $intermediate = 'left';
        $value = 1;
    } else {
        $selector = 'second';
        $intermediate = 'right';
        $value = 2;
    }
    if ($selector === 'first') {
        \PHPStan\dumpType($intermediate);
        \PHPStan\dumpType($value);
    }
}

/** @param 'a'|'b' $names */
function absentSubset(bool $flag, string $names): void
{
    if ($flag) {
        $selector = $names;
    } else {
        $selector = 'c';
        $value = 1;
    }
    if ($selector === 'a') {
        echo $value;
    }
    if ($selector === 'a' || $selector === 'b') {
        echo $value;
    }
}

/**
 * @param 1|2 $first
 * @param 2|3 $second
 */
function narrowedTarget(bool $flag, int $first, int $second): void
{
    if ($flag) {
        $selector = 'first';
        $value = $first;
    } else {
        $selector = 'second';
        $value = $second;
    }
    if ($value === 2 && $selector === 'first') {
        \PHPStan\dumpType($value);
    }
}

function sameGuardConsequences(bool $outer, bool $inner): void
{
    if ($outer) {
        if ($inner) {
            $selector = 'first';
            $value = 1;
        } else {
            $selector = 'second';
            $value = 2;
        }
    } else {
        if ($inner) {
            $selector = 'first';
            $value = 3;
        } else {
            $selector = 'second';
            $value = 4;
        }
    }
    if ($selector === 'first') {
        \PHPStan\dumpType($value);
    }
    if ($selector === 'second') {
        \PHPStan\dumpType($value);
    }
}

function guardUnset(bool $flag): void
{
    if ($flag) {
        $selector = 'first';
        $value = 1;
    } else {
        $selector = 'second';
        $value = 2;
    }
    unset($selector);
    $selector = 'first';
    if ($selector === 'first') {
        \PHPStan\dumpType($value);
    }
}

function stringFallback(bool $flag, string $wide): void
{
    if ($flag) {
        $selector = $wide;
        $value = 'wide';
    } else {
        $selector = 'fixed';
        $value = 'fixed';
    }
    if ($selector === 'other') {
        \PHPStan\dumpType($value);
    }
}

function booleanRemainder(bool $flag, bool $wide): void
{
    if ($flag) {
        $selector = $wide;
        $value = 'wide';
    } else {
        $selector = true;
        $value = 'true';
    }
    if ($selector === false) {
        \PHPStan\dumpType($value);
    }
    if ($selector === true) {
        \PHPStan\dumpType($value);
    }
}
