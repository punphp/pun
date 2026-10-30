<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(int $i): void
    {
    }
}

class B
{
    public function m(int $i): void
    {
    }
}

function f(A|B $a): void
{
    $a->m(1);
}
