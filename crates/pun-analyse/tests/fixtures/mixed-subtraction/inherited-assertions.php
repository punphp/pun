<?php

namespace MixedInheritedAssertions;

class ParentPredicate
{
    public function guessed(mixed $value): bool
    {
        return \is_string($value);
    }

    public static function guessedStatic(mixed $value): bool
    {
        return \is_int($value);
    }
}

class ChildPredicate extends ParentPredicate
{
    public function guessed(mixed $value): bool
    {
        return \is_int($value);
    }

    public static function guessedStatic(mixed $value): bool
    {
        return true;
    }
}

function invoke(ChildPredicate $predicate, mixed $value): void
{
    if ($predicate->guessed($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if (ChildPredicate::guessedStatic($value)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}
