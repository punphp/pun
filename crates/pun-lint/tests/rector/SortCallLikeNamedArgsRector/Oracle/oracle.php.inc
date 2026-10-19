<?php

namespace App;

function run($foo = null, $bar = null, $baz = null) {}

class Service
{
    public function __construct($a = 1, $b = 2, $c = 3) {}

    public function call($x = null, $y = null, $z = null)
    {
        $this->call(z: 1, x: 2);
        self::make(second: 1, first: 2);
        static::make(second: 1, first: 2);
        $other = new self(c: 3, a: 1);
        $other->call(y: 1, x: 2);
        return new Service(
            c: 3,
            b: 2,
            a: 1,
        );
    }

    public static function make($first = null, $second = null) {}
}

class Child extends Service
{
    public function go()
    {
        parent::make(second: 1, first: 2);
        $this->call(z: 1, unknown: 2, x: 3);
        $s = new Service();
        $s->call(z: 1, y: 2);
        run(...[1, 2], baz: 3, foo: 4);
        str_replace(subject: 'a', search: 'b', replace: 'c');
        \App\run(baz: 1, bar: 2);
        unknown_fn(b: 1, a: 2);
        $f->call(z: 1, x: 2);
        Missing::call(z: 1, x: 2);
        run(1, 2);
        $fn(z: 1, x: 2);
        $this->$name(z: 1, x: 2);
        self::$name(second: 1, first: 2);
        $plain = new Service;
        $dyn = new $cls(c: 3, a: 1);
        $either = rand() ? new Service() : new Child();
        $either->call(z: 1, x: 2);
        run(foo: 1, bar: 2);
    }
}

#[\Attribute]
class Attr
{
    public function __construct(public $one = null, public $two = null) {}
}

#[Attr(two: 2, one: 1)]
#[Attr(
    two: 2,
    one: 1,
)]
#[Attr(1, two: 2)]
#[Attr(two: 2)]
#[Missing(two: 2, one: 1)]
#[Attr]
class Annotated {}
-----
<?php

namespace App;

function run($foo = null, $bar = null, $baz = null) {}

class Service
{
    public function __construct($a = 1, $b = 2, $c = 3) {}

    public function call($x = null, $y = null, $z = null)
    {
        $this->call(x: 2, z: 1);
        self::make(first: 2, second: 1);
        static::make(first: 2, second: 1);
        $other = new self(a: 1, c: 3);
        $other->call(x: 2, y: 1);
        return new Service(
            a: 1,
            b: 2,
            c: 3,
        );
    }

    public static function make($first = null, $second = null) {}
}

class Child extends Service
{
    public function go()
    {
        parent::make(first: 2, second: 1);
        $this->call(x: 3, z: 1, unknown: 2);
        $s = new Service();
        $s->call(y: 2, z: 1);
        run(...[1, 2], foo: 4, baz: 3);
        str_replace(search: 'b', replace: 'c', subject: 'a');
        \App\run(bar: 2, baz: 1);
        unknown_fn(b: 1, a: 2);
        $f->call(z: 1, x: 2);
        Missing::call(z: 1, x: 2);
        run(1, 2);
        $fn(z: 1, x: 2);
        $this->$name(z: 1, x: 2);
        self::$name(second: 1, first: 2);
        $plain = new Service;
        $dyn = new $cls(c: 3, a: 1);
        $either = rand() ? new Service() : new Child();
        $either->call(z: 1, x: 2);
        run(foo: 1, bar: 2);
    }
}

#[\Attribute]
class Attr
{
    public function __construct(public $one = null, public $two = null) {}
}

#[Attr(two: 2, one: 1)]
#[Attr(
    two: 2,
    one: 1,
)]
#[Attr(1, two: 2)]
#[Attr(two: 2)]
#[Missing(two: 2, one: 1)]
#[Attr]
class Annotated {}
