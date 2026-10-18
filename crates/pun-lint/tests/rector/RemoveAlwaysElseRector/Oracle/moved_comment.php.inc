<?php

function a($a)
{
    if ($a) {
        return 1;
    } else {
        echo 2;
        // trailing
    }
}

function b($a)
{
    if ($a) {
        return 1;
    } else {
        echo 2;
        // trailing
    }
    echo 3;
}

function c($a)
{
    if ($a) {
        return 1;
    } else {
        // only
    }
}

function d($a)
{
    foreach ($a as $b) {
        if ($b) {
            return 1;
        } else {
            echo 2;
            /* block */
        }
    }
}
-----
<?php

function a($a)
{
    if ($a) {
        return 1;
    }
    echo 2;
    // trailing

}

function b($a)
{
    if ($a) {
        return 1;
    }
    echo 2;
    // trailing

    echo 3;
}

function c($a)
{
    if ($a) {
        return 1;
    }
    // only

}

function d($a)
{
    foreach ($a as $b) {
        if ($b) {
            return 1;
        }
        echo 2;
        /* block */

    }
}
