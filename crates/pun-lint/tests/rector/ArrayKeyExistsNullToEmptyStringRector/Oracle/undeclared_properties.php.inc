<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

class P2
{
    public $p;
    public static $sp;
}

final class Undeclared
{
    public function run(P2 $pl, $arr)
    {
        array_key_exists($pl->nope, $arr);
        array_key_exists($this->nope, $arr);
        array_key_exists(self::$nope, $arr);
        array_key_exists(P2::$nope, $arr);
        array_key_exists(P2::$sp, $arr);
        array_key_exists(Missing::$sp, $arr);
        $f = function () use ($arr, $pl) {
            array_key_exists($pl->nope, $arr);
            array_key_exists($pl->p, $arr);
        };
        $g = fn () => array_key_exists($this->nope, $arr);
        $h = fn ($a) => fn () => array_key_exists($a, $arr);
        $i = function ($b) use ($arr) {
            $j = fn () => array_key_exists($b, $arr);
            $k = fn () => array_key_exists($c, $arr);
        };
    }
}
-----
<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

class P2
{
    public $p;
    public static $sp;
}

final class Undeclared
{
    public function run(P2 $pl, $arr)
    {
        array_key_exists($pl->nope, $arr);
        array_key_exists($this->nope, $arr);
        array_key_exists(self::$nope, $arr);
        array_key_exists(P2::$nope, $arr);
        array_key_exists((string) P2::$sp, $arr);
        array_key_exists(Missing::$sp, $arr);
        $f = function () use ($arr, $pl) {
            array_key_exists($pl->nope, $arr);
            array_key_exists((string) $pl->p, $arr);
        };
        $g = fn () => array_key_exists($this->nope, $arr);
        $h = fn ($a) => fn () => array_key_exists((string) $a, $arr);
        $i = function ($b) use ($arr) {
            $j = fn () => array_key_exists($b, $arr);
            $k = fn () => array_key_exists($c, $arr);
        };
    }
}
