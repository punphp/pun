<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function g2(): mixed
{
    return 1;
}

class Closures
{
    public function run($k, $arr, $items, mixed $x)
    {
        $f = function ($p) use ($arr) {
            array_key_exists($p, $arr);
            $q = $p;
            array_key_exists($q, $arr);
            array_key_exists($p->x, $arr);
            array_key_exists($p->m(), $arr);
            array_key_exists(g2($p), $arr);
            array_key_exists($p ?? 1, $arr);
            array_key_exists($this->undefinedProperty, $arr);
            array_key_exists($zz, $arr);
        };
        $f2 = function () use ($k, $arr) {
            array_key_exists($k, $arr);
            if ($k !== null) {
                array_key_exists($k, $arr);
            }
        };
        $f3 = function () use ($arr) {
            array_key_exists($items, $arr);
            array_key_exists($x, $arr);
            array_key_exists($this, $arr);
        };
        $g = fn ($p) => array_key_exists($p, $arr);
        $h = fn () => array_key_exists($items, $arr);
        $l = fn () => array_key_exists($later, $arr);
        $later = 1;
        $i = fn () => array_key_exists($k2 ?? null, $arr);
        $j = fn () => $k !== null ? array_key_exists($k, $arr) : 1;
        $w = fn (mixed $q) => $q !== null ? array_key_exists($q, $arr) : 1;
        if ($x !== null) {
            $n = fn () => array_key_exists($x, $arr);
        }
        $o = function () use ($arr) {
            $c = function ($p) use ($arr) {
                array_key_exists($p, $arr);
            };
        };
    }

    public static function stat($arr)
    {
        $f = function () use ($arr) {
            array_key_exists($this, $arr);
        };
    }
}

function outer($arr, $known)
{
    $pre = 1;
    preg_match('/a/', 'a', $matches);
    $f = function () use ($arr) {
        array_key_exists($known, $arr);
        array_key_exists($pre, $arr);
        array_key_exists($matches, $arr);
        array_key_exists($post, $arr);
    };
    $post = 2;
}

$cl = function ($p) use ($arr) {
    array_key_exists($p, $arr);
};
$v = 5;
$cl2 = fn () => array_key_exists($v, $arr);
-----
<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function g2(): mixed
{
    return 1;
}

class Closures
{
    public function run($k, $arr, $items, mixed $x)
    {
        $f = function ($p) use ($arr) {
            array_key_exists($p, $arr);
            $q = $p;
            array_key_exists($q, $arr);
            array_key_exists($p->x, $arr);
            array_key_exists($p->m(), $arr);
            array_key_exists(g2($p), $arr);
            array_key_exists($p ?? 1, $arr);
            array_key_exists($this->undefinedProperty, $arr);
            array_key_exists($zz, $arr);
        };
        $f2 = function () use ($k, $arr) {
            array_key_exists($k, $arr);
            if ($k !== null) {
                array_key_exists($k, $arr);
            }
        };
        $f3 = function () use ($arr) {
            array_key_exists($items, $arr);
            array_key_exists($x, $arr);
            array_key_exists($this, $arr);
        };
        $g = fn ($p) => array_key_exists($p, $arr);
        $h = fn () => array_key_exists($items, $arr);
        $l = fn () => array_key_exists($later, $arr);
        $later = 1;
        $i = fn () => array_key_exists('', $arr);
        $j = fn () => $k !== null ? array_key_exists($k, $arr) : 1;
        $w = fn (mixed $q) => $q !== null ? array_key_exists($q, $arr) : 1;
        if ($x !== null) {
            $n = fn () => array_key_exists($x, $arr);
        }
        $o = function () use ($arr) {
            $c = function ($p) use ($arr) {
                array_key_exists($p, $arr);
            };
        };
    }

    public static function stat($arr)
    {
        $f = function () use ($arr) {
            array_key_exists($this, $arr);
        };
    }
}

function outer($arr, $known)
{
    $pre = 1;
    preg_match('/a/', 'a', $matches);
    $f = function () use ($arr) {
        array_key_exists($known, $arr);
        array_key_exists($pre, $arr);
        array_key_exists($matches, $arr);
        array_key_exists($post, $arr);
    };
    $post = 2;
}

$cl = function ($p) use ($arr) {
    array_key_exists($p, $arr);
};
$v = 5;
$cl2 = fn () => array_key_exists($v, $arr);
