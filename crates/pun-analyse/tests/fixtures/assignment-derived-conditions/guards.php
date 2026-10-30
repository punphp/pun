<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

function selfRead(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $key = 'prefix-' . $key;
    if ($method === 'first') {
        \PHPStan\dumpType($key);
    } else {
        \PHPStan\dumpType($key);
    }
    return true;
}

function staleTargetGuard(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $method = 'replacement-' . $key;
    if ($key === 'left') {
        \PHPStan\dumpType($method);
    } else {
        \PHPStan\dumpType($method);
    }
    return true;
}

function maybeUndefined(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
    }
    $copy = $key;
    if ($method === 'first') {
        \PHPStan\dumpType($copy);
    } else {
        \PHPStan\dumpType($copy);
    }
    return true;
}

function unchangedConsequence(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $length = strlen($key);
    $fixed = [$key, 1][1];
    if ($method === 'first') {
        \PHPStan\dumpType($length);
        \PHPStan\dumpType($fixed);
    } else {
        \PHPStan\dumpType($length);
        \PHPStan\dumpType($fixed);
    }
    return true;
}
