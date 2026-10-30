<?php

namespace App;

function bar(): ?\stdClass
{
    return null;
}

function plain()
{
    $value = bar();
    if (! $value instanceof \stdClass) {
        return null;
    }

    return $value;
}

function nested()
{
    if (rand(0, 1)) {
        $value = bar();
        if (! ($value instanceof \stdClass)) {
            return null;
        }

        return $value;
    }

    foreach ([1] as $x) {
        $value = bar();
        if ($value instanceof \stdClass) {
            return $value;
        }
        return null;
    }
}

function parenthesized()
{
    $value = (bar());
    if (! $value instanceof \stdClass) {
        return null;
    }

    return $value;
}

function differentCase()
{
    $value = bar();
    if (! $value instanceof \STDCLASS) {
        return null;
    }

    return $value;
}

function byReference()
{
    $value = &$GLOBALS['x'];
    $value = bar();
    if (! $value instanceof \stdClass) {
        return null;
    }

    return $value;
}

function elseBranch()
{
    $value = bar();
    if (! $value instanceof \stdClass) {
        return null;
    } else {
        $x = 1;
    }

    return $value;
}

$closure = function () {
    $value = bar();
    if (! $value instanceof \stdClass) {
        return null;
    }

    return $value;
};
-----
<?php

namespace App;

function bar(): ?\stdClass
{
    return null;
}

function plain()
{
    return bar();
}

function nested()
{
    if (rand(0, 1)) {
        return bar();
    }

    foreach ([1] as $x) {
        return bar();
    }
}

function parenthesized()
{
    return bar();
}

function differentCase()
{
    $value = bar();
    if (! $value instanceof \STDCLASS) {
        return null;
    }

    return $value;
}

function byReference()
{
    $value = &$GLOBALS['x'];
    return bar();
}

function elseBranch()
{
    $value = bar();
    if (! $value instanceof \stdClass) {
        return null;
    } else {
        $x = 1;
    }

    return $value;
}

$closure = function () {
    return bar();
};
