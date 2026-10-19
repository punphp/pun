<?php

function f18($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', 'x')) {
        foo();
    }
}

function f2($x, $sign, $v)
{
    if (foo() === version_compare(PHP_VERSION, '7.0', '>=')) {
        foo();
    }
}

function f12($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', $sign)) {
        foo();
    }
}
