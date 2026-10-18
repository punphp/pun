<?php

function multipleBooleanNotOracle($a, $b)
{
    $r = !!($a && $b);
    $r = !(!$a);
    $r = !!($a = $b);
    $r = -(-$a);
}
-----
<?php

function multipleBooleanNotOracle($a, $b)
{
    $r = (bool) ($a && $b);
    $r = (bool) $a;
    $r = (bool) $a = $b;
    $r = -(-$a);
}
