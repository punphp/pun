<?php

function e($x, $y)
{
    if ($x !== null) {
        return $y;
    }
    if ($x !== null) {
        return $x;
    }
    return false;
}

function f($x)
{
    if ($x != null) {
        return $x;
    }
    return null;
}

function g($x)
{
    if ($x !== null) {
        return $x;
        // trailing
    }
    return null;
}

function h($x)
{
    if ($x !== null) {
        return;
    }
    return null;
}

function i($x)
{
    if ($x !== null) {
        return $x;
    }
    echo 1;
}

function j($x)
{
    if ($x !== null) {
        return $x;
    }
    return;
}

function k($x)
{
    if ($x !== 1) {
        return $x;
    }
    return null;
}

function l($x)
{
    if ($x !== null) {
        return $x;
    }
}
