<?php

namespace App;

class NoParent
{
    public function f(string $m)
    {
        parent::$m();
        PARENT::$m();
    }
}

function outside(string $m)
{
    self::$m();
    static::$m();
    parent::$m();
    SELF::$m();
    Parent::$m();
}

class Kid extends NoParent
{
    public function g(string $m)
    {
        Parent::$m();
        Static::$m();
    }
}

function h(string $m)
{
    return;
    $m->$m();
    \App\Kid::$m();
}
-----
9: Variable static method call on parent.
10: Variable static method call on PARENT.
16: Variable static method call on self.
17: Variable static method call on static.
18: Variable static method call on parent.
19: Variable static method call on SELF.
20: Variable static method call on Parent.
27: Variable static method call on App\NoParent.
28: Variable static method call on App\Kid.
