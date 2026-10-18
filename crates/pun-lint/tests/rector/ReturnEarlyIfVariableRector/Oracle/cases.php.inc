<?php

function byRefClosure($value)
{
    $f = function () use (&$value) {};
    if ($value === 1) {
        $value = 2;
    }
    return $value;
}

function staticVar()
{
    static $value;
    if ($value === 1) {
        $value = 2;
    }
    return $value;
}

function phpstanVar($value)
{
    if ($value === 1) {
        /** @phpstan-var int $value */
        $value = 2;
    }
    return $value;
}

function plainComment($value)
{
    if ($value === 1) {
        /* not a doc */
        $value = 2;
    }
    return $value;
}

function laterClosure($value)
{
    if ($value === 1) {
        $value = 2;
    }
    return $value;
    $f = function () use (&$value) {};
}
-----
<?php

function byRefClosure($value)
{
    $f = function () use (&$value) {};
    if ($value === 1) {
        $value = 2;
    }
    return $value;
}

function staticVar()
{
    static $value;
    if ($value === 1) {
        $value = 2;
    }
    return $value;
}

function phpstanVar($value)
{
    if ($value === 1) {
        /** @phpstan-var int $value */
        return 2;
    }
    return $value;
}

function plainComment($value)
{
    if ($value === 1) {
        /* not a doc */
        return 2;
    }
    return $value;
}

function laterClosure($value)
{
    if ($value === 1) {
        return 2;
    }
    return $value;
    $f = function () use (&$value) {};
}
