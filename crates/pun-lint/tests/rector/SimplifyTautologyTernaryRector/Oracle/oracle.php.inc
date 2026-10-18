<?php

function tautologyTernaryOracle($a, $b, $c)
{
    $r = $a !== $b ? $a : $b;
    $r = $a !== $b ? $b : $a;
    $r = $a === $b ? $a : $c;
    $r = $a == $b ? $a : $b;
    $r = $a ?: $b;
    $r = $a ? $a : $b;
    $r = ($a + 1 !== $b ? $a + 1 : $b) * 2;
    $r = 2 * ($a !== $b + 1 ? $a : $b + 1);
    $r = ($a !== ($b = 1) ? $a : ($b = 1)) . 'x';
}
-----
<?php

function tautologyTernaryOracle($a, $b, $c)
{
    $r = $a;
    $r = $b;
    $r = $a === $b ? $a : $c;
    $r = $a == $b ? $a : $b;
    $r = $a ?: $b;
    $r = $a ? $a : $b;
    $r = ($a + 1) * 2;
    $r = 2 * ($a);
    $r = ($a) . 'x';
}
