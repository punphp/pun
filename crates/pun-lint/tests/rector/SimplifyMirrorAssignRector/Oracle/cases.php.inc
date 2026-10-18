<?php

function f($a, $b)
{
    if ($a) $b = $b;
    $a = ($a);
    // a comment before
    $b = $b;
    foreach ([] as $x) {
        $x = $x;
    }
    $this->x = $this->x;
    $a = $A;
    return $a;
}

function last($a)
{
    echo 1;
    $a = $a;
}
-----
<?php

function f($a, $b)
{
    if ($a) {
    }
    foreach ([] as $x) {
    }
    $a = $A;
    return $a;
}

function last($a)
{
    echo 1;
}
