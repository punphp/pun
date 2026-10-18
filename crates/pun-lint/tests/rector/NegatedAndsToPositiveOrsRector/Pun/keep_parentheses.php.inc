<?php

function negatedAndsKeepParentheses($a, $b, $c, $d)
{
    $r = !(($a = $b) && $c);
    $r = !(($a ? $b : $c) && $d);
}
-----
<?php

function negatedAndsKeepParentheses($a, $b, $c, $d)
{
    $r = !($a = $b) || !$c;
    $r = !($a ? $b : $c) || !$d;
}
