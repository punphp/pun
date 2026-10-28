<?php

namespace Probe;

final class Reduce
{
    /**
     * @param list<int> $numbers
     */
    public function noInitial(array $numbers)
    {
        return array_reduce($numbers, function ($carry, $number) {
            return $carry + $number;
        });
    }

    /**
     * @param list<int> $numbers
     */
    public function arrow(array $numbers)
    {
        return array_reduce($numbers, fn ($carry, $number) => $carry + $number, 0);
    }

    public function untypedArray($numbers)
    {
        return array_reduce($numbers, function ($carry, $number) {
            return 1;
        }, 0);
    }

    /**
     * @param list<int> $numbers
     */
    public function declaredReturn(array $numbers)
    {
        return array_reduce($numbers, function ($carry, $number): int {
            return $carry + $number;
        }, 0);
    }

    /**
     * @param list<\DateTime> $dates
     */
    public function objects(array $dates)
    {
        return \array_reduce($dates, function ($carry, $date) {
            return $date;
        }, null);
    }

    public function literal()
    {
        return array_reduce(['a', 'b'], function ($carry, $item) {
            return $carry . $item;
        }, '');
    }
}
-----
<?php

namespace Probe;

final class Reduce
{
    /**
     * @param list<int> $numbers
     */
    public function noInitial(array $numbers)
    {
        return array_reduce($numbers, function ($carry, int $number) {
            return $carry + $number;
        });
    }

    /**
     * @param list<int> $numbers
     */
    public function arrow(array $numbers)
    {
        return array_reduce($numbers, fn ($carry, $number) => $carry + $number, 0);
    }

    public function untypedArray($numbers)
    {
        return array_reduce($numbers, function (int $carry, $number) {
            return 1;
        }, 0);
    }

    /**
     * @param list<int> $numbers
     */
    public function declaredReturn(array $numbers)
    {
        return array_reduce($numbers, function (int $carry, int $number): int {
            return $carry + $number;
        }, 0);
    }

    /**
     * @param list<\DateTime> $dates
     */
    public function objects(array $dates)
    {
        return \array_reduce($dates, function (?\DateTime $carry, \DateTime $date) {
            return $date;
        }, null);
    }

    public function literal()
    {
        return array_reduce(['a', 'b'], function (string $carry, string $item) {
            return $carry . $item;
        }, '');
    }
}
