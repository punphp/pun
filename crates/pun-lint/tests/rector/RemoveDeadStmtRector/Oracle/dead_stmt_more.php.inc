<?php

class A
{
    public $plain;

    public static $static;

    public int $hooked {
        get => 1;
    }

    public function __construct(public int $promoted)
    {
    }

    public function run($name)
    {
        $this->plain;
        $this->hooked;
        $this->promoted;
        $this->undeclared;
        $this->$name;
        self::$static;
        static::$static;
        self::$$name;
        $this?->plain;
    }

    public int $value {
        get {
            1;
            return 1;
        }
    }
}

trait T
{
    public $plain;

    public function run()
    {
        $this->plain;
    }
}

enum E
{
    public function run()
    {
        self::$static;
    }
}

function outside($name)
{
    $this->plain;
    $object->{$name};
    $object->{foo()};
    $object->$name;
    ${foo()};
    $$name;
    A::{foo()};
    $a::$static;
    (foo());
    -(foo());
    "string";
    `ls`;
    __LINE__;
    -foo();
    ~foo();
}

function bodies($x)
{
    while ($x)
        foo() + 1;
    /** @var int $y */
    1;
}

class C implements ArrayAccess
{
    public function offsetExists($o): bool
    {
        return true;
    }

    public function offsetGet($o): mixed
    {
        return 1;
    }

    public function offsetSet($o, $v): void
    {
    }

    public function offsetUnset($o): void
    {
    }

    public function run(C $a, self $c, ?int $h, int|string $i, array $m)
    {
        $a[0];
        $c[0];
        $h[0];
        $i[0];
        $m[0];
        clone $h;
    }
}

class D extends C
{
    public function run2(parent $p)
    {
        $p[0];
    }
}
-----
<?php

class A
{
    public $plain;

    public static $static;

    public int $hooked {
        get => 1;
    }

    public function __construct(public int $promoted)
    {
    }

    public function run($name)
    {
        $this->undeclared;
        $this->$name;
        self::$$name;
        $this?->plain;
    }

    public int $value {
        get {
            return 1;
        }
    }
}

trait T
{
    public $plain;

    public function run()
    {
        $this->plain;
    }
}

enum E
{
    public function run()
    {
        self::$static;
    }
}

function outside($name)
{
    $this->plain;
    $object->{$name};
    $object->{foo()};
    $object->$name;
    foo();
    foo();
    $a::$static;
    (foo());
    foo();
    `ls`;
    foo();
    foo();
}

function bodies($x)
{
    while ($x)
        foo();
    /** @var int $y */

}

class C implements ArrayAccess
{
    public function offsetExists($o): bool
    {
        return true;
    }

    public function offsetGet($o): mixed
    {
        return 1;
    }

    public function offsetSet($o, $v): void
    {
    }

    public function offsetUnset($o): void
    {
    }

    public function run(C $a, self $c, ?int $h, int|string $i, array $m)
    {
        $a[0];
        $c[0];
    }
}

class D extends C
{
    public function run2(parent $p)
    {
        $p[0];
    }
}
