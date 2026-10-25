<?php declare(strict_types=1);

namespace CallableOriginAssignments;

use function PHPStan\dumpType;

/**
 * @param callable(string &): string $reference
 * @param callable(string): string $value
 */
function assignments(callable $reference, callable $value, callable $unknown): void
{
    $copy = $reference;
    dumpType($copy);
    'text' |> $copy;
    $reference = $value;
    dumpType($reference);
    'text' |> $reference;
    $copy = fn(string $input) => 'literal';
    dumpType($copy);
    'text' |> $copy;
    $reference = $unknown;
    'text' |> $reference;
}

/** @param-out callable(string): string $callback */
function replace(callable &$callback): void { $callback = static fn(string $input): string => $input; }

/** @param callable(string &): string $reference */
function invalidation(callable $reference): void
{
    replace($reference);
    dumpType($reference);
    'text' |> $reference;
}

/** @return callable(int): int */
function identity(): callable { return static fn(int $input): int => $input; }

/** @param callable(string &): string $reference */
function returnedAssignment(callable $reference): void
{
    $reference = identity();
    dumpType($reference);
    1 |> $reference;
}
