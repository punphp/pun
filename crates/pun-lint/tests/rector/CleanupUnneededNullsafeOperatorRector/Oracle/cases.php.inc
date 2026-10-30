<?php

namespace App;

class Foo
{
    public function bar(): Foo { return $this; }
    public function nullable(): ?Foo { return null; }
    public function st(): static { return $this; }
    public function se(): self { return $this; }
    public function obj(): object { return $this; }
    public function mixed(): mixed { return $this; }
    public function untyped() { return $this; }
    public static function make(): Foo { return new Foo(); }
    /** @return Foo */
    public function doc() { return $this; }
    public function e(): E { return E::A; }
    public function cl(): \Closure { return fn() => 1; }
}

enum E { case A; public function x() {} }

function make(): Foo { return new Foo(); }

class C extends Foo
{
    public function run(Foo $foo, ?Foo $n, \DateTime $dt)
    {
        $foo->bar()?->bar();
        $foo->nullable()?->bar();
        $foo->st()?->bar();
        $foo->se()?->bar();
        $foo->obj()?->bar();
        $foo->mixed()?->bar();
        $foo->untyped()?->bar();
        Foo::make()?->bar();
        make()?->bar();
        $foo->doc()?->bar();
        $foo->e()?->x();
        $foo->cl()?->call($this);
        $this->st()?->bar();
        $n->bar()?->bar();
        $n?->bar()?->bar();
        $dt->modify('+1 day')?->format('Y');
        \date_create()?->format('Y');
        $foo->bar()?->bar(
            1,
            2,
        );
        $foo->bar()?->bar(/* c */ 1);
        $foo->bar()?->bar()?->bar()?->bar();
        $foo->bar()?->{'bar'}();
        ($foo->bar())?->bar();
        $foo->bar()?->prop;
        self::make()?->bar();
        static::make()?->bar();
        parent::make()?->bar();
        $foo->bar() /* x */ ?-> /* y */ bar();
        $foo
            ->bar()
            ?->bar();
    }
}
-----
<?php

namespace App;

class Foo
{
    public function bar(): Foo { return $this; }
    public function nullable(): ?Foo { return null; }
    public function st(): static { return $this; }
    public function se(): self { return $this; }
    public function obj(): object { return $this; }
    public function mixed(): mixed { return $this; }
    public function untyped() { return $this; }
    public static function make(): Foo { return new Foo(); }
    /** @return Foo */
    public function doc() { return $this; }
    public function e(): E { return E::A; }
    public function cl(): \Closure { return fn() => 1; }
}

enum E { case A; public function x() {} }

function make(): Foo { return new Foo(); }

class C extends Foo
{
    public function run(Foo $foo, ?Foo $n, \DateTime $dt)
    {
        $foo->bar()->bar();
        $foo->nullable()?->bar();
        $foo->st()->bar();
        $foo->se()->bar();
        $foo->obj()?->bar();
        $foo->mixed()?->bar();
        $foo->untyped()?->bar();
        Foo::make()->bar();
        make()->bar();
        $foo->doc()?->bar();
        $foo->e()->x();
        $foo->cl()->call($this);
        $this->st()->bar();
        $n->bar()?->bar();
        $n?->bar()?->bar();
        $dt->modify('+1 day')->format('Y');
        \date_create()?->format('Y');
        $foo->bar()->bar(1, 2);
        $foo->bar()->bar(
            /* c */
            1
        );
        $foo->bar()->bar()->bar()->bar();
        $foo->bar()?->{'bar'}();
        $foo->bar()->bar();
        $foo->bar()?->prop;
        self::make()->bar();
        static::make()->bar();
        parent::make()->bar();
        $foo->bar()->bar();
        $foo
            ->bar()->bar();
    }
}
