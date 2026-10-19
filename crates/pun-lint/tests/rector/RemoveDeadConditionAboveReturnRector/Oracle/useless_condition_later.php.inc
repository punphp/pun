<?php

function f($a)
{
    if ($a) {
        echo 1;
    }
    return $a;
    foo();
    return 2;
}
