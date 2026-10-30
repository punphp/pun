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
7: Variable property access on Unknown.
8: Variable property access on Unknown.
