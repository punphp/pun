<?php

function singleInArrayKeepParentheses($a, $b)
{
    $r = in_array($a ?? $b, [$b ?: $a]);
    $r = in_array($a xor $b, [$a or $b]);
    $r = in_array($a, [$b]) || $b;
}

function singleInArrayPrecedence($a, $b)
{
    $r = in_array($a |> trim(...), [$b]);
    $r = in_array(yield $a, [$b]);
    $r = in_array(yield from $a, [$b]);
    $r = in_array(print $a, [$b]);
    $r = in_array(include $a, [$b]);
    $r = in_array(fn () => $a, [$b]);
    $r = in_array(throw $a, [$b]);
    $r = in_array((int) $a, [-$b]);
    $r = in_array(++$a, [!$b]);
}
-----
<?php

function singleInArrayKeepParentheses($a, $b)
{
    $r = ($a ?? $b) == ($b ?: $a);
    $r = ($a xor $b) == ($a or $b);
    $r = $a == $b || $b;
}

function singleInArrayPrecedence($a, $b)
{
    $r = $a |> trim(...) == $b;
    $r = (yield $a) == $b;
    $r = (yield from $a) == $b;
    $r = (print $a) == $b;
    $r = (include $a) == $b;
    $r = (fn () => $a) == $b;
    $r = (throw $a) == $b;
    $r = (int) $a == -$b;
    $r = ++$a == !$b;
}
