<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct()
    {
    }

    /** @param positive-int $a */
    public function m(int $a): void
    {
    }
}

(new A())->m(-1);
