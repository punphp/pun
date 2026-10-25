<?php

namespace MixedGuards;

function scalarGuards(mixed $value): void
{
    if (!is_int($value)) {
        \PHPStan\dumpType($value);
        $copy = $value;
        \PHPStan\dumpType($copy);
    }
    if (!is_string($value)) {
        \PHPStan\dumpType($value);
    }
    if (!is_float($value)) {
        \PHPStan\dumpType($value);
    }
    if (!is_bool($value)) {
        \PHPStan\dumpType($value);
    }
    if (!is_resource($value)) {
        \PHPStan\dumpType($value);
    }
    \PHPStan\dumpType($value);
}

function offsetDomains(mixed $value): void
{
    if (!is_array($value)) {
        \PHPStan\dumpType($value);
    }
    if (!is_object($value)) {
        \PHPStan\dumpType($value);
    }
    if (!$value instanceof \ArrayAccess) {
        \PHPStan\dumpType($value);
    }
    if (!is_string($value) && !is_array($value)) {
        \PHPStan\dumpType($value);
        if (!$value instanceof \ArrayAccess) {
            \PHPStan\dumpType($value);
            unset($value['excluded']);
        }
    }
}

function partialOffsetDomain(mixed $value): void
{
    if (!is_string($value) && !is_array($value)) {
        unset($value['partial']);
    }
}

function terminatingOr(mixed $value): void
{
    if (is_int($value) || is_string($value)) {
        return;
    }
    \PHPStan\dumpType($value);
}

function partialJoin(mixed $value, bool $choose): void
{
    if (is_int($value)) {
        return;
    }
    if ($choose) {
        $value = 13;
    }
    \PHPStan\dumpType($value);
}

function branchJoin(mixed $value, bool $choose): void
{
    if ($choose) {
        if (is_int($value)) {
            return;
        }
    } elseif (is_string($value)) {
        return;
    }
    \PHPStan\dumpType($value);
}

function repeatedAndNull(mixed $value): void
{
    if (is_int($value)) {
        return;
    }
    \PHPStan\dumpType($value);
    if (is_int($value)) {
        return;
    }
    if ($value !== null) {
        \PHPStan\dumpType($value);
    }
    if (!is_null($value)) {
        \PHPStan\dumpType($value);
    }
    \PHPStan\dumpType($value);
    $value = 5;
    \PHPStan\dumpType($value);
}

function strictAndLoose(mixed $value): void
{
    if ($value !== 0) {
        \PHPStan\dumpType($value);
    }
    if ($value !== 'x') {
        \PHPStan\dumpType($value);
    }
    if ($value != 'x') {
        \PHPStan\dumpType($value);
    }
    if ($value !== false) {
        \PHPStan\dumpType($value);
    }
}

function errorGuards(): void
{
    $value = $undefined;
    if (!is_int($value)) {
        \PHPStan\dumpType($value);
    }
    if (!is_string($value)) {
        \PHPStan\dumpType($value);
    }
    if (!is_null($value)) {
        \PHPStan\dumpType($value);
    }
}

function nullGuards(mixed $value): void
{
    if (!is_null($value)) {
        \PHPStan\dumpType($value);
    }
    if ($value !== null) {
        \PHPStan\dumpType($value);
    }
    \PHPStan\dumpType($value);
}

function truthinessGuards(mixed $value): void
{
    if (!$value) {
        if (!is_int($value)) {
            \PHPStan\dumpType($value);
        }
        if (!is_string($value)) {
            \PHPStan\dumpType($value);
        }
    }
    if ($value) {
        if (!is_int($value)) {
            \PHPStan\dumpType($value);
        }
    }
}

function mutatedOffsetDomain(mixed $value): void
{
    if (!is_string($value) && !is_array($value)) {
        unset($value['partial']);
        if (!$value instanceof \ArrayAccess) {
            \PHPStan\dumpType($value);
            unset($value['still-possible']);
        }
    }
}
