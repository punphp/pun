<?php

function a($v)
{
    if ($v === 'x') {
        return 'x';
    }

    // the value
    return $v;
}

function b($v)
{
    if ($v == 1) return 1;
    return $v;
}

function c($v)
{
    if ($v === null) {
        return null;
    } else {
        echo 1;
    }
    return $v;
}

function d($v)
{
    if ($v === bar()) {
        return bar();
    }
    return $v;
}

function e($v)
{
    if (($v === 2)) {
        return (2);
    }
    return ($v);
}
-----
<?php

function a($v)
{
    // the value
    return $v;
}

function b($v)
{
    return $v;
}

function c($v)
{
    return $v;
}

function d($v)
{
    if ($v === bar()) {
        return bar();
    }
    return $v;
}

function e($v)
{
    return ($v);
}
