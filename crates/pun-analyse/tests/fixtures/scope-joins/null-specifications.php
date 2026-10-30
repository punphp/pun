<?php

namespace SharedNullSpecifications;

/** @phpstan-assert int|string $value */
function requireNonNull(mixed $value): bool { return true; }

/** @phpstan-assert string|null $value */
function requireNullableString(mixed $value): bool { return true; }

/** @param array<string, int|string|null> $values */
function disjunction(array $values, string $key): void
{
    requireNonNull($values[$key]) || requireNullableString($values[$key]);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function conjunction(array $values, string $key): void
{
    requireNonNull($values[$key]) && requireNullableString($values[$key]);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function independentDisjunction(array $values, string $left, string $right): void
{
    requireNonNull($values[$left]) or requireNullableString($values[$right]);
    \PHPStan\dumpType($values[$left]);
    \PHPStan\dumpType($values[$right]);
}

/** @param array<string, int|string|null> $values */
function independentConjunction(array $values, string $left, string $right): void
{
    requireNonNull($values[$left]) and requireNullableString($values[$right]);
    \PHPStan\dumpType($values[$left]);
    \PHPStan\dumpType($values[$right]);
}

/** @param array<string, int|string|null> $values */
function nested(bool $pick, array $values, string $key): void
{
    $pick || (requireNonNull($values[$key]) && requireNullableString($values[$key]));
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function replacedAfterAssertion(array $values, string $key): void
{
    requireNonNull($values[$key]) || ($values[$key] = null);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function plainPredicates(bool $pick, array $values, string $key): void
{
    $pick || is_string($values[$key]);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function assignmentResult(bool $pick, array $values, string $key): void
{
    $result = ($pick || requireNonNull($values[$key]));
    \PHPStan\dumpType($values[$key]);
    \PHPStan\dumpType($result);
}

function assignmentTarget(int|string|null $value): void
{
    $value = requireNonNull($value);
    \PHPStan\dumpType($value);
}

/** @param array<string, int|string|null> $values */
function nonShortCircuiting(array $values, string $key): void
{
    requireNonNull($values[$key]) xor requireNullableString($values[$key]);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function negated(array $values, string $key): void
{
    !requireNonNull($values[$key]);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function assertionCondition(array $values, string $key): void
{
    if (assert(is_string($values[$key]))) {
        \PHPStan\dumpType($values[$key]);
    } else {
        \PHPStan\dumpType($values[$key]);
    }
}

/** @param array<string, int|string|null> $values */
function assertionLeftOperand(array $values, string $key): void
{
    assert(is_string($values[$key])) && ($observed = $values[$key]);
    \PHPStan\dumpType($observed);
    echo $observed;
}

/** @phpstan-assert string $value */
function providerRequiresString(mixed $value): bool { return true; }

function providerDirect(int|string $value): void
{
    providerRequiresString($value);
    \PHPStan\dumpType($value); // provider observation
}

function providerDisjunction(bool $pick, int|string $value): void
{
    $pick || providerRequiresString($value);
    \PHPStan\dumpType($value); // provider observation
}

function providerConjunction(bool $pick, int|string $value): void
{
    $pick && providerRequiresString($value);
    \PHPStan\dumpType($value); // provider observation
}
