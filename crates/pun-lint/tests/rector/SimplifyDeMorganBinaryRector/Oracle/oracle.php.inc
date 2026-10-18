<?php

function deMorganOracle($a, $b, $c, $d)
{
    $r = !($a == $b || $c != $d);
    $r = !($a <> $b || $c >= $d);
    $r = !($a ** 2 > $b || $c);
    $r = !($a % 2 || $b << 1);
    $r = !($a . $b || $a & $b);
    $r = !($a ^ $b || $a | $b);
    $r = !($a ?? $b || $c);
    $r = !($a || ($b = $c));
    $r = !($a or $b);
    $r = !($a <=> $b || $c);
    $r = !($a - $b * $c || $c / $d);
    $r = !($a >> 1 || $a xor $b);
    $r = !($a || ($b and $c));
    $r = $a && !($b || $c);
    $r = !($b || $c) && $a;
    $r = (bool) !($b || $c);
    $r = !($b || $c) ? $a : $d;
    $r = $a ?: !($b || $c);
}
-----
<?php

function deMorganOracle($a, $b, $c, $d)
{
    $r = $a != $b && $c == $d;
    $r = $a == $b && $c < $d;
    $r = $a ** 2 <= $b && !$c;
    $r = !($a % 2) && !($b << 1);
    $r = !($a . $b) && !($a & $b);
    $r = !($a ^ $b) && !($a | $b);
    $r = !($a ?? $b || $c);
    $r = !$a && !$b = $c;
    $r = !($a or $b);
    $r = !($a <=> $b) && !$c;
    $r = !($a - $b * $c) && !($c / $d);
    $r = !($a >> 1 || $a xor $b);
    $r = !$a && !($b and $c);
    $r = $a && (!$b && !$c);
    $r = !$b && !$c && $a;
    $r = (bool) (!$b && !$c);
    $r = !$b && !$c ? $a : $d;
    $r = $a ?: !$b && !$c;
}
