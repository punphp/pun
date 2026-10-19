<?php

function more($a, $b)
{
    array_values($a)[0] = 1;
    $x = array_values($a)[count() - 1];
    $y = array_values($a)[count($a, $b) - 1];
}
