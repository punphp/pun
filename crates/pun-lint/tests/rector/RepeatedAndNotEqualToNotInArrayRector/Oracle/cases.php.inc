<?php

function f($a, $b)
{
    $x = $a != 1 && $a !== 2 && $a <> 3;
    $y = $b && $a !== 1 && $a !== 2 && $a !== 3;
    $z = !($a !== 1 && $a !== 2 && $a !== 3);
    $w = $a !== 1 && $a !== 2 && $a !== 3 && $b;
    return [$x, $y, $z, $w];
}
-----
<?php

function f($a, $b)
{
    $x = !in_array($a, [1, 2, 3]);
    $y = $b && $a !== 1 && $a !== 2 && $a !== 3;
    $z = !(!in_array($a, [1, 2, 3], true));
    $w = !in_array($a, [1, 2, 3], true) && $b;
    return [$x, $y, $z, $w];
}
