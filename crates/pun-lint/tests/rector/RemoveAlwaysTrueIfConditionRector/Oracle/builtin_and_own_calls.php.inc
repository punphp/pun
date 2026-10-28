<?php

namespace Oracle\BuiltinCalls;

function a(array $items)
{
    if (is_array($items)) {
        echo 9;
    }
}

function b(int $n)
{
    if (is_int($n)) {
        echo 9;
    }
}

function c(array $items)
{
    if (is_array($items) && $items) {
        echo 9;
    }
}

function d(int $n)
{
    if (\is_int($n)) {
        echo 10;
    }
}

function own(): true
{
    return true;
}

function e()
{
    if (own()) {
        echo 11;
    }
}
-----
<?php

namespace Oracle\BuiltinCalls;

function a(array $items)
{
    if (is_array($items)) {
        echo 9;
    }
}

function b(int $n)
{
    if (is_int($n)) {
        echo 9;
    }
}

function c(array $items)
{
    if ($items) {
        echo 9;
    }
}

function d(int $n)
{
    if (\is_int($n)) {
        echo 10;
    }
}

function own(): true
{
    return true;
}

function e()
{
    echo 11;
}
