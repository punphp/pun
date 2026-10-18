<?php

function funcGetArgsCountOracle($f)
{
    $r = \count(\func_get_args());
    $r = COUNT(func_get_args(), COUNT_RECURSIVE);
    $r = count();
    $r = count($f);
    $r = count(func_num_args());
    $r = $f(func_get_args());
    $r = sizeof(func_get_args());
}
-----
<?php

function funcGetArgsCountOracle($f)
{
    $r = func_num_args();
    $r = func_num_args();
    $r = count();
    $r = count($f);
    $r = count(func_num_args());
    $r = $f(func_get_args());
    $r = sizeof(func_get_args());
}
