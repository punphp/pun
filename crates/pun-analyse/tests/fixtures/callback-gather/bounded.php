<?php declare(strict_types=1);

namespace CallbackGatherBounded;

/**
 * @template T of array{int}
 * @template U
 * @param T $input
 * @param callable(T): U $callback
 * @return U
 */
function firstField(array $input, callable $callback)
{
    return $callback($input);
}

function examples(): void
{
    $result = firstField([11], function (array $entry): int {
        \PHPStan\dumpType($entry);
        return $entry[0];
    });
    \PHPStan\dumpType($result);
    \PHPStan\dumpType(firstField([11], fn (array $entry): int => $entry[0]));
    \PHPStan\dumpType(firstField([11], fn (): int => 23));
}
