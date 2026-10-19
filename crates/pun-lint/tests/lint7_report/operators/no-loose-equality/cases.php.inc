<?php

$a = 1 == 2;
$b = $a != 3;
$c = $a <> 4;
$d = $a === $b || $a !== $c;
if ($a
    == $b && ($c != $d) == true) {
}
-----
3: Loose comparison via "==" is not allowed.
4: Loose comparison via "!=" is not allowed.
5: Loose comparison via "!=" is not allowed.
7: Loose comparison via "==" is not allowed.
8: Loose comparison via "!=" is not allowed.
8: Loose comparison via "==" is not allowed.
