<?php

namespace App;

class Base
{
    public function one($a) {}
}

function f(?Base $n, Base|int $bi, Base|false $bf, Base|null|false $bnf)
{
    $n->one(1, 2);
    $bi->one(1, 2);
    $bf->one(1, 2);
    $bnf->one(1, 2);
}

function g(?Base $n)
{
    $n->one(1, 2);
}
