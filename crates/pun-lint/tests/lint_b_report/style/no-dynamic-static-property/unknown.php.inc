<?php

namespace N;

function f($m, \Unknown $u, ?\Unknown $n)
{
    $u->$m;
    $n?->$m;
    \Unknown::$$m;
    $u::$$m;
}
-----
9: Variable static property access on Unknown.
10: Variable static property access on Unknown.
