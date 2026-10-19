<?php

function a($x)
{
    // keep
    if ($x) {
        return 1;
    }
    return 1;
}

function b($x)
{
    $y = 1;
    if ($x->call()) {
        return 1;
    }

    return 1;
}

function c($x)
{
    if ($x) return 2;
    return 2;
}

function d($x)
{
    if (count($x)) {
        // comment only
    }

    return 3;
}

function f($x)
{
    if ($x):
        return 5;
    endif;
    return 5;
}

function g($x)
{
    if ($x === foo()) {
        return 6;
    }
    return 6;
}
-----
<?php

function a($x)
{
    return 1;
}

function b($x)
{
    $y = 1;
    if ($x->call()) {
        return 1;
    }

    return 1;
}

function c($x)
{
    return 2;
}

function d($x)
{
    if (count($x)) {
        // comment only
    }

    return 3;
}

function f($x)
{
    return 5;
}

function g($x)
{
    if ($x === foo()) {
        return 6;
    }
    return 6;
}
