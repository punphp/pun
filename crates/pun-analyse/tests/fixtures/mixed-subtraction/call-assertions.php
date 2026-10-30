<?php

namespace MixedCallAssertions;

function guessedString(mixed $value): bool
{
    return \is_string($value);
}

function guessedNotInt(mixed $value): bool
{
    return !\is_int($value);
}

function guessedNull(mixed $value): bool
{
    return $value === null;
}

function undocumented(mixed $value): void
{
    if (guessedString($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if (guessedNotInt($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if (guessedNull($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if (guessedString($copy = $value)) {
        \PHPStan\dumpType($copy);
        \PHPStan\dumpType($value);
    }
}

/** @phpstan-assert-if-true string $value */
function declaredTrue(mixed $value): bool
{
    return \is_string($value);
}

/** @phpstan-assert-if-false int $value */
function declaredFalse(mixed $value): bool
{
    return !\is_int($value);
}

/** @phpstan-assert string $value */
function requireString(mixed $value): void
{
    if (!is_string($value)) {
        throw new \InvalidArgumentException();
    }
}

function documented(mixed $value): void
{
    if (declaredTrue($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if (declaredFalse($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    requireString($value);
    \PHPStan\dumpType($value);
}

class Predicate
{
    public function guessed(mixed $value): bool
    {
        return \is_string($value);
    }

    public static function guessedStatic(mixed $value): bool
    {
        return \is_int($value);
    }

    /** @phpstan-assert-if-true string $value */
    public function declared(mixed $value): bool
    {
        return \is_string($value);
    }
}

class InheritedPredicate extends Predicate
{
    public function declared(mixed $value): bool
    {
        return \is_string($value);
    }
}

function methods(Predicate $predicate, InheritedPredicate $child, mixed $value): void
{
    if ($predicate->guessed($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if (Predicate::guessedStatic($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if ($predicate->declared($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if ($child->declared($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}

function closures(mixed $value): void
{
    $arrow = static fn (mixed $argument): bool => \is_string($argument);
    if ($arrow($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    $closure = static function (mixed $argument): bool {
        return \is_int($argument);
    };
    if ($closure($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}

function guessedReference(mixed &$value): bool
{
    return \is_string($value);
}

function references(): void
{
    if (guessedReference($new)) {
        \PHPStan\dumpType($new);
    } else {
        \PHPStan\dumpType($new);
    }
    \PHPStan\dumpType($new);
}
