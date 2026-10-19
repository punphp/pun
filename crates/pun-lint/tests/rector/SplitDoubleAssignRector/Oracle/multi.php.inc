<?php

function a()
{
    $x = 1;

    // first comment
    $a = $b = 5;

    $c = $d = $e = foo();
    $f = $g = new Foo();
    $h = $i = $j->call();
}

function b()
{
    $a = $b = 1;
    echo 1;
}

function c()
{
    $a = $b = 1;
}

if ($x) $a = $b = 2;

$arr[1] = $b = 3;
$a = $arr[] = 4;
$a = $b += 5;
$a = $b = (7);
$o->p = Foo::$q = bar();
-----
<?php

function a()
{
    $x = 1;
    // first comment
    $a = 5;
    $b = 5;
    $c = foo();
    $d = $c;
    $e = $c;
    $f = new Foo();
    $g = $f;
    $h = $j->call();
    $i = $h;
}

function b()
{
    $a = 1;
    $b = 1;
    echo 1;
}

function c()
{
    $a = 1;
    $b = 1;
}

if ($x) {
    $a = 2;
    $b = 2;
}

$arr[1] = $b = 3;
$a = $arr[] = 4;
$a = $b += 5;
$a = 7;
$b = 7;
$o->p = bar();
Foo::$q = $o->p;
