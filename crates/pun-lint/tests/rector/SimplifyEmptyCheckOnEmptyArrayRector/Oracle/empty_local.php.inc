<?php

function f()
{
    $a = [];
    if (rand()) {
        $a = [1];
    }
    return empty($a);
}

function g()
{
    $b = foo();
    return empty($b);
}
-----
<?php

function f()
{
    $a = [];
    if (rand()) {
        $a = [1];
    }
    return $a === [];
}

function g()
{
    $b = foo();
    return empty($b);
}
