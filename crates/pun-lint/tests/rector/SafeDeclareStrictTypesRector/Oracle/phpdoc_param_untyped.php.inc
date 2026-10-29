<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct()
    {
    }

    /** @param int $a */
    public function m($a): void
    {
    }
}

(new A())->m("1");
