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
    \Closure::fromCallable('x')->$m();
}
-----
25: Variable static method call on N\Foo.
26: Variable static method call on N\Foo.
27: Variable static method call on N\Foo.
28: Variable static method call on N\Bar.
29: Variable static method call on N\Foo.
37: Variable static method call on N\Foo.
