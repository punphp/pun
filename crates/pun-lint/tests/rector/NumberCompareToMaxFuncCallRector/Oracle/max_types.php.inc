<?php

class A
{
    public function i(): int
    {
        return 1;
    }

    public static function s(): int
    {
        return 1;
    }

    public function run(int $p, $q, string $str)
    {
        $a = $p > -1 ? $p : -1;
        $b = $p > +1 ? $p : +1;
        $c = $p > -$q ? $p : -$q;
        $d = (int) $q > $p ? (int) $q : $p;
        $e = (integer) $q > $p ? (integer) $q : $p;
        $f = strlen($str) > $p ? strlen($str) : $p;
        $g = $this->i() > $p ? $this->i() : $p;
        $h = self::s() > $p ? self::s() : $p;
        $i = static::s() > $p ? static::s() : $p;
        $j = $q > $p ? $q : $p;
        $k = $p ?: 1;
        $l = $p == 1 ? $p : 1;
        $m = foo() ? $p : 1;
        $n = (string) $q > $p ? (string) $q : $p;
        $o = $p < 1 ? 2 : $p;
    }
}

trait T
{
    public function run(int $p)
    {
        return $p > 1 ? $p : 1;
    }
}

function locals(int $p)
{
    $x = 5;
    return $x > $p ? $x : $p;
}
-----
<?php

class A
{
    public function i(): int
    {
        return 1;
    }

    public static function s(): int
    {
        return 1;
    }

    public function run(int $p, $q, string $str)
    {
        $a = max($p, -1);
        $b = max($p, +1);
        $c = $p > -$q ? $p : -$q;
        $d = max((int) $q, $p);
        $e = max((integer) $q, $p);
        $f = max(strlen($str), $p);
        $g = max($this->i(), $p);
        $h = max(self::s(), $p);
        $i = max(static::s(), $p);
        $j = $q > $p ? $q : $p;
        $k = $p ?: 1;
        $l = $p == 1 ? $p : 1;
        $m = foo() ? $p : 1;
        $n = (string) $q > $p ? (string) $q : $p;
        $o = $p < 1 ? 2 : $p;
    }
}

trait T
{
    public function run(int $p)
    {
        return $p > 1 ? $p : 1;
    }
}

function locals(int $p)
{
    $x = 5;
    return max($x, $p);
}
