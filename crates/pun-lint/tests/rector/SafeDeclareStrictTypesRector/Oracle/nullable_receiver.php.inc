<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(int $i): void
    {
    }
}

function f(?A $a): void
{
    $a->m(1);
}
