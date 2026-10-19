<?php

function shapes($x, $o)
{
    $a = 1;
    $b = compact(var_name: $x);
    $c = compact($o->names);
    $d = compact(['a', ...$x]);
    $e = compact(['k' => 'a']);
}
-----
<?php

function shapes($x, $o)
{
    $a = 1;
    $b = compact(var_name: $x);
    $c = compact($o->names);
    $d = compact(['a', ...$x]);
    $e = ['a' => $a];
}
