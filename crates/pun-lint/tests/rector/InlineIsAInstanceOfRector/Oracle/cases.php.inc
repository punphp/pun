<?php

namespace App;

final class Fin {}
class Open {}
interface Iface {}

class C
{
    public function run(object $o, ?object $n, Open $open, \Stringable $s, $mixed, Open&Iface $both, string $cls, self $me)
    {
        $a = is_a($o, Fin::class);
        $b = is_a($o, Open::class);
        $c = is_a($o, Unknown::class);
        $d = is_a($o, self::class);
        $e = is_a($o, static::class);
        $f = is_a($n, Open::class);
        $g = is_a($open, Iface::class);
        $h = is_a($s, Open::class);
        $i = is_a($mixed, Open::class);
        $j = is_a($both, Open::class);
        $k = is_a($this, Open::class);
        $l = is_a($o, $cls);
        $m = is_a($o, $open::class);
        $n2 = is_a($me, Open::class);
        $q = is_a(new Open(), Iface::class);
        $r = is_a($o, Open::class, true);
        $s2 = is_a($o, parent::class);
        $t = is_a($o, \DateTime::class);
        $u = is_a($o, Iface::CLASS);
        $v = IS_A($o, Iface::class);
        $w = \is_a($o, Iface::class);
        $x = is_a(...);
        $y = is_a($o, $$cls);
        $z = is_a(object: $o, class: Iface::class);
        return [$a];
    }
}
-----
<?php

namespace App;

final class Fin {}
class Open {}
interface Iface {}

class C
{
    public function run(object $o, ?object $n, Open $open, \Stringable $s, $mixed, Open&Iface $both, string $cls, self $me)
    {
        $a = $o instanceof \App\Fin;
        $b = $o instanceof \App\Open;
        $c = is_a($o, Unknown::class);
        $d = $o instanceof \App\C;
        $e = $o instanceof \App\C;
        $f = is_a($n, Open::class);
        $g = $open instanceof \App\Iface;
        $h = $s instanceof \App\Open;
        $i = is_a($mixed, Open::class);
        $j = is_a($both, Open::class);
        $k = is_a($this, Open::class);
        $l = $o instanceof $cls;
        $m = is_a($o, $open::class);
        $n2 = $me instanceof \App\Open;
        $q = new Open() instanceof \App\Iface;
        $r = $o instanceof \App\Open;
        $s2 = is_a($o, parent::class);
        $t = $o instanceof \DateTime;
        $u = $o instanceof \App\Iface;
        $v = $o instanceof \App\Iface;
        $w = $o instanceof \App\Iface;
        $x = is_a(...);
        $y = $o instanceof $$cls;
        $z = $o instanceof \App\Iface;
        return [$a];
    }
}
