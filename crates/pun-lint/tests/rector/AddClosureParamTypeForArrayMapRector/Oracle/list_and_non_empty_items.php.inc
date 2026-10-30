<?php

namespace Probe;

final class Items
{
    /**
     * @param  array<int, list<int>>  $a
     * @param  array<int, array<int, int>>  $b
     * @param  array<int, non-empty-array<int, int>>  $c
     * @param  array<int, list<int>|null>  $d
     * @param  array<int, int[]>  $e
     * @param  list<array{int, string}>  $f
     */
    public function run(array $a, array $b, array $c, array $d, array $e, array $f)
    {
        array_map(function ($x) {
            return $x;
        }, $a);
        array_map(function ($x) {
            return $x;
        }, $b);
        array_map(function ($x) {
            return $x;
        }, $c);
        array_map(function ($x) {
            return $x;
        }, $d);
        array_map(function ($x) {
            return $x;
        }, $e);
        array_map(function ($x) {
            return $x;
        }, $f);
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
     * @param  array<int, list<int>|null>  $d
     * @param  array<int, int[]>  $e
     * @param  list<array{int, string}>  $f
     */
    public function run(array $a, array $b, array $c, array $d, array $e, array $f)
    {
        array_map(function ($x) {
            return $x;
        }, $a);
        array_map(function (array $x) {
            return $x;
        }, $b);
        array_map(function ($x) {
            return $x;
        }, $c);
        array_map(function ($x) {
            return $x;
        }, $d);
        array_map(function (array $x) {
            return $x;
        }, $e);
        array_map(function (array $x) {
            return $x;
        }, $f);
    }
}
