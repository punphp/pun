<?php

function a($a, $b, $c)
{
    $x = !($a === $b) && $c;
    $x = !($a == $b) . 'x';
    $x = !($a < ($b ?: $c));
    $x = !($a <> $b);
    $x = !(($a) >= ($b));
    $x = !($a instanceof B);
    $x = !($a + $b);
    $x = !($a || $b);
    $x = !($a && $b);
    $x = !($a === $b + 1);
    $x = !($a ?? $b);
    $x = !$a;
    $x = (($a === $b)) === FALSE;
    $x = TRUE === ($a !== $b);
    $x = ($a === $b) !== false;
    $x = ($a == $b) === false;
    $x = ($a === $b) === null;
    $x = ($a === $b) === false && $c;
    $x = $c . ($a === $b) === true;
    $x = !($a = $b);
}
-----
<?php

function a($a, $b, $c)
{
    $x = $a !== $b && $c;
    $x = ($a != $b) . 'x';
    $x = $a >= ($b ?: $c);
    $x = $a == $b;
    $x = $a < $b;
    $x = !($a instanceof B);
    $x = !($a + $b);
    $x = !($a || $b);
    $x = !($a && $b);
    $x = !($a === $b + 1);
    $x = !($a ?? $b);
    $x = !$a;
    $x = $a !== $b;
    $x = $a !== $b;
    $x = ($a === $b) !== false;
    $x = ($a == $b) === false;
    $x = ($a === $b) === null;
    $x = $a !== $b && $c;
    $x = $c . ($a === $b) === true;
    $x = !($a = $b);
}
