<?php

function names($x, $b, $f)
{
    $x?->foo->bar();
    $x?->foo->baz();
    $y = 1;
    A::$$b = 1;
    A::$$b = 2;
    $z = 1;
    ($f)()->x();
    ($f)()->y();
    $w = 1;
    $x->{$b} = 1;
    $x->{$b} = 2;
    $v = 1;
}
-----
<?php

function names($x, $b, $f)
{
    $x?->foo->bar();
    $x?->foo->baz();

    $y = 1;
    A::$$b = 1;
    A::$$b = 2;
    $z = 1;
    ($f)()->x();
    ($f)()->y();
    $w = 1;
    $x->{$b} = 1;
    $x->{$b} = 2;

    $v = 1;
}
