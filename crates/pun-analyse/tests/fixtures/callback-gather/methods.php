<?php declare(strict_types=1);

namespace CallbackGatherMethods;

final class Mapper
{
    private const KEY = 17;

    private function label(): string
    {
        return 'mapper';
    }

    /**
     * @template V
     * @template RK of array-key
     * @template RV
     * @param array<V> $source
     * @param callable(V): array{RK, RV} $transform
     * @return array<RK, RV>
     */
    public function pairs(array $source, callable $transform): array
    {
        $result = [];
        foreach ($source as $value) {
            [$key, $mapped] = $transform($value);
            $result[$key] = $mapped;
        }
        return $result;
    }

    public function examples(): void
    {
        $result = $this->pairs(transform: function (array $entry): array {
            \PHPStan\dumpType($entry);
            return [$entry[0], $entry[0]];
        }, source: [[1], [2]]);
        \PHPStan\dumpType($result);
        \PHPStan\dumpType($this->pairs([[1]], fn (array $entry): array => [self::KEY, $this->label()]));
    }
}
