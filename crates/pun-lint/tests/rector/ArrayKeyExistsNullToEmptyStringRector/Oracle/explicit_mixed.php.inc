<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function g(): mixed
{
    return 1;
}

/** @return mixed */
function h()
{
    return 1;
}

function u()
{
    return 1;
}

class ExplicitMixed
{
    public mixed $p;
    public $q;
    public static $sp;
    public static mixed $sp2;

    public function m(): mixed
    {
        return 1;
    }

    public function run($k, $j, $arr, mixed $x)
    {
        $v = g();
        if ($v !== null) {
            array_key_exists($v, $arr);
        }
        $w = h();
        if ($w !== null) {
            array_key_exists($w, $arr);
        }
        $z = u();
        if ($z !== null) {
            array_key_exists($z, $arr);
        }
        if ($this->p !== null) {
            array_key_exists($this->p, $arr);
        }
        if ($this->q !== null) {
            array_key_exists($this->q, $arr);
        }
        if ($this->m() !== null) {
            array_key_exists($this->m(), $arr);
        }
        if (g() !== null) {
            array_key_exists(g(), $arr);
        }
        if (self::$sp !== null) {
            array_key_exists(self::$sp, $arr);
        }
        if (self::$sp2 !== null) {
            array_key_exists(self::$sp2, $arr);
        }
        if (isset($k)) {
            array_key_exists($k, $arr);
        }
        if ($k) {
            array_key_exists($k, $arr);
        }
        if (!is_null($k)) {
            array_key_exists($k, $arr);
        }
        if ($x !== null) {
            array_key_exists($x, $arr);
        }
        $y = $x;
        if ($y !== null) {
            array_key_exists($y, $arr);
        }
        $a = $k;
        $a = $x;
        if ($a !== null) {
            array_key_exists($a, $arr);
        }
        $b = $x;
        $b = $k;
        if ($b !== null) {
            array_key_exists($b, $arr);
        }
        if ($j === null) {
            return;
        }
        array_key_exists($j, $arr);
    }
}
-----
<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function g(): mixed
{
    return 1;
}

/** @return mixed */
function h()
{
    return 1;
}

function u()
{
    return 1;
}

class ExplicitMixed
{
    public mixed $p;
    public $q;
    public static $sp;
    public static mixed $sp2;

    public function m(): mixed
    {
        return 1;
    }

    public function run($k, $j, $arr, mixed $x)
    {
        $v = g();
        if ($v !== null) {
            array_key_exists((string) $v, $arr);
        }
        $w = h();
        if ($w !== null) {
            array_key_exists($w, $arr);
        }
        $z = u();
        if ($z !== null) {
            array_key_exists($z, $arr);
        }
        if ($this->p !== null) {
            array_key_exists($this->p, $arr);
        }
        if ($this->q !== null) {
            array_key_exists($this->q, $arr);
        }
        if ($this->m() !== null) {
            array_key_exists((string) $this->m(), $arr);
        }
        if (g() !== null) {
            array_key_exists((string) g(), $arr);
        }
        if (self::$sp !== null) {
            array_key_exists(self::$sp, $arr);
        }
        if (self::$sp2 !== null) {
            array_key_exists(self::$sp2, $arr);
        }
        if (isset($k)) {
            array_key_exists($k, $arr);
        }
        if ($k) {
            array_key_exists((string) $k, $arr);
        }
        if (!is_null($k)) {
            array_key_exists($k, $arr);
        }
        if ($x !== null) {
            array_key_exists((string) $x, $arr);
        }
        $y = $x;
        if ($y !== null) {
            array_key_exists((string) $y, $arr);
        }
        $a = $k;
        $a = $x;
        if ($a !== null) {
            array_key_exists((string) $a, $arr);
        }
        $b = $x;
        $b = $k;
        if ($b !== null) {
            array_key_exists($b, $arr);
        }
        if ($j === null) {
            return;
        }
        array_key_exists($j, $arr);
    }
}
