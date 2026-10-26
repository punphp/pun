<?php

namespace MixedArrayGuards;

function positiveGuard(mixed $value, bool $choose): void
{
    if (is_array($value)) {
        \PHPStan\dumpType($value);
        $copy = $value;
        \PHPStan\dumpType($copy);
        if ($choose) {
            $copy = [];
        }
        \PHPStan\dumpType($copy);
        if ($choose) {
            $copy = [5];
        }
        \PHPStan\dumpType($copy);
        if (is_array($copy)) {
            \PHPStan\dumpType($copy);
        }
    }
}

function implicitArray(array $value): void
{
    \PHPStan\dumpType($value);
    $copy = $value;
    if (is_array($copy)) {
        \PHPStan\dumpType($copy);
    }
}

function implicitUnknown($value): void
{
    \PHPStan\dumpType($value);
    if (is_array($value)) {
        \PHPStan\dumpType($value);
    }
}

function falseyIntersections(mixed $value): void
{
    if (!$value) {
        if (is_int($value)) {
            \PHPStan\dumpType($value);
        }
        if (is_float($value)) {
            \PHPStan\dumpType($value);
        }
        if (is_string($value)) {
            \PHPStan\dumpType($value);
        }
        if (is_array($value)) {
            \PHPStan\dumpType($value);
        }
        if (is_bool($value)) {
            \PHPStan\dumpType($value);
        }
        if (is_null($value)) {
            \PHPStan\dumpType($value);
        }
    }
}

function copiedExclusion(mixed $value, bool $choose): void
{
    if (!is_array($value)) {
        $copy = $value;
        \PHPStan\dumpType($copy);
        if ($choose) {
            unset($copy['removed']);
        }
        \PHPStan\dumpType($copy);
        if (!is_string($copy) && !$copy instanceof \ArrayAccess) {
            \PHPStan\dumpType($copy);
            unset($copy['possible']);
        }
    }
}

/** @param array<string, int>|null $value */
function typedArrayGuard(?array $value): void
{
    if (is_array($value)) {
        \PHPStan\dumpType($value);
    }
}

/** @param array<array-key, int> $value */
function keyAliasGuard(array $value): void
{
    if (is_array($value)) {
        \PHPStan\dumpType($value);
    }
}

/** @param array<int|string, int> $value */
function keyUnionGuard(array $value): void
{
    if (is_array($value)) {
        \PHPStan\dumpType($value);
    }
}

namespace CustomArrayGuard;

/**
 * @phpstan-assert-if-true string $value
 * @assert-if-true string $value
 */
function is_array(mixed $value): bool
{
    return \is_string($value);
}

function customGuard(mixed $value): void
{
    if (is_array($value)) {
        \PHPStan\dumpType($value);
    }
    if (\is_array($value)) {
        \PHPStan\dumpType($value);
    }
}

namespace UnspecifiedCustomArrayGuard;

function is_array(mixed $value): bool
{
    return \is_string($value);
}

function customGuard(mixed $value): void
{
    if (is_array($value)) {
        \PHPStan\dumpType($value);
    }
}
