<?php declare(strict_types=1);

namespace MixedAddition;

function explicitMixed(mixed $left, mixed $right): int
{
    return $left + $right;
}

function implicitMixed($left, $right): int
{
    return $left + $right;
}

function errorMixed(\stdClass $notCallable, mixed $right): int
{
    return $notCallable() + $right;
}

function mixedError(mixed $left, \stdClass $notCallable): int
{
    return $left + $notCallable();
}

function errorError(\stdClass $notCallable): int
{
    return $notCallable() + $notCallable();
}

function excludedArrays(mixed $left, mixed $right): int
{
    if (is_array($left) || is_array($right)) {
        return 0;
    }
    return $left + $right;
}

function integerControl(): int
{
    return 1 + 2;
}

function arrayControl(): array
{
    return [] + [];
}

function subtractionControl(int $left, int $right): int
{
    return $left - $right;
}
