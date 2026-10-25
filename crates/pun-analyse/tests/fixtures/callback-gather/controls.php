<?php declare(strict_types=1);

namespace CallbackGatherControls;

/**
 * @template T
 * @template U
 * @param list<T> $source
 * @param callable(T): U $callback
 * @return list<U>
 */
function mapValues(array $source, callable $callback): array
{
    return array_map($callback, $source);
}

/** @param list<int> $items */
function examples(array $items): void
{
    \PHPStan\dumpType(array_map(fn (int $value): int => $value, $items));
    $result = mapValues([1, 2, 3], function ($value) {
        \PHPStan\dumpType($value);
        \PHPStan\dumpType(isset($value));
        return $value;
    });
    \PHPStan\dumpType($result);
    $fixed = 19;
    \PHPStan\dumpType(mapValues([1, 2], fn ($value) => $fixed));
    $observed = mapValues([[1], [2]], function (array $entry): array {
        if ($entry) {
            $present = 1;
        }
        \PHPStan\dumpType(isset($present));
        return [$entry[0], $entry[0]];
    });
    \PHPStan\dumpType($observed);
    \PHPStan\dumpType(mapValues([[1], [2]], fn (array $entry): array => array_slice([10 => 1], 0)));
}
