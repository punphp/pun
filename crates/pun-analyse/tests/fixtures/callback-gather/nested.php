<?php declare(strict_types=1);

namespace CallbackGatherNested;

/**
 * @template T
 * @param callable(array): T $callback
 * @return T
 */
function forward(array $input, callable $callback)
{
    return $callback($input);
}

function examples(): void
{
    \PHPStan\dumpType(forward([[1]], fn (array $a): array =>
        forward($a, fn (array $b): array =>
            forward($b, fn (array $c): array => [11, 'x']))));
    \PHPStan\dumpType(forward([[1]], fn (array $a) =>
        forward($a, fn (array $b) =>
            forward($b, fn (array $c) => [11, 'x']))));
}
