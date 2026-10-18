<?php

function singleInArrayOracle($a, $b, $f)
{
    $r = \in_array($a, array('one'));
    $r = in_array($a, ['k' => 'one'], false);
    $r = !\in_array($a, [$b], true);
    $r = !(in_array($a, [$b]));
    $r = in_array($a, $b);
    $r = in_array($a);
    $r = in_array($a, []);
    $r = in_array($a, [...$b]);
    $r = $f($a, [$b]);
    $r = -in_array($a, [$b]);
    $r = 'x' . in_array($a, [$b]);
    $r = (int) in_array($a, [$b]);
    $r = in_array($a, [$b]) && $b;
    $r = in_array($a, [$b]) ? 1 : 2;
    $r = !in_array($a, [$b, $a]);
}
-----
<?php

function singleInArrayOracle($a, $b, $f)
{
    $r = $a == 'one';
    $r = $a === 'one';
    $r = $a !== $b;
    $r = $a != $b;
    $r = in_array($a, $b);
    $r = in_array($a);
    $r = in_array($a, []);
    $r = in_array($a, [...$b]);
    $r = $f($a, [$b]);
    $r = -($a == $b);
    $r = 'x' . ($a == $b);
    $r = (int) ($a == $b);
    $r = $a == $b && $b;
    $r = $a == $b ? 1 : 2;
    $r = !in_array($a, [$b, $a]);
}
