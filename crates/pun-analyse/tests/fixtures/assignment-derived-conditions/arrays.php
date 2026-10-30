<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

function normal(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $arguments = [$key => 1];
    if ($method === 'first') {
        \PHPStan\dumpType($arguments);
    } else {
        \PHPStan\dumpType($arguments);
    }
    return true;
}

function reversed(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'right';
    } else {
        $method = 'second';
        $key = 'left';
    }
    $arguments = [$key => 1];
    if ($method === 'first') {
        \PHPStan\dumpType($arguments);
    } else {
        \PHPStan\dumpType($arguments);
    }
    return true;
}

function capturedValueNotAlias(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $arguments = [$key => 1];
    $key = 'overwritten';
    if ($method === 'first') {
        \PHPStan\dumpType($arguments);
    } else {
        \PHPStan\dumpType($arguments);
    }
    return true;
}

function replacement(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $arguments = [$key => 1];
    $arguments = ['replacement' => 2];
    if ($method === 'first') {
        \PHPStan\dumpType($arguments);
    } else {
        \PHPStan\dumpType($arguments);
    }
    return true;
}

function compound(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
        $value = 1;
    } else {
        $method = 'second';
        $key = 'right';
        $value = 2;
    }
    $arguments = ['nested' => [$key => $value], 'copy' => $key . $key];
    if ($method === 'first') {
        \PHPStan\dumpType($arguments);
    } else {
        \PHPStan\dumpType($arguments);
    }
    return true;
}
