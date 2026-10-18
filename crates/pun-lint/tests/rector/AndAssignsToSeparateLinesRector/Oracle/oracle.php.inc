<?php

function andAssignsOracle($a, $b, $c)
{
    $a = 1 and $b = 2;
    ($a = 1 and $b = 2);
    $a = 1 and $b += 2;
    $a .= 1 and $b = 2;
    $a = 1 and $b = &$c;
    $a = 1 or $b = 2;
    $a = 1 && $b = 2;
    foo();
    switch ($a) {
        case 1:
            $a = 1 and $b = 2;
    }
}
-----
<?php

function andAssignsOracle($a, $b, $c)
{
    $a = 1;
    $b = 2;
    $a = 1;
    $b = 2;
    $a = 1 and $b += 2;
    $a .= 1 and $b = 2;
    $a = 1 and $b = &$c;
    $a = 1 or $b = 2;
    $a = 1 && $b = 2;
    foo();
    switch ($a) {
        case 1:
            $a = 1;
            $b = 2;
    }
}
