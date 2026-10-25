<?php declare(strict_types=1);

namespace CallbackObserver;

/**
 * @template T
 * @template U
 * @param list<T> $items
 * @param callable(T): U $callback
 * @return list<U>
 */
function mapValues(array $items, callable $callback): array
{
    return array_map($callback, $items);
}

/** @param list<array> $unknown */
function observe(array $unknown): void
{
    $result = mapValues([[1], [2]], static function (array $entry): int {
        foreach ($entry as $value) {
            $present = 1;
        }
        return $present ?? 23;
    });

    $possible = mapValues($unknown, static function (array $entry): int {
        foreach ($entry as $value) {
            $present = 1;
        }
        return $present ?? 37;
    });
}
