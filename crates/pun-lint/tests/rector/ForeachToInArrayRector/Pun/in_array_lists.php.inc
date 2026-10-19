<?php

function after(array $items, $x)
{
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
    echo 1;
}

function comment(array $items, $x)
{
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
    // trailing
}

function otherwise(array $items, $x)
{
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        } else {
            echo 1;
        }
    }
    return false;
}

if ($a)
    foreach ([1, 2] as $item) {
        if ($item === $x) {
            return true;
        }
    }
