<?php
declare(strict_types=1);

namespace ArrayMapAcceptorValues;

/**
 * @template T
 * @param T $value
 * @return T
 */
function identity(mixed $value): mixed { return $value; }

/**
 * @template T
 * @param T $value
 * @return array{T}
 */
function wrap(mixed $value): array { return [$value]; }

/** @return ($value is int ? int : bool) */
function conditional(mixed $value): int|bool { return is_int($value) ? $value : false; }

function probe(): void
{
    \PHPStan\dumpType(array_map(identity(...), [1, 2]));
    \PHPStan\dumpType(array_map(__NAMESPACE__ . '\\identity', [3, 4]));
    \PHPStan\dumpType(array_map(wrap(...), [5, 6]));
    \PHPStan\dumpType(array_map(conditional(...), [7, 8]));
    \PHPStan\dumpType(array_map(conditional(...), ['x', 'y']));
    $callback = identity(...);
    \PHPStan\dumpType(array_map($callback, [9, 10]));
    $callback = wrap(...);
    \PHPStan\dumpType(array_map($callback, [11, 12]));
    \PHPStan\dumpType(array_map(null, [13, 14]));
}
