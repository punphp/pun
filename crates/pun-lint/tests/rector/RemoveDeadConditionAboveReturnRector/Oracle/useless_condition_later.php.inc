<?php

function f($a)
{
    if (foo()) {
        echo 1;
    }
    return $a;
    bar();
    return 2;
}
