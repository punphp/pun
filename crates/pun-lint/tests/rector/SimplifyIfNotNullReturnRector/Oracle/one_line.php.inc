<?php

function d($x)
{
    $a = 1; if ($x !== null) return $x; return null;
}
-----
<?php

function d($x)
{
    $a = 1;
    return $x;
}
