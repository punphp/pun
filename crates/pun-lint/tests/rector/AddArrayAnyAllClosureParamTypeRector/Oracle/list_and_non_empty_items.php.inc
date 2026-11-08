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
        array_any($a, function ($x) {
            return $x !== [];
        });
        array_any($b, function ($x) {
            return $x !== [];
        });
        array_any($c, function ($x) {
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
        array_any($a, function (array $x) {
            return $x !== [];
        });
        array_any($b, function (array $x) {
            return $x !== [];
        });
        array_any($c, function (array $x) {
            return $x !== [];
        });
    }
}
