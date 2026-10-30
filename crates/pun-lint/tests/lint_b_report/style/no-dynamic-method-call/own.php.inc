<?php

namespace N;

class Foo
{
    /** @var Foo */
    public $doc;

    public function run($m, Foo $foo, ?Foo $maybe, Foo|\stdClass $either, int $i, string $s, array $a, self $me, \Closure $c)
    {
        $this->$m();
        $foo->$m();
        $maybe->$m();
        $either->$m();
        $i->$m();
        $s->$m();
        $a->$m();
        $me->$m();
        $m->$m();
        $undefined->$m();
        $this->doc->$m();
        (new Foo())->$m();
        $c->$m();
        static::$m();
        self::$m();
        Foo::$m();
        Bar::$m();
        $foo::$m();
        $this->{'x'}();
        $this->{$m . 'x'}();
        $this?->$m();
    }

    public static function st($m)
    {
        static::$m();
        $x = new static();
        $x->$m();
    }
}

function f($m) {
    $f = function () use ($m) { $this->$m(); };
    \Closure::fromCallable('strlen')->$m();
}
-----
12: Variable method call on $this(N\Foo).
13: Variable method call on N\Foo.
14: Variable method call on N\Foo|null.
15: Variable method call on N\Foo|stdClass.
16: Variable method call on int.
17: Variable method call on string.
18: Variable method call on array.
19: Variable method call on N\Foo.
20: Variable method call on mixed.
21: Variable method call on mixed.
22: Variable method call on N\Foo.
23: Variable method call on N\Foo.
24: Variable method call on Closure.
30: Variable method call on $this(N\Foo).
31: Variable method call on $this(N\Foo).
32: Variable method call on $this(N\Foo).
39: Variable method call on static(N\Foo).
44: Variable method call on mixed.
45: Variable method call on Closure.
