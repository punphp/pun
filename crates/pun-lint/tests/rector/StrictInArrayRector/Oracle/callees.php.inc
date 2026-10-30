<?php

function f(int $a, callable $f)
{
    $b = [1, 2];
    $f($a, $b);
    strlen('x');
    in_array($a, $b);
}
-----
<?php

function f(int $a, callable $f)
{
    $b = [1, 2];
    $f($a, $b);
    strlen('x');
    in_array($a, $b, true);
}
