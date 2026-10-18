<?php

function f($a)
{
    if ($a) return 1; else return 1;
}

function g($a)
{
    if ($a) {
        // one
        return 1;
    } else {
        // two
        return 1;
    }
}

function h($a)
{
    if ($a) {
        echo 1;
        echo 2;
    } else if ($a > 1) {
        echo 1;
        echo 2;
    } else {
        echo 1;
        echo 2;
    }
}

function i($a)
{
    if ($a):
        echo 1;
    else:
        echo 1;
    endif;
    while ($a)
        if ($a) { echo 3; } else { echo 3; }
}
-----
<?php

function f($a)
{
    return 1;
}

function g($a)
{
    if ($a) {
        // one
        return 1;
    } else {
        // two
        return 1;
    }
}

function h($a)
{
    echo 1;
    echo 2;
}

function i($a)
{
    echo 1;
    while ($a)
        echo 3;
}
