<?php

namespace App;

final class Fin {}
class Open {}
trait T {}
enum E { case A; }

class Child extends Open
{
    /**
     * @param Open $doc
     * @param object $odoc
     */
    public function run(object $o, $doc, $odoc, E $e, \Closure $cl, iterable $it, Open|Fin $u)
    {
        $a = is_a($o, fin::class);
        $b = is_a($o, T::class);
        $c = is_a($o, E::class);
        $d = is_a($o, parent::class);
        $f = is_a($doc, Open::class);
        $g = is_a($odoc, Open::class);
        $h = is_a($e, E::class);
        $i = is_a(E::A, E::class);
        $j = is_a($cl, Open::class);
        $k = is_a($u, Open::class);
        $l = is_a($o, \Closure::class);
        $m = is_a($o, \stdClass::class);
        $n = is_a($o, $this->cls);
        $p = is_a($o, self::$scls);
        $q = is_a($o, $arr['x']);
        $r = is_a($o, Fin::class) && is_a($o, Open::class);
        $s = is_a($o = new Open(), Open::class);
        $t = is_a($o, Fin::class) ? 1 : 2;
        $v = is_a($o, Fin::class) . 'x';
        $w = (string) is_a(clone $o, Fin::class);
        $x = is_a($o?->x ?? $o, Fin::class);
        $y = is_a(match(true) { default => $o }, Fin::class);
        $z = is_a(new class {}, Fin::class);
        return [$a];
    }
}
-----
<?php

namespace App;

final class Fin {}
class Open {}
trait T {}
enum E { case A; }

class Child extends Open
{
    /**
     * @param Open $doc
     * @param object $odoc
     */
    public function run(object $o, $doc, $odoc, E $e, \Closure $cl, iterable $it, Open|Fin $u)
    {
        $a = is_a($o, fin::class);
        $b = $o instanceof \App\T;
        $c = $o instanceof \App\E;
        $d = $o instanceof \App\Open;
        $f = $doc instanceof \App\Open;
        $g = $odoc instanceof \App\Open;
        $h = $e instanceof \App\E;
        $i = E::A instanceof \App\E;
        $j = $cl instanceof \App\Open;
        $k = is_a($u, Open::class);
        $l = $o instanceof \Closure;
        $m = $o instanceof \stdClass;
        $n = is_a($o, $this->cls);
        $p = is_a($o, self::$scls);
        $q = is_a($o, $arr['x']);
        $r = $o instanceof \App\Fin && $o instanceof \App\Open;
        $s = ($o = new Open()) instanceof \App\Open;
        $t = $o instanceof \App\Fin ? 1 : 2;
        $v = $o instanceof \App\Fin . 'x';
        $w = (string) (clone $o instanceof \App\Fin);
        $x = is_a($o?->x ?? $o, Fin::class);
        $y = match(true) { default => $o } instanceof \App\Fin;
        $z = new class {} instanceof \App\Fin;
        return [$a];
    }
}
