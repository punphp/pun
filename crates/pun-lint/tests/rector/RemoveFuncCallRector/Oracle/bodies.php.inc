<?php

function run($x)
{
    $a = 1;

    // dump it
    var_dump($x);
    if ($x) var_dump($x);
    if ($x) {
        var_dump($x);
    }
    foreach ($x as $y) {
        $a++;
        var_dump($y);
    }
    (var_dump($x));
    var_dump(...);
    return $a;
}
-----
<?php

function run($x)
{
    $a = 1;
    if ($x) {
    }
    if ($x) {
    }
    foreach ($x as $y) {
        $a++;
    }
    return $a;
}
