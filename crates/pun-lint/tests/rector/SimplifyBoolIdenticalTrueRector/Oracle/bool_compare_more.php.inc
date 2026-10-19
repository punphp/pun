<?php

function f(bool $a, $b, $c)
{
    $x = $a === $b;
    $y = ($c > 1) !== true;
    $v = is_int($c) !== true;
}
-----
<?php

function f(bool $a, $b, $c)
{
    $x = $a === $b;
    $y = !($c > 1);
    $v = !is_int($c);
}
