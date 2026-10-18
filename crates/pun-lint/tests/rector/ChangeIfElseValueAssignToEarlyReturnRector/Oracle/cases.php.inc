<?php

function notLast($a)
{
    if ($a) {
        $x = 1;
    } else {
        echo 'else';
        $x = 2;
    }
    // about the return
    return $x;
    echo 'unreachable';
}

function colon($a)
{
    if ($a):
        $x = 1;
    else:
        foo();
        $x = 2;
    endif;
    return $x;
}

function bare($a)
{
    if ($a) $x = 1; else $x = 2;
    return $x;
}

function differentVar($a)
{
    if ($a) {
        $x = 1;
    } else {
        $y = 2;
    }
    return $x;
}
-----
<?php

function notLast($a)
{
    if ($a) {
        return 1;
    }
    return 2;
    echo 'else';
    echo 'unreachable';
}

function colon($a)
{
    if ($a):
        return 1;
    endif;
    foo();
    return 2;
}

function bare($a)
{
    if ($a) return 1;
    return 2;
}

function differentVar($a)
{
    if ($a) {
        $x = 1;
    } else {
        $y = 2;
    }
    return $x;
}
