<?php

function compound($x)
{
    $items = [1];
    $items += [2];
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function destructured($x)
{
    $items = [1];
    [$items] = [[1]];
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function referenced($x)
{
    $items = [1];
    $r = &$items;
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function foreach_target($x)
{
    $items = [1];
    foreach ([[1]] as $items) {
    }
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}

function captured($x)
{
    $items = [1];
    $f = function () use (&$items) {
    };
    foreach ($items as $item) {
        if ($item === $x) {
            return true;
        }
    }
    return false;
}
