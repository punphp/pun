<?php declare(strict_types = 1);

namespace StrictConditions;

enum Suit: string
{
    case Hearts = 'h';
}

class Casts
{
    /**
     * @param int $documented
     * @param non-empty-string $nonEmpty
     * @param list<int> $list
     * @param int<0, 10> $range
     */
    public function casts(int $int, string $string, float $float, bool $bool, array $array, object $object, ?int $nullable, $documented, string $nonEmpty, array $list, int $range, int|string $union, mixed $mixed): void
    {
        $values = [
            (int) $int,
            (string) $string,
            (float) $float,
            (bool) $bool,
            (array) $array,
            (object) $object,
            (int) $nullable,
            (int) $documented,
            (string) $nonEmpty,
            (array) $list,
            (int) $range,
            (int) $union,
            (int) $mixed,
            (int) 5,
            (string) 'literal',
            (bool) true,
            (float) 1.5,
            (int) '5',
            (string) 5,
            (string) Suit::Hearts->value,
            (array) [1, 2],
            (integer) $int,
            (boolean) $bool,
            (double) $float,
            (binary) $string,
            (string) \PHP_EOL,
            (int) \PHP_INT_MAX,
        ];
        (void) $int;
        echo \count($values);
    }
}
