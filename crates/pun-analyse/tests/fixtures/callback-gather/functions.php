<?php declare(strict_types=1);

namespace CallbackGather;

/**
 * @template K of array-key
 * @template V
 * @template RK of array-key
 * @template RV
 * @param array<K, V> $source
 * @param callable(V, K): array{RK, RV} $transform
 * @return array<RK, RV>
 */
function collectPairs(array $source, callable $transform): array
{
    $result = [];
    foreach ($source as $key => $value) {
        [$newKey, $newValue] = $transform($value, $key);
        $result[$newKey] = $newValue;
    }
    return $result;
}

function examples(): void
{
    $result = collectPairs([[1], [2], [3]], function (array $entry): array {
        \PHPStan\dumpType($entry);
        [$key] = $entry;
        \PHPStan\dumpType($key);
        return [$key, $key];
    });
    \PHPStan\dumpType($result);
    $copy = $result;
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType(collectPairs([[1], [2]], fn (array $entry) => [$entry[0], $entry[0]]));
    \PHPStan\dumpType(collectPairs([[1], [2]], fn (array $entry): array => [$entry[0], $entry[0]]));
    \PHPStan\dumpType(collectPairs([[1], [2]], function (array $entry) { return [$entry[0], $entry[0]]; }));
    \PHPStan\dumpType(collectPairs([[1], [2]], fn (array $entry): array => [11, 'x']));
    $fixed = 11;
    \PHPStan\dumpType(collectPairs([[1], [2]], fn (array $entry) => [$fixed, $fixed]));
    \PHPStan\dumpType(collectPairs([[1], [2]], function (array $entry) use ($fixed): array { return [$fixed, $fixed]; }));
    \PHPStan\dumpType(collectPairs([[1], [2]], (/** @return array{int, int} */ function (array $entry): array { return [$entry[0], $entry[0]]; })));
    \PHPStan\dumpType(collectPairs([[1], [2]], (/** @return array{11, 'x'} */ fn (array $entry): array => [11, 'x'])));
}
