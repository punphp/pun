<?php

function comments()
{
    // the default
    $var = null;
    if (rand(0, 1)) {
        // one
        $var = 1;
    }
    return $var;
}

function loopClosure()
{
    $var = null;
    foreach ([1] as $x) {
        $f = function () { $var = 2; };
    }
    if (rand(0, 1)) {
        $var = 1;
    }
    return $var;
}

function interpolated()
{
    $var = 'a';
    if (rand(0, 1)) {
        $var = "x{$var}";
    }
    return $var;
}

function bareIf()
{
    $var = 0;
    if (rand(0, 1)) $var = 1;
    return $var;
}
-----
<?php

function comments()
{
    if (rand(0, 1)) {
        return 1;
    }
    return null;
}

function loopClosure()
{
    $var = null;
    foreach ([1] as $x) {
        $f = function () { $var = 2; };
    }
    if (rand(0, 1)) {
        $var = 1;
    }
    return $var;
}

function interpolated()
{
    $var = 'a';
    if (rand(0, 1)) {
        $var = "x{$var}";
    }
    return $var;
}

function bareIf()
{
    if (rand(0, 1)) return 1;
    return 0;
}
