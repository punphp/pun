<?php

final class ExplicitMixedUnions
{
    public function m(): mixed
    {
        return 1;
    }

    public function run(mixed $m, $u, bool $c, array $arr)
    {
        $a = fn () => $u ?? $m;
        $b = fn () => $m ?? $u;
        $d = fn () => 1 ?? $m;
        $e = fn () => $c ? $m : $u;
        $f = fn () => $c ? $u : $m;
        $g = fn () => $c ? $m : 1;
        $h = fn () => $u ?: $m;
        $i = fn () => $m ?: $u;
        $j = fn () => $arr['k'];
        $k = fn () => $m[0][1];
        $l = fn () => $this->m()['x'];
        $n = fn () => match ($c) { true => $m, false => 1 };
        $o = fn () => $m ?? null;
        $p = fn () => @$m;
        $q = fn () => ($m);
        $r = fn () => clone $m;
        $s = fn () => $m = 1;
        $y = $m;
        $t = fn () => $y;
        $m = 5;
        $v = fn () => $m;
        $w = function () use ($u) { return $u ?? $this->m(); };
    }

    public function later(mixed $z)
    {
        if ($z) {
            $z = 1;
        }

        return fn () => $z;
    }
}
