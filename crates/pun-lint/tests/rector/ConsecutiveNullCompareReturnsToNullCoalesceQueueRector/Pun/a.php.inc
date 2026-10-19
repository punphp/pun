<?php

function a($a, $b)
{
    $x = 1;

    // lead
    if ($a !== null) {
        return $a;
    }

    if ($b !== null) {
        return $b;
    }

    // tail
    return null;
}

function b($a, $b)
{
    $x = 1;
    if ($a !== null) {
        return $a;
    }
    $y = 2;
    if ($b !== null) {
        return $b;
    }
    return 5;
}

function c($a, $b)
{
    if (null !== $a) {
        return $a;
    } else {
        echo 1;
    }
    if ($b !== null) {
        return $b;
    }
    return $c ?: $d;
}
-----
<?php

function a($a, $b)
{
    $x = 1;
    return $a ?? $b;
}

function b($a, $b)
{
    $x = 1;
    if ($a !== null) {
        return $a;
    }
    $y = 2;
    if ($b !== null) {
        return $b;
    }
    return 5;
}

function c($a, $b)
{
    if (null !== $a) {
        return $a;
    } else {
        echo 1;
    }
    if ($b !== null) {
        return $b;
    }
    return $c ?: $d;
}
