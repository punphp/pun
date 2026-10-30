<?php

namespace App;

function none() {}

function two($a, $b = 1) {}

function variadic($a, ...$rest) {}

function reads() { return func_num_args(); }

trait Helps
{
    public function help($a) {}
}

class Base
{
    use Helps;

    public function one($a) {}

    public static function make() {}

    protected function zero() {}
}

class Child extends Base
{
    public function run()
    {
        $this->one(1, 2);
        $this->zero(1);
        $this->help(1, 2, 3);
        static::make(1);
        self::make(1, 2);
        Base::make(1);
        parent::one(1, 2);
    }
}

enum Suit
{
    case Hearts;

    public function color($a) {}
}

function f(?Base $n, Base|Child $u, Base|int $bi, Child $c, $m, array $args)
{
    none(1);
    none(1, 2);
    \App\none(1);
    two(1, 2, 3);
    two(
        1,
        2,
        3,
    );
    two(1, 2, ...$args);
    two(a: 1, b: 2, c: 3);
    variadic(1, 2, 3);
    reads(1, 2);
    $n->one(1, 2);
    $n?->one(1, 2);
    $u->one(1, 2);
    $bi->one(1, 2);
    $c->one(1, 2);
    $c->one(...$args);
    Suit::Hearts->color(1, 2);
    $m(1, 2);
    $c->$m(1, 2);
    strlen('a', 'b');
    str_replace('a', 'b', 'c', $count, 1);
    array_merge([], [], []);
    sprintf('%s', 1, 2);
    printf('%s', 1, 2);
    in_array(1, [], true, 4);
    count([], 0, 1);
    unknown_function(1, 2);
    $f = none(...);
}
-----
<?php

namespace App;

function none() {}

function two($a, $b = 1) {}

function variadic($a, ...$rest) {}

function reads() { return func_num_args(); }

trait Helps
{
    public function help($a) {}
}

class Base
{
    use Helps;

    public function one($a) {}

    public static function make() {}

    protected function zero() {}
}

class Child extends Base
{
    public function run()
    {
        $this->one(1);
        $this->zero();
        $this->help(1);
        static::make();
        self::make();
        Base::make();
        parent::one(1, 2);
    }
}

enum Suit
{
    case Hearts;

    public function color($a) {}
}

function f(?Base $n, Base|Child $u, Base|int $bi, Child $c, $m, array $args)
{
    none();
    none();
    \App\none();
    two(1, 2);
    two(
        1,
        2,
    );
    two(1, 2);
    two(a: 1, b: 2);
    variadic(1, 2, 3);
    reads(1, 2);
    $n->one(1, 2);
    $n?->one(1, 2);
    $u->one(1);
    $bi->one(1, 2);
    $c->one(1);
    $c->one(...$args);
    Suit::Hearts->color(1);
    $m(1, 2);
    $c->$m(1, 2);
    strlen('a');
    str_replace('a', 'b', 'c', $count);
    array_merge([], [], []);
    sprintf('%s', 1, 2);
    printf('%s', 1, 2);
    in_array(1, [], true);
    count([], 0);
    unknown_function(1, 2);
    $f = none(...);
}
