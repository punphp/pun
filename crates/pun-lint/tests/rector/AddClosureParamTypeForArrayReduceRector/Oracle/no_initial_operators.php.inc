<?php
namespace Probe;
final class R
{
    /** @param list<int> $n */
    public function plus(array $n) { return array_reduce($n, function ($carry, $x) { return $carry + $x; }); }
    /** @param list<int> $n */
    public function minus(array $n) { return array_reduce($n, function ($carry, $x) { return $carry - $x; }); }
    /** @param list<int> $n */
    public function item(array $n) { return array_reduce($n, function ($carry, $x) { return $x + 1; }); }
    /** @param list<int> $n */
    public function plusOne(array $n) { return array_reduce($n, function ($carry, $x) { return $carry + 1; }); }
}
-----
<?php
namespace Probe;
final class R
{
    /** @param list<int> $n */
    public function plus(array $n) { return array_reduce($n, function ($carry, int $x) { return $carry + $x; }); }
    /** @param list<int> $n */
    public function minus(array $n) { return array_reduce($n, function (float|int $carry, int $x) { return $carry - $x; }); }
    /** @param list<int> $n */
    public function item(array $n) { return array_reduce($n, function (int $carry, int $x) { return $x + 1; }); }
    /** @param list<int> $n */
    public function plusOne(array $n) { return array_reduce($n, function (int|float $carry, int $x) { return $carry + 1; }); }
}
