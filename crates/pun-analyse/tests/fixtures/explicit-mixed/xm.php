<?php

class XmBase
{
    public function push(): mixed
    {
        return 1;
    }
}

final class ExplicitMixed extends XmBase
{
    public mixed $prop;

    public $untypedProp;

    public function m(): mixed
    {
        return 1;
    }

    public function n()
    {
        return 1;
    }

    public static function sm(): mixed
    {
        return 1;
    }

    public function run(mixed $m, $u, string $s, \ReflectionAttribute $attribute)
    {
        $a = fn () => $this->m();
        $b = fn () => $this->n();
        $c = fn () => json_decode($s);
        $d = fn () => $m;
        $e = fn () => $u;
        $f = fn () => $this->prop;
        $g = fn () => $this->untypedProp;
        $h = fn () => parent::push();
        $i = fn () => static::sm();
        $j = fn () => self::sm();
        $k = fn () => $m ?? 1;
        $l = fn () => $this->m() ?: null;
        $n = fn () => $attribute->getArguments();
        $o = fn ($x) => $x;
        $p = fn (mixed $x) => $x;
        $q = fn () => (string) $m;
        $r = fn () => $m['k'];
        $t = fn () => $m->x;
        $v = fn () => $m->call();
        $w = fn () => unserialize($s);
        $x = fn () => array_pop($s);
    }
}
