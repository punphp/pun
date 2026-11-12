<?php

function flipNegatedTernaryInstanceof($o, $a, $b, $x)
{
    $r = ! $o instanceof P ? null : $o->getPrice();
    $r = !$o instanceof P?1:2;
    $r = !$o instanceof P ? $x = 1 : 2;
    $r = !$o instanceof P ? print 1 : 2;
    $r = !$o instanceof P ? fn () => 1 : 2;
    $r = !$o instanceof P ? throw new E : 2;
    $r = !$o instanceof P ? $a ?? 2 : 3;
    $r = !$o instanceof P ? 1 : ($x = 2);
    $r = !$o instanceof P ? 1 : ($a ? 2 : 3);
    $r = !$o instanceof P ? ($a and $b) : ($a ? 2 : 3);
    $r = !$o instanceof P ? /* a */ 1 /* b */ : /* c */ 2 /* d */;
    $r = !($o) instanceof P ? 1 : 2;
    $r = !  ($o instanceof P) ? 1 : 2;
    $r = (!($o instanceof P)) ? 1
        : 2;
    $r = ! ! $o instanceof P ? 1 : 2;
    $r = @ ! $o instanceof P ? 1 : 2;
    $r = ! $o ? 1 : 2;
    $r = -$o instanceof P ? 1 : 2;
    $r = ! ($o instanceof P && $a) ? 1 : 2;
    $r = $o instanceof P ? 1 : 2;
}
-----
<?php

function flipNegatedTernaryInstanceof($o, $a, $b, $x)
{
    $r = $o instanceof P ? $o->getPrice() : null;
    $r = $o instanceof P?2:1;
    $r = $o instanceof P ? 2 : $x = 1;
    $r = $o instanceof P ? 2 : print 1;
    $r = $o instanceof P ? 2 : fn () => 1;
    $r = $o instanceof P ? 2 : throw new E;
    $r = $o instanceof P ? 3 : $a ?? 2;
    $r = $o instanceof P ? $x = 2 : (1);
    $r = $o instanceof P ? $a ? 2 : 3 : (1);
    $r = $o instanceof P ? ($a ? 2 : 3) : ($a and $b);
    $r = $o instanceof P ? /* a */ 2 /* b */ : /* c */ 1 /* d */;
    $r = ($o) instanceof P ? 2 : 1;
    $r = $o instanceof P ? 2 : 1;
    $r = ($o instanceof P) ? 2
        : 1;
    $r = ! ! $o instanceof P ? 1 : 2;
    $r = @ ! $o instanceof P ? 1 : 2;
    $r = ! $o ? 1 : 2;
    $r = -$o instanceof P ? 1 : 2;
    $r = ! ($o instanceof P && $a) ? 1 : 2;
    $r = $o instanceof P ? 1 : 2;
}
