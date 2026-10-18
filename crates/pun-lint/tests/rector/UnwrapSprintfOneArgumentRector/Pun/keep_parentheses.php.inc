<?php

function sprintfKeepParentheses($a, $b)
{
    $r = sprintf($a ? $b : 'c') . 'x';
    $r = 'x' . sprintf($a ? $b : 'c');
    $r = -sprintf($a + $b);
    $r = sprintf($a ?: $b) ? 1 : 2;
    $r = sprintf($a and $b);
    $r = sprintf($a . $b)[0];
    $r = $a - sprintf($a - $b);
    $r = $a ? sprintf($b ?: $a) : 1;
}
-----
<?php

function sprintfKeepParentheses($a, $b)
{
    $r = ($a ? $b : 'c') . 'x';
    $r = 'x' . ($a ? $b : 'c');
    $r = -($a + $b);
    $r = ($a ?: $b) ? 1 : 2;
    $r = ($a and $b);
    $r = ($a . $b)[0];
    $r = $a - ($a - $b);
    $r = $a ? $b ?: $a : 1;
}
