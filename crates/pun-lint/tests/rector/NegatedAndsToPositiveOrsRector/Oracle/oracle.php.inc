<?php

function negatedAndsOracle($a, $b, $c, $d)
{
    $r = !($a == $b && $c != $d);
    $r = !($a <> $b && $c >= $d);
    $r = !($a && !$b);
    $r = !($a + 1 && $b . $c);
    $r = !($a && ($b = $c));
    $r = !($a && ($b && $c));
    $r = !($a & $b);
    $r = -$a;
    $r = !$a;
    $r = $d . !($a && $b);
    $r = !($a && $b) . $d;
    $r = (int) !($a && $b);
    $r = !($a instanceof Foo && $b);
    $r = !($a ? $b : $c && $d);
}
-----
<?php

function negatedAndsOracle($a, $b, $c, $d)
{
    $r = $a != $b || $c == $d;
    $r = $a == $b || $c < $d;
    $r = !$a || $b;
    $r = !($a + 1) || !($b . $c);
    $r = !$a || !$b = $c;
    $r = !($a && ($b && $c));
    $r = !($a & $b);
    $r = -$a;
    $r = !$a;
    $r = $d . (!$a || !$b);
    $r = (!$a || !$b) . $d;
    $r = (int) (!$a || !$b);
    $r = !$a instanceof Foo || !$b;
    $r = !($a ? $b : $c && $d);
}
