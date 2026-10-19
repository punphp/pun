<?php

function a($param)
{
    $r = foo('a');
    $r = compact(...$param);
    $r = compact($param);
    $r = compact(array('a', 'b'));
}

function b($x)
{
    $list = ['a', 'b'];
    if ($x) {
        $list[] = foo();
    }
    $r = compact($list);
}

function c($x)
{
    $list = ['a', 'b'];
    if ($x) {
        $list[0] = 'c';
    }
    $r = compact($list);
}

function d($items)
{
    $list = ['a', 'b'];
    foreach ($items as $list) {
    }
    $r = compact($list);
}

function e($x)
{
    $list = ['a', 'b'];
    if ($x) {
        $list = &$x;
    }
    $r = compact($list);
}

function f()
{
    $list = ['a', 'b'];
    $r = compact($list);
}

$closure = function () {
    $list = ['a'];
    $r = compact($list);
};

class A
{
    public function m()
    {
        $list = ['a', 'b'];
        $other = &$x;
        $r = compact($list);
    }
}
-----
<?php

function a($param)
{
    $r = foo('a');
    $r = compact(...$param);
    $r = compact($param);
    $r = ['a' => $a, 'b' => $b];
}

function b($x)
{
    $list = ['a', 'b'];
    if ($x) {
        $list[] = foo();
    }
    $r = compact($list);
}

function c($x)
{
    $list = ['a', 'b'];
    if ($x) {
        $list[0] = 'c';
    }
    $r = compact($list);
}

function d($items)
{
    $list = ['a', 'b'];
    foreach ($items as $list) {
    }
    $r = compact($list);
}

function e($x)
{
    $list = ['a', 'b'];
    if ($x) {
        $list = &$x;
    }
    $r = compact($list);
}

function f()
{
    $list = ['a', 'b'];
    $r = ['a' => $a, 'b' => $b];
}

$closure = function () {
    $list = ['a'];
    $r = ['a' => $a];
};

class A
{
    public function m()
    {
        $list = ['a', 'b'];
        $other = &$x;
        $r = ['a' => $a, 'b' => $b];
    }
}
