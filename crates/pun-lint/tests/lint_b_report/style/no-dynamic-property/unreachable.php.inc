<?php

namespace App;

class Box
{
    public static $p;
}

function f(string $m, Box $b)
{
    $b->{$m};
    return;
    $b->$m(...);
    Box::$m(...);
    Box::$$m;
    Box::${$m};
    $b->$m;
}
-----
12: Variable property access on App\Box.
