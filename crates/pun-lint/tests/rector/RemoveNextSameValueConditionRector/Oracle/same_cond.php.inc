<?php

function a($items)
{
    if ($items === []) {
        $count = 0;
    }
    if ($items === []) {
        return $count;
    }
}

function b($items)
{
    if ($items === []) $count = 0;

    /** doc */
    if ($items === []) {
        echo 1;
        return $count;
    }
}

function c($x)
{
    if (foo($x)) {
        echo 1;
    }
    if (foo($x)) {
        echo 2;
    }
}

function e($x)
{
    if ($x) {
    }
    if ($x) {
        echo 2; // trailing
    }
}
-----
<?php

function a($items)
{
    if ($items === []) {
        $count = 0;
        return $count;
    }
}

function b($items)
{
    /** doc */
    if ($items === []) {
        $count = 0;
        echo 1;
        return $count;
    }
}

function c($x)
{
    if (foo($x)) {
        echo 1;
    }
    if (foo($x)) {
        echo 2;
    }
}

function e($x)
{
    if ($x) {
        echo 2;
        // trailing
    }
}
