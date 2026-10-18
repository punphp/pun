<?php

function run(array $a)
{
    $x = $a ? '' : implode(',', $a);
    $y = $a === [] ? null : implode(',', $a);
}
