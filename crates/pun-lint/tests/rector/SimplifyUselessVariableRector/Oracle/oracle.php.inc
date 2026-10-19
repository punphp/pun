<?php

class Foo
{
    public function with(): self
    {
        return $this;
    }
}

final class Single
{
    private function __construct()
    {
    }

    public static function make(): self
    {
        $instance = new self();
        return $instance;
    }
}

interface I
{
}

function a()
{
    $x = new Foo();
    return $x;
}

function g(\DateTime $x)
{
    $x = 1;
    return $x;
}

function h(I $x)
{
    $x = 1;
    return $x;
}

function i(Single $x)
{
    $x = 2;
    return $x;
}

function &j()
{
    $x = 1;
    return $x;
}

function k(&$x)
{
    $x = 1;
    return $x;
}

function l()
{
    global $x;
    if (true) {
        $x = 1;
        return $x;
    }
}

function m()
{
    if (true) {
        static $y;
    }
    $y = 1;
    return $y;
}

function n()
{
    $f = function () use (&$z) {
    };
    $z = 1;
    return $z;
}

function o()
{
    $z = 1;
    return $z;
    $f = function () use (&$z) {
    };
}

function p()
{
    // comment
    $x = 1;
    return $x;
}

function q()
{
    $x = 1;
    /** @var int $x */
    return $x;
}

function r()
{
    $x = 1;
    /** @var mixed $x */
    return $x;
}

function s()
{
    $x = (1 + 2);
    return $x;
}

function t($a)
{
    $x ??= $a;
    return $x;
}

function u($a, $b)
{
    $x -= $a - $b;
    return $x;
}

function v($a, $b)
{
    $x .= ($a ? $b : 'c');
    return $x;
}

function w()
{
    $x = &$y;
    return $x;
}

function x()
{
    echo 1;
    return 2;
    $x = 1;
    return $x;
}

function y()
{
    $x[0] = 1;
    return $x;
}

function z()
{
    $x = 1;
    return $y;
}

function aa()
{
    $x = 1;
    return;
}

function bb()
{
    if (true):
        $x = 1;
        return $x;
    endif;
    switch (1) {
        case 1:
            $y = 1;
            return $y;
    }
}

function cc()
{
    $x = 1;
    // after
    return $x;
}

function dd()
{
    $x = new Foo();
    $f = fn () => $x;
    $x = 3;
    return $x;
}

function ops($a)
{
    $f = function () use ($a) {
        $b *= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b /= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b %= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b &= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b |= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b ^= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b <<= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b >>= $a;
        return $b;
    };
    $f = function () use ($a) {
        $b **= $a + 1;
        return $b;
    };
    $f = function () use ($a) {
        $b += $a;
        return $b;
    };
    $g = fn () => function () {
        $c = 1;
        return $c;
    };
    $h = fn (&$d) => function () {
        $d = 1;
        return $d;
    };
    $n = 'name';
    $$n = 1;
    return $$n;
}

class Hooks
{
    public $p {
        &get {
            if (true) {
                $y = 1;
                return $y;
            }
        }
    }
}

$top = 1;
$top = $top + 1;
return $top;
-----
<?php

class Foo
{
    public function with(): self
    {
        return $this;
    }
}

final class Single
{
    private function __construct()
    {
    }

    public static function make(): self
    {
        return new self();
    }
}

interface I
{
}

function a()
{
    return new Foo();
}

function g(\DateTime $x)
{
    $x = 1;
    return $x;
}

function h(I $x)
{
    return 1;
}

function i(Single $x)
{
    return 2;
}

function &j()
{
    $x = 1;
    return $x;
}

function k(&$x)
{
    $x = 1;
    return $x;
}

function l()
{
    global $x;
    if (true) {
        $x = 1;
        return $x;
    }
}

function m()
{
    if (true) {
        static $y;
    }
    return 1;
}

function n()
{
    $f = function () use (&$z) {
    };
    $z = 1;
    return $z;
}

function o()
{
    return 1;
    $f = function () use (&$z) {
    };
}

function p()
{
    // comment
    $x = 1;
    return $x;
}

function q()
{
    $x = 1;
    /** @var int $x */
    return $x;
}

function r()
{
    /** @var mixed $x */
    return 1;
}

function s()
{
    return 1 + 2;
}

function t($a)
{
    $x ??= $a;
    return $x;
}

function u($a, $b)
{
    return $x - ($a - $b);
}

function v($a, $b)
{
    $x .= ($a ? $b : 'c');
    return $x;
}

function w()
{
    $x = &$y;
    return $x;
}

function x()
{
    echo 1;
    return 2;
    $x = 1;
    return $x;
}

function y()
{
    $x[0] = 1;
    return $x;
}

function z()
{
    $x = 1;
    return $y;
}

function aa()
{
    $x = 1;
    return;
}

function bb()
{
    if (true):
        return 1;
    endif;
    switch (1) {
        case 1:
            return 1;
    }
}

function cc()
{
    // after
    return 1;
}

function dd()
{
    $x = new Foo();
    $f = fn () => $x;
    $x = 3;
    return $x;
}

function ops($a)
{
    $f = function () use ($a) {
        return $b * $a;
    };
    $f = function () use ($a) {
        return $b / $a;
    };
    $f = function () use ($a) {
        return $b % $a;
    };
    $f = function () use ($a) {
        return $b & $a;
    };
    $f = function () use ($a) {
        return $b | $a;
    };
    $f = function () use ($a) {
        return $b ^ $a;
    };
    $f = function () use ($a) {
        return $b << $a;
    };
    $f = function () use ($a) {
        return $b >> $a;
    };
    $f = function () use ($a) {
        return $b ** ($a + 1);
    };
    $f = function () use ($a) {
        return $b + $a;
    };
    $g = fn () => function () {
        return 1;
    };
    $h = fn (&$d) => function () {
        $d = 1;
        return $d;
    };
    $n = 'name';
    return 1;
}

class Hooks
{
    public $p {
        &get {
            if (true) {
                $y = 1;
                return $y;
            }
        }
    }
}

$top = 1;
return $top + 1;
