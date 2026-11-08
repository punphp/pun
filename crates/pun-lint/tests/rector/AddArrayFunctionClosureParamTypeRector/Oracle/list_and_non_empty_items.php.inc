<?php

namespace Probe;

final class Items
{
    /**
     * @param  array<int, list<int>>  $a
     * @param  array<int, array<int, int>>  $b
     * @param  array<int, non-empty-array<int, int>>  $c
     */
    public function run(array $a, array $b, array $c)
    {
        array_filter($a, function ($x) {
            return $x !== [];
        });
        array_filter($b, function ($x) {
            return $x !== [];
        });
        array_filter($c, function ($x) {
            return $x !== [];
        });
    }
}
-----
<?php

namespace Probe;

final class Items
{
    /**
     * @param  array<int, list<int>>  $a
     * @param  array<int, array<int, int>>  $b
     * @param  array<int, non-empty-array<int, int>>  $c
     */
    public function run(array $a, array $b, array $c)
    {
        array_filter($a, function (array $x) {
            return $x !== [];
        });
        array_filter($b, function (array $x) {
            return $x !== [];
        });
        array_filter($c, function (array $x) {
            return $x !== [];
        });
    }
}
