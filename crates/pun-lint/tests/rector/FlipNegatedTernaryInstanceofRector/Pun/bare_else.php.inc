<?php

function flipNegatedTernaryInstanceofBareElse($o, $a, $b)
{
    $r = !$o instanceof P ? ($a ? 1 : 2) : 3;
    $r = !$o instanceof P ? ($a and $b) : 3;
    $r = !$o instanceof P ? $a ?: 2 : 3;
    $r = !$o instanceof P ? $a xor $b : 3;
    $r = !$o instanceof P ? $a or $b : (3);
}
-----
<?php

function flipNegatedTernaryInstanceofBareElse($o, $a, $b)
{
    $r = $o instanceof P ? (3) : ($a ? 1 : 2);
    $r = $o instanceof P ? (3) : ($a and $b);
    $r = $o instanceof P ? 3 : ($a ?: 2);
    $r = $o instanceof P ? 3 : ($a xor $b);
    $r = $o instanceof P ? 3 : ($a or $b);
}
