<?php

function f(float $a, bool $b)
{
    if ($b) {
        return 1.5;
    }
    return $a;
}
