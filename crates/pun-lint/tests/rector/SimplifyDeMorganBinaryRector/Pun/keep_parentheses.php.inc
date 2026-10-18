<?php

function deMorganKeepParentheses($a, $b, $c, $d)
{
    $r = !(($a ? $b : $c) || $d);
    $r = !(($a = $b) || $c);
}
-----
<?php

function deMorganKeepParentheses($a, $b, $c, $d)
{
    $r = !($a ? $b : $c) && !$d;
    $r = !($a = $b) && !$c;
}
