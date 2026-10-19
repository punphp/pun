<?php

class Foo
{
    public function with(): self
    {
        return $this;
    }
}

abstract class Bar
{
}

function a()
{
    $x = new Foo();
    return $x;
}

function b(Foo $x)
{
    $x = $x->with();
    return $x;
}

function c(Bar $x)
{
    $x = 1;
    return $x;
}

function d(?Foo $x)
{
    $x = 1;
    return $x;
}

function e(Foo $x)
{
    $x .= 'a';
    return $x;
}

function f()
{
    $x = new Foo();
    $x = $x->with();
    return $x;
}

function g(\DateTime $x)
{
    $x = 1;
    return $x;
}
-----
<?php

class Foo
{
    public function with(): self
    {
        return $this;
    }
}

abstract class Bar
{
}

function a()
{
    return new Foo();
}

function b(Foo $x)
{
    $x = $x->with();
    return $x;
}

function c(Bar $x)
{
    return 1;
}

function d(?Foo $x)
{
    return 1;
}

function e(Foo $x)
{
    $x .= 'a';
    return $x;
}

function f()
{
    $x = new Foo();
    $x = $x->with();
    return $x;
}

function g(\DateTime $x)
{
    $x = 1;
    return $x;
}
