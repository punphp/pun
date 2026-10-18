<?php

function f($a, $b)
{
    $x = ($a == 1) || ($a == 2) || $a == 3;
    $y = $a === 1 || $a === 2 || $a === 3 || $b === 4;
    $z = $b || $a === 1 || $a === 2 || $a === 3;
    $w = $a === 1 || $a === 2 || $a === ($b == 3);
    $v = $a == 1 or $a == 2 or $a == 3;
    return [$x, $y, $z, $w, $v];
}
-----
<?php

function f($a, $b)
{
    $x = in_array($a, [1, 2, 3]);
    $y = in_array($a, [1, 2, 3], true) || $b === 4;
    $z = $b || $a === 1 || $a === 2 || $a === 3;
    $w = $a === 1 || $a === 2 || $a === ($b == 3);
    $v = $a == 1 or $a == 2 or $a == 3;
    return [$x, $y, $z, $w, $v];
}
